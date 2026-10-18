//! Server configuration.

use crate::error::{Error, Result};
use crate::registry::constant_time_eq;
use cottak_proto::frame::DEFAULT_MAX_FRAME_LEN;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::WebPkiClientVerifier;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// TLS settings for one listener.
///
/// Construct with [`ServerTlsConfig::mutual`] (what TAK Server's port 8089
/// does) or, explicitly, [`ServerTlsConfig::server_only`].
#[derive(Clone)]
pub struct ServerTlsConfig {
    cert_chain: Vec<u8>,
    key: Vec<u8>,
    client_ca: Option<Vec<u8>>,
}

impl std::fmt::Debug for ServerTlsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never print key material.
        f.debug_struct("ServerTlsConfig")
            .field("cert_chain_bytes", &self.cert_chain.len())
            .field("mutual", &self.client_ca.is_some())
            .finish_non_exhaustive()
    }
}

impl ServerTlsConfig {
    /// Presents `cert_chain_pem` and requires every client to present a
    /// certificate issued by one of the CAs in `client_ca_pem`.
    pub fn mutual(
        cert_chain_pem: impl Into<Vec<u8>>,
        key_pem: impl Into<Vec<u8>>,
        client_ca_pem: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            cert_chain: cert_chain_pem.into(),
            key: key_pem.into(),
            client_ca: Some(client_ca_pem.into()),
        }
    }

    /// Presents `cert_chain_pem` but does **not** authenticate clients.
    ///
    /// Anyone who can reach the port can connect. Use only on trusted
    /// networks or in tests.
    pub fn server_only(cert_chain_pem: impl Into<Vec<u8>>, key_pem: impl Into<Vec<u8>>) -> Self {
        Self {
            cert_chain: cert_chain_pem.into(),
            key: key_pem.into(),
            client_ca: None,
        }
    }

    /// Like [`mutual`](Self::mutual), reading the PEM data from files.
    pub fn mutual_from_files(
        cert_chain: impl AsRef<Path>,
        key: impl AsRef<Path>,
        client_ca: impl AsRef<Path>,
    ) -> Result<Self> {
        Ok(Self::mutual(
            std::fs::read(cert_chain)?,
            std::fs::read(key)?,
            std::fs::read(client_ca)?,
        ))
    }

    /// Builds the rustls configuration, validating all PEM data.
    pub fn server_config(&self) -> Result<Arc<rustls::ServerConfig>> {
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let chain = CertificateDer::pem_slice_iter(&self.cert_chain)
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::TlsConfig(format!("bad certificate PEM: {e}")))?;
        if chain.is_empty() {
            return Err(Error::TlsConfig("certificate PEM is empty".into()));
        }
        let key = PrivateKeyDer::from_pem_slice(&self.key)
            .map_err(|e| Error::TlsConfig(format!("bad key PEM: {e}")))?;

        let builder = rustls::ServerConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()
            .map_err(|e| Error::TlsConfig(e.to_string()))?;
        let builder = match &self.client_ca {
            Some(ca_pem) => {
                let mut roots = rustls::RootCertStore::empty();
                for cert in CertificateDer::pem_slice_iter(ca_pem) {
                    let cert =
                        cert.map_err(|e| Error::TlsConfig(format!("bad client CA PEM: {e}")))?;
                    roots
                        .add(cert)
                        .map_err(|e| Error::TlsConfig(format!("bad client CA: {e}")))?;
                }
                if roots.is_empty() {
                    return Err(Error::TlsConfig(
                        "client CA PEM contains no certificate; client \
                         authentication cannot be silently disabled"
                            .into(),
                    ));
                }
                let verifier =
                    WebPkiClientVerifier::builder_with_provider(Arc::new(roots), provider)
                        .build()
                        .map_err(|e| Error::TlsConfig(format!("client verifier: {e}")))?;
                builder.with_client_cert_verifier(verifier)
            }
            None => builder.with_no_client_auth(),
        };
        let config = builder
            .with_single_cert(chain, key)
            .map_err(|e| Error::TlsConfig(format!("certificate and key do not match: {e}")))?;
        Ok(Arc::new(config))
    }
}

/// HTTP Basic credentials required by the Marti API.
#[derive(Clone)]
pub struct BasicAuth {
    user: String,
    password: String,
}

impl BasicAuth {
    /// Requires exactly this user name and password.
    pub fn new(user: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            user: user.into(),
            password: password.into(),
        }
    }

    pub(crate) fn matches(&self, user: &str, password: &str) -> bool {
        // Evaluate both so timing does not reveal which one was wrong.
        let u = constant_time_eq(self.user.as_bytes(), user.as_bytes());
        let p = constant_time_eq(self.password.as_bytes(), password.as_bytes());
        u & p
    }
}

impl std::fmt::Debug for BasicAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicAuth")
            .field("user", &self.user)
            .finish_non_exhaustive()
    }
}

/// What a listener serves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ListenerKind {
    /// The streaming CoT protocol.
    Tak,
    /// The HTTP API.
    Marti,
}

#[derive(Debug, Clone)]
pub(crate) struct ListenerConfig {
    pub(crate) addr: String,
    pub(crate) tls: Option<ServerTlsConfig>,
    pub(crate) kind: ListenerKind,
}

/// Configuration of a [`Server`](crate::Server).
///
/// At least one listener must be added with [`listen_tcp`](Self::listen_tcp)
/// or [`listen_tls`](Self::listen_tls). There are no default listeners.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ServerConfig {
    pub(crate) listeners: Vec<ListenerConfig>,
    /// UID the server uses in the control events it sends.
    pub server_uid: String,
    /// TAK Protocol versions offered to clients; empty means XML only.
    pub protocol_versions: Vec<u32>,
    /// Most simultaneous connections (including those still in TLS handshake).
    pub max_clients: usize,
    /// Largest accepted message, in bytes.
    pub max_message_len: usize,
    /// Events that may wait for one slow client before further events for it
    /// are dropped.
    pub client_queue: usize,
    /// A client that sends nothing for this long is disconnected. Clients
    /// normally ping every 30 s.
    pub read_timeout: Duration,
    /// A write that does not finish in this time disconnects the client.
    pub write_timeout: Duration,
    /// Time allowed for the TLS handshake.
    pub handshake_timeout: Duration,
    /// Send the latest known position of every entity to new clients.
    pub replay_on_connect: bool,
    /// Most entities kept for replay.
    pub max_stored_events: usize,
    /// Size of the queue feeding [`Server::subscribe`](crate::Server::subscribe).
    pub observer_queue: usize,
    /// Credentials the Marti API requires; `None` leaves it open (rely on mutual TLS).
    pub marti_basic_auth: Option<BasicAuth>,
    /// Largest Marti request body (and so the largest upload), in bytes.
    pub marti_max_body: usize,
    /// Time allowed to receive one Marti request; also the keep-alive idle limit.
    pub marti_request_timeout: Duration,
    /// Most missions held in memory.
    pub max_missions: usize,
    /// Most uploaded files held in memory.
    pub max_files: usize,
    /// Most uploaded bytes held in memory.
    pub max_store_bytes: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            listeners: Vec::new(),
            server_uid: "cottak-server".to_owned(),
            protocol_versions: vec![1],
            max_clients: 1024,
            max_message_len: DEFAULT_MAX_FRAME_LEN,
            client_queue: 1024,
            read_timeout: Duration::from_secs(90),
            write_timeout: Duration::from_secs(10),
            handshake_timeout: Duration::from_secs(10),
            replay_on_connect: true,
            max_stored_events: 10_000,
            observer_queue: 1024,
            marti_basic_auth: None,
            marti_max_body: 16 * 1024 * 1024,
            marti_request_timeout: Duration::from_secs(30),
            max_missions: 1000,
            max_files: 1000,
            max_store_bytes: 256 * 1024 * 1024,
        }
    }
}

impl ServerConfig {
    /// A configuration with defaults and no listeners.
    pub fn new() -> Self {
        Self::default()
    }

    /// Listens for plain TCP (TAK Server's port 8087) on `addr`, for example
    /// `127.0.0.1:8087`. Use port 0 to let the OS choose; see
    /// [`Server::local_addrs`](crate::Server::local_addrs).
    #[must_use]
    pub fn listen_tcp(mut self, addr: impl Into<String>) -> Self {
        self.listeners.push(ListenerConfig {
            addr: addr.into(),
            tls: None,
            kind: ListenerKind::Tak,
        });
        self
    }

    /// Listens for TLS (TAK Server's port 8089) on `addr`.
    #[must_use]
    pub fn listen_tls(mut self, addr: impl Into<String>, tls: ServerTlsConfig) -> Self {
        self.listeners.push(ListenerConfig {
            addr: addr.into(),
            tls: Some(tls),
            kind: ListenerKind::Tak,
        });
        self
    }

    /// Serves the Marti HTTP API (TAK Server's port 8443) on `addr`.
    ///
    /// Pass a [`ServerTlsConfig`] for HTTPS, which is what TAK clients and
    /// `cottak-marti` use; `None` serves plain HTTP.
    #[must_use]
    pub fn listen_marti(mut self, addr: impl Into<String>, tls: Option<ServerTlsConfig>) -> Self {
        self.listeners.push(ListenerConfig {
            addr: addr.into(),
            tls,
            kind: ListenerKind::Marti,
        });
        self
    }

    /// Requires HTTP Basic credentials on the Marti API.
    #[must_use]
    pub fn with_marti_basic_auth(mut self, auth: BasicAuth) -> Self {
        self.marti_basic_auth = Some(auth);
        self
    }

    /// Sets the protocol versions to offer; an empty list disables the
    /// upgrade and keeps every connection on XML.
    #[must_use]
    pub fn with_protocol_versions(mut self, versions: &[u32]) -> Self {
        self.protocol_versions = versions.to_vec();
        self
    }

    /// Sets the idle timeout after which a silent client is disconnected.
    #[must_use]
    pub fn with_read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = timeout;
        self
    }

    pub(crate) fn validate(&self) -> Result<()> {
        if self.listeners.is_empty() {
            return Err(Error::InvalidConfig(
                "no listener configured; call listen_tcp or listen_tls".into(),
            ));
        }
        if self.max_clients == 0 || self.max_missions == 0 || self.max_files == 0 {
            return Err(Error::InvalidConfig(
                "max_clients, max_missions and max_files must be at least 1".into(),
            ));
        }
        if self.protocol_versions.iter().any(|&v| v != 1) {
            return Err(Error::InvalidConfig(
                "only TAK Protocol version 1 is supported".into(),
            ));
        }
        Ok(())
    }
}
