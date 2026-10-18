//! TLS configuration (rustls, no OpenSSL).
//!
//! TAK Servers authenticate clients with X.509 certificates (mutual TLS), and
//! usually present a certificate issued by a private CA, or a self-signed one.
//! This module therefore requires an explicit way to authenticate the server:
//!
//! * a trusted CA ([`TlsConfig::with_ca_pem`]), and/or
//! * a pinned server certificate fingerprint
//!   ([`TlsConfig::with_pinned_sha256`]), which suits self-signed servers.
//!
//! There is no switch that disables verification. To learn the fingerprint of
//! a server you have not met before, use [`probe_server_certificate`] and
//! confirm the value with the server's operator out of band.

use crate::error::{Error, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::CryptoProvider;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime};
use rustls::{CertificateError, DigitallySignedStruct, SignatureScheme};
use sha2::{Digest, Sha256};
use std::fmt;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

/// How to establish TLS to a TAK Server.
#[derive(Clone, Default)]
pub struct TlsConfig {
    ca_pem: Vec<u8>,
    pinned_sha256: Option<[u8; 32]>,
    client_cert_pem: Option<Vec<u8>>,
    client_key_pem: Option<Vec<u8>>,
    server_name: Option<String>,
}

impl fmt::Debug for TlsConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TlsConfig")
            .field("ca_pem_bytes", &self.ca_pem.len())
            .field("pinned_sha256", &self.pinned_sha256.map(hex))
            .field("client_cert", &self.client_cert_pem.is_some())
            .field("client_key", &"<redacted>")
            .field("server_name", &self.server_name)
            .finish_non_exhaustive()
    }
}

impl TlsConfig {
    /// An empty configuration; add a CA and/or a pin before use.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds trusted CA certificate(s) in PEM form. May be called repeatedly.
    #[must_use]
    pub fn with_ca_pem(mut self, pem: impl AsRef<[u8]>) -> Self {
        self.ca_pem.extend_from_slice(pem.as_ref());
        self.ca_pem.push(b'\n');
        self
    }

    /// Accepts only the server certificate whose SHA-256 fingerprint is `fingerprint`.
    ///
    /// Without a CA configured, the chain and host name are not checked: the
    /// pin alone identifies the server (the handshake signature is still
    /// verified, so the server must hold the matching private key). With a CA
    /// configured, the chain and host name are checked as well.
    #[must_use]
    pub fn with_pinned_sha256(mut self, fingerprint: [u8; 32]) -> Self {
        self.pinned_sha256 = Some(fingerprint);
        self
    }

    /// Like [`with_pinned_sha256`](Self::with_pinned_sha256), parsing hex such
    /// as `AB:CD:...` or `abcd...` (64 hex digits; `:` and spaces ignored).
    pub fn with_pinned_sha256_hex(self, fingerprint: &str) -> Result<Self> {
        Ok(self.with_pinned_sha256(parse_fingerprint(fingerprint)?))
    }

    /// Sets the client identity (certificate chain and private key, PEM) for mutual TLS.
    #[must_use]
    pub fn with_client_identity_pem(
        mut self,
        cert_pem: impl Into<Vec<u8>>,
        key_pem: impl Into<Vec<u8>>,
    ) -> Self {
        self.client_cert_pem = Some(cert_pem.into());
        self.client_key_pem = Some(key_pem.into());
        self
    }

    /// Overrides the name checked against the server certificate.
    ///
    /// By default the host part of the connection address is used. TAK
    /// Servers frequently present certificates whose name differs from the
    /// DNS name clients use; set the name the certificate actually carries.
    #[must_use]
    pub fn with_server_name(mut self, name: impl Into<String>) -> Self {
        self.server_name = Some(name.into());
        self
    }

    /// Loads a CA file and, optionally, a client certificate and key file.
    pub fn from_files(ca: impl AsRef<Path>, client: Option<(&Path, &Path)>) -> Result<Self> {
        let mut cfg = Self::new().with_ca_pem(std::fs::read(ca)?);
        if let Some((cert, key)) = client {
            cfg = cfg.with_client_identity_pem(std::fs::read(cert)?, std::fs::read(key)?);
        }
        Ok(cfg)
    }

    /// The server-name override, if one was set.
    pub fn server_name(&self) -> Option<&str> {
        self.server_name.as_deref()
    }

    pub(crate) fn server_name_override(&self) -> Option<&str> {
        self.server_name.as_deref()
    }

    /// Checks that the configuration is usable without connecting.
    pub fn validate(&self) -> Result<()> {
        self.client_config().map(|_| ())
    }

    /// Builds the rustls client configuration, for use with other TLS-based
    /// clients (for example an HTTPS client talking to the same server).
    pub fn client_config(&self) -> Result<Arc<rustls::ClientConfig>> {
        let provider = provider();
        let mut roots = rustls::RootCertStore::empty();
        let mut any_ca = false;
        for cert in CertificateDer::pem_slice_iter(&self.ca_pem) {
            let cert = cert.map_err(|e| Error::TlsConfig(format!("bad CA PEM: {e}")))?;
            roots
                .add(cert)
                .map_err(|e| Error::TlsConfig(format!("bad CA certificate: {e}")))?;
            any_ca = true;
        }
        if !any_ca && self.pinned_sha256.is_none() {
            return Err(Error::TlsConfig(
                "neither a CA certificate nor a pinned fingerprint is configured; \
                 server verification cannot be skipped"
                    .into(),
            ));
        }

        let builder = rustls::ClientConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()?;
        let builder = match self.pinned_sha256 {
            Some(pin) => {
                let chain: Option<Arc<dyn ServerCertVerifier>> = if any_ca {
                    Some(
                        rustls::client::WebPkiServerVerifier::builder_with_provider(
                            Arc::new(roots),
                            provider.clone(),
                        )
                        .build()
                        .map_err(|e| Error::TlsConfig(format!("CA verifier: {e}")))?,
                    )
                } else {
                    None
                };
                builder
                    .dangerous()
                    .with_custom_certificate_verifier(Arc::new(PinnedVerifier {
                        pin,
                        chain,
                        provider,
                    }))
            }
            None => builder.with_root_certificates(roots),
        };

        let config = match (&self.client_cert_pem, &self.client_key_pem) {
            (Some(cert), Some(key)) => {
                let chain = CertificateDer::pem_slice_iter(cert)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| Error::TlsConfig(format!("bad client certificate PEM: {e}")))?;
                if chain.is_empty() {
                    return Err(Error::TlsConfig("client certificate PEM is empty".into()));
                }
                let key = PrivateKeyDer::from_pem_slice(key)
                    .map_err(|e| Error::TlsConfig(format!("bad client key PEM: {e}")))?;
                builder.with_client_auth_cert(chain, key)?
            }
            _ => builder.with_no_client_auth(),
        };
        Ok(Arc::new(config))
    }

    /// Builds a connector, validating the configuration.
    pub(crate) fn connector(&self) -> Result<TlsConnector> {
        Ok(TlsConnector::from(self.client_config()?))
    }
}

fn provider() -> Arc<CryptoProvider> {
    Arc::new(rustls::crypto::ring::default_provider())
}

fn hex(bytes: [u8; 32]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(":")
}

fn parse_fingerprint(s: &str) -> Result<[u8; 32]> {
    let digits: Vec<u8> = s
        .bytes()
        .filter(|b| !matches!(b, b':' | b' ' | b'-'))
        .map(|b| match b {
            b'0'..=b'9' => Ok(b - b'0'),
            b'a'..=b'f' => Ok(b - b'a' + 10),
            b'A'..=b'F' => Ok(b - b'A' + 10),
            _ => Err(Error::TlsConfig(
                "fingerprint contains a non-hex character".into(),
            )),
        })
        .collect::<Result<_>>()?;
    if digits.len() != 64 {
        return Err(Error::TlsConfig(format!(
            "a SHA-256 fingerprint has 64 hex digits, got {}",
            digits.len()
        )));
    }
    let mut out = [0u8; 32];
    for (o, pair) in out.iter_mut().zip(digits.chunks(2)) {
        *o = (pair[0] << 4) | pair[1];
    }
    Ok(out)
}

fn sha256(der: &[u8]) -> [u8; 32] {
    Sha256::digest(der).into()
}

/// Accepts exactly one server certificate; optionally also requires a valid chain.
#[derive(Debug)]
struct PinnedVerifier {
    pin: [u8; 32],
    chain: Option<Arc<dyn ServerCertVerifier>>,
    provider: Arc<CryptoProvider>,
}

impl ServerCertVerifier for PinnedVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> std::result::Result<ServerCertVerified, rustls::Error> {
        if sha256(end_entity) != self.pin {
            return Err(rustls::Error::InvalidCertificate(
                CertificateError::ApplicationVerificationFailure,
            ));
        }
        if let Some(chain) = &self.chain {
            chain.verify_server_cert(end_entity, intermediates, server_name, ocsp_response, now)?;
        }
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

/// The certificate a server presented, as observed by [`probe_server_certificate`].
///
/// It has **not** been authenticated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerCertInfo {
    /// DER encoding of the end-entity certificate.
    pub der: Vec<u8>,
}

impl ServerCertInfo {
    /// SHA-256 fingerprint of the certificate.
    pub fn sha256(&self) -> [u8; 32] {
        sha256(&self.der)
    }

    /// The fingerprint as colon-separated upper-case hex, the form shown by
    /// browsers and `openssl x509 -fingerprint -sha256`.
    pub fn fingerprint(&self) -> String {
        hex(self.sha256())
    }

    /// The certificate in PEM form.
    pub fn to_pem(&self) -> String {
        let b64 = STANDARD.encode(&self.der);
        let mut out = String::from("-----BEGIN CERTIFICATE-----\n");
        for line in b64.as_bytes().chunks(64) {
            out.push_str(std::str::from_utf8(line).unwrap_or_default());
            out.push('\n');
        }
        out.push_str("-----END CERTIFICATE-----\n");
        out
    }
}

/// Records the presented certificate and accepts it.
#[derive(Debug)]
struct RecordingVerifier {
    seen: Mutex<Option<Vec<u8>>>,
    provider: Arc<CryptoProvider>,
}

impl ServerCertVerifier for RecordingVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> std::result::Result<ServerCertVerified, rustls::Error> {
        if let Ok(mut seen) = self.seen.lock() {
            *seen = Some(end_entity.to_vec());
        }
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

/// Connects to `addr` (`host:port`), completes a TLS handshake and returns the
/// certificate the server presented. **No application data is sent** and the
/// certificate is **not authenticated**.
///
/// This supports the "trust on first use" workflow: show
/// [`ServerCertInfo::fingerprint`] to the user, have them compare it with the
/// value published by the server's operator, and only then pass it to
/// [`TlsConfig::with_pinned_sha256`]. Never send credentials based on an
/// unconfirmed fingerprint.
pub async fn probe_server_certificate(addr: &str) -> Result<ServerCertInfo> {
    let host = host_of(addr)?;
    let name =
        ServerName::try_from(host.to_owned()).map_err(|_| Error::InvalidAddress(addr.into()))?;
    let provider = provider();
    let verifier = Arc::new(RecordingVerifier {
        seen: Mutex::new(None),
        provider: provider.clone(),
    });
    let config = rustls::ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .dangerous()
        .with_custom_certificate_verifier(verifier.clone())
        .with_no_client_auth();

    let tcp = TcpStream::connect(addr).await?;
    let tls = TlsConnector::from(Arc::new(config))
        .connect(name, tcp)
        .await?;
    drop(tls);
    let der = verifier
        .seen
        .lock()
        .ok()
        .and_then(|mut s| s.take())
        .ok_or_else(|| Error::TlsConfig("server presented no certificate".into()))?;
    Ok(ServerCertInfo { der })
}

/// Splits `host:port` (or `[v6]:port`) into the host part.
pub(crate) fn host_of(addr: &str) -> Result<&str> {
    let (host, port) = addr
        .rsplit_once(':')
        .ok_or_else(|| Error::InvalidAddress(addr.into()))?;
    if host.is_empty() || port.parse::<u16>().is_err() {
        return Err(Error::InvalidAddress(addr.into()));
    }
    Ok(host.trim_start_matches('[').trim_end_matches(']'))
}

pub(crate) fn server_name(addr: &str, tls: &TlsConfig) -> Result<ServerName<'static>> {
    let name = match tls.server_name_override() {
        Some(n) => n,
        None => host_of(addr)?,
    };
    ServerName::try_from(name.to_owned()).map_err(|_| Error::InvalidAddress(addr.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_parsing() {
        assert_eq!(host_of("tak.example.com:8089").unwrap(), "tak.example.com");
        assert_eq!(host_of("[::1]:8089").unwrap(), "::1");
        assert_eq!(host_of("10.0.0.1:8087").unwrap(), "10.0.0.1");
        for bad in ["nohost", ":8089", "host:port", "host:99999"] {
            assert!(host_of(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn refuses_to_run_without_a_way_to_authenticate_the_server() {
        assert!(matches!(
            TlsConfig::new().validate(),
            Err(Error::TlsConfig(_))
        ));
        assert!(matches!(
            TlsConfig::new().with_ca_pem("not pem at all").validate(),
            Err(Error::TlsConfig(_))
        ));
        assert!(TlsConfig::new()
            .with_pinned_sha256([7; 32])
            .validate()
            .is_ok());
    }

    #[test]
    fn debug_does_not_leak_the_key() {
        let cfg = TlsConfig::new().with_client_identity_pem("cert", "SECRET-KEY-MATERIAL");
        assert!(!format!("{cfg:?}").contains("SECRET"));
    }

    #[test]
    fn fingerprint_parsing() {
        let want = [0xAB; 32];
        let colon = vec!["AB"; 32].join(":");
        assert_eq!(parse_fingerprint(&colon).unwrap(), want);
        assert_eq!(parse_fingerprint(&"ab".repeat(32)).unwrap(), want);
        assert_eq!(hex(want), colon);
        for bad in ["", "AB", &"zz".repeat(32), &"ab".repeat(33)] {
            assert!(parse_fingerprint(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn pem_round_trips_through_rustls() {
        let info = ServerCertInfo {
            der: vec![0x30, 0x03, 0x02, 0x01, 0x05],
        };
        let pem = info.to_pem();
        let back = CertificateDer::pem_slice_iter(pem.as_bytes())
            .next()
            .unwrap()
            .unwrap();
        assert_eq!(back.as_ref(), info.der.as_slice());
    }
}
