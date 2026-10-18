//! Certificate pinning and the trust-on-first-use probe.

use cottak_net::{
    probe_server_certificate, ClientConfig, ClientEvent, ProtocolPreference, TakClient, TlsConfig,
};
use rcgen::{CertificateParams, KeyPair};
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_rustls::TlsAcceptor;

const WAIT: Duration = Duration::from_secs(10);

/// A self-signed server certificate: no CA anywhere, as on a default TAK Server.
struct SelfSigned {
    der: Vec<u8>,
    key_pem: String,
}

fn self_signed(name: &str) -> SelfSigned {
    let key = KeyPair::generate().unwrap();
    let params = CertificateParams::new(vec![name.to_owned()]).unwrap();
    let cert = params.self_signed(&key).unwrap();
    SelfSigned {
        der: cert.der().to_vec(),
        key_pem: key.serialize_pem(),
    }
}

async fn server(cert: &SelfSigned) -> (String, tokio::task::JoinHandle<()>) {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let config = rustls::ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(cert.der.clone())],
            PrivateKeyDer::from_pem_slice(cert.key_pem.as_bytes()).unwrap(),
        )
        .unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let task = tokio::spawn(async move {
        loop {
            let Ok((tcp, _)) = listener.accept().await else {
                return;
            };
            let acceptor = acceptor.clone();
            tokio::spawn(async move {
                // Hold the connection open until the client goes away.
                if let Ok(mut tls) = acceptor.accept(tcp).await {
                    let mut buf = [0u8; 256];
                    while let Ok(n) = tokio::io::AsyncReadExt::read(&mut tls, &mut buf).await {
                        if n == 0 {
                            break;
                        }
                    }
                }
            });
        }
    });
    (addr, task)
}

fn config(addr: &str, tls: TlsConfig) -> ClientConfig {
    let mut c = ClientConfig::new(addr, "CLIENT")
        .with_tls(tls)
        .with_protocol(ProtocolPreference::XmlOnly);
    c.backoff = cottak_net::Backoff {
        min: Duration::from_millis(50),
        max: Duration::from_millis(100),
    };
    c
}

#[tokio::test]
async fn probe_reports_the_presented_certificate() {
    let cert = self_signed("tak.local");
    let (addr, _task) = server(&cert).await;
    let info = timeout(WAIT, probe_server_certificate(&addr))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(info.der, cert.der);
    let want: [u8; 32] = Sha256::digest(&cert.der).into();
    assert_eq!(info.sha256(), want);
    assert_eq!(info.fingerprint().split(':').count(), 32);
    assert!(info.to_pem().starts_with("-----BEGIN CERTIFICATE-----"));
}

#[tokio::test]
async fn pinned_self_signed_server_is_accepted_without_any_ca() {
    let cert = self_signed("not-the-hostname-we-dial");
    let (addr, _task) = server(&cert).await;

    // The workflow: probe, (user confirms the fingerprint), then pin.
    let seen = probe_server_certificate(&addr).await.unwrap();
    let tls = TlsConfig::new()
        .with_pinned_sha256_hex(&seen.fingerprint())
        .unwrap();
    let (handle, mut events) = TakClient::start(config(&addr, tls)).unwrap();
    let first = timeout(WAIT, events.recv()).await.unwrap().unwrap();
    assert!(matches!(first, ClientEvent::Connected { .. }), "{first:?}");
    handle.shutdown().await;
}

#[tokio::test]
async fn a_different_certificate_than_the_pin_is_rejected() {
    let genuine = self_signed("tak.local");
    let impostor = self_signed("tak.local");
    let (addr, _task) = server(&impostor).await;

    let pin: [u8; 32] = Sha256::digest(&genuine.der).into();
    let (handle, mut events) =
        TakClient::start(config(&addr, TlsConfig::new().with_pinned_sha256(pin))).unwrap();
    match timeout(WAIT, events.recv()).await.unwrap().unwrap() {
        ClientEvent::Disconnected { reason, .. } => {
            assert!(reason.to_lowercase().contains("certificate"), "{reason}");
        }
        other => panic!("connected to a server that does not match the pin: {other:?}"),
    }
    handle.shutdown().await;
}

#[tokio::test]
async fn unpinned_self_signed_server_is_rejected_when_only_a_ca_is_configured() {
    let cert = self_signed("tak.local");
    let (addr, _task) = server(&cert).await;
    let other_ca = self_signed("some-other-ca");
    let pem = format!(
        "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----\n",
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &other_ca.der)
    );
    let (handle, mut events) =
        TakClient::start(config(&addr, TlsConfig::new().with_ca_pem(pem))).unwrap();
    assert!(matches!(
        timeout(WAIT, events.recv()).await.unwrap().unwrap(),
        ClientEvent::Disconnected { .. }
    ));
    handle.shutdown().await;
}
