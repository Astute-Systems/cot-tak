//! Mutual-TLS tests against an in-process rustls server.

use cottak_core::{Event, Point};
use cottak_net::{
    ClientConfig, ClientEvent, ClientEvents, ProtocolPreference, TakClient, TlsConfig,
};
use rcgen::{
    BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::WebPkiClientVerifier;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_rustls::TlsAcceptor;

const WAIT: Duration = Duration::from_secs(10);

struct Ca {
    pem: String,
    params: CertificateParams,
    key: KeyPair,
}

struct Identity {
    cert_pem: String,
    key_pem: String,
}

fn new_ca(name: &str) -> Ca {
    let key = KeyPair::generate().unwrap();
    let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.distinguished_name.push(DnType::CommonName, name);
    params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
    let pem = params.self_signed(&key).unwrap().pem();
    Ca { pem, params, key }
}

fn issue(ca: &Ca, names: &[&str], usage: ExtendedKeyUsagePurpose) -> Identity {
    let key = KeyPair::generate().unwrap();
    let mut params =
        CertificateParams::new(names.iter().map(ToString::to_string).collect::<Vec<_>>()).unwrap();
    params.distinguished_name.push(
        DnType::CommonName,
        names.first().copied().unwrap_or("client"),
    );
    params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    params.extended_key_usages = vec![usage];
    let issuer = Issuer::from_params(&ca.params, &ca.key);
    let cert = params.signed_by(&key, &issuer).unwrap();
    Identity {
        cert_pem: cert.pem(),
        key_pem: key.serialize_pem(),
    }
}

fn provider() -> Arc<rustls::crypto::CryptoProvider> {
    Arc::new(rustls::crypto::ring::default_provider())
}

/// A TLS server that requires client certificates signed by `client_ca`.
async fn tls_server(server: &Identity, client_ca: &Ca) -> (TcpListener, TlsAcceptor, String) {
    let mut roots = rustls::RootCertStore::empty();
    for c in CertificateDer::pem_slice_iter(client_ca.pem.as_bytes()) {
        roots.add(c.unwrap()).unwrap();
    }
    let verifier = WebPkiClientVerifier::builder_with_provider(Arc::new(roots), provider())
        .build()
        .unwrap();
    let chain = CertificateDer::pem_slice_iter(server.cert_pem.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let key = PrivateKeyDer::from_pem_slice(server.key_pem.as_bytes()).unwrap();
    let config = rustls::ServerConfig::builder_with_provider(provider())
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_client_cert_verifier(verifier)
        .with_single_cert(chain, key)
        .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    (listener, TlsAcceptor::from(Arc::new(config)), addr)
}

fn client_config(addr: &str, tls: TlsConfig) -> ClientConfig {
    let mut c = ClientConfig::new(addr, "CLIENT")
        .with_tls(tls)
        .with_protocol(ProtocolPreference::XmlOnly);
    c.backoff = cottak_net::Backoff {
        min: Duration::from_millis(50),
        max: Duration::from_millis(100),
    };
    c
}

async fn next(events: &mut ClientEvents) -> ClientEvent {
    timeout(WAIT, events.recv())
        .await
        .expect("no client event")
        .expect("client stopped")
}

fn ev(uid: &str) -> Event {
    Event::new(
        uid,
        "a-f-G-U-C".parse().unwrap(),
        Point::new(1.0, 2.0).unwrap(),
    )
}

#[tokio::test]
async fn mutual_tls_succeeds_and_server_sees_the_client_certificate() {
    let ca = new_ca("Test CA");
    let server_id = issue(&ca, &["localhost"], ExtendedKeyUsagePurpose::ServerAuth);
    let client_id = issue(&ca, &["client-1"], ExtendedKeyUsagePurpose::ClientAuth);
    let (listener, acceptor, addr) = tls_server(&server_id, &ca).await;

    let tls = TlsConfig::new()
        .with_ca_pem(&ca.pem)
        .with_client_identity_pem(client_id.cert_pem, client_id.key_pem)
        .with_server_name("localhost");
    let (handle, mut events) = TakClient::start(client_config(&addr, tls)).unwrap();

    let (tcp, _) = timeout(WAIT, listener.accept()).await.unwrap().unwrap();
    let mut stream = timeout(WAIT, acceptor.accept(tcp))
        .await
        .unwrap()
        .expect("TLS handshake failed");
    assert!(
        stream.get_ref().1.peer_certificates().is_some(),
        "client certificate not presented"
    );
    assert!(matches!(
        next(&mut events).await,
        ClientEvent::Connected { .. }
    ));

    handle.send(ev("OVER-TLS")).await.unwrap();
    let mut buf = vec![0u8; 4096];
    let mut got = String::new();
    while !got.contains("</event>") {
        let n = timeout(WAIT, stream.read(&mut buf)).await.unwrap().unwrap();
        assert!(n > 0);
        got.push_str(std::str::from_utf8(&buf[..n]).unwrap());
    }
    assert!(got.contains("OVER-TLS"));
    handle.shutdown().await;
}

#[tokio::test]
async fn untrusted_server_certificate_is_rejected() {
    let real_ca = new_ca("Real CA");
    let rogue_ca = new_ca("Rogue CA");
    let rogue_server = issue(
        &rogue_ca,
        &["localhost"],
        ExtendedKeyUsagePurpose::ServerAuth,
    );
    let client_id = issue(&real_ca, &["client-1"], ExtendedKeyUsagePurpose::ClientAuth);
    let (listener, acceptor, addr) = tls_server(&rogue_server, &real_ca).await;
    tokio::spawn(async move {
        loop {
            let Ok((tcp, _)) = listener.accept().await else {
                return;
            };
            let _ = acceptor.accept(tcp).await; // fails: client aborts
        }
    });

    let tls = TlsConfig::new()
        .with_ca_pem(&real_ca.pem)
        .with_client_identity_pem(client_id.cert_pem, client_id.key_pem)
        .with_server_name("localhost");
    let (handle, mut events) = TakClient::start(client_config(&addr, tls)).unwrap();
    match next(&mut events).await {
        ClientEvent::Disconnected { reason, .. } => {
            assert!(reason.to_lowercase().contains("certificate"), "{reason}");
        }
        other => panic!("must not connect to an untrusted server: {other:?}"),
    }
    handle.shutdown().await;
}

#[tokio::test]
async fn wrong_server_name_is_rejected() {
    let ca = new_ca("Test CA");
    let server_id = issue(
        &ca,
        &["tak.example.com"],
        ExtendedKeyUsagePurpose::ServerAuth,
    );
    let client_id = issue(&ca, &["client-1"], ExtendedKeyUsagePurpose::ClientAuth);
    let (listener, acceptor, addr) = tls_server(&server_id, &ca).await;
    tokio::spawn(async move {
        loop {
            let Ok((tcp, _)) = listener.accept().await else {
                return;
            };
            let _ = acceptor.accept(tcp).await;
        }
    });

    // Connecting to 127.0.0.1 with no name override: the certificate is for another name.
    let tls = TlsConfig::new()
        .with_ca_pem(&ca.pem)
        .with_client_identity_pem(client_id.cert_pem, client_id.key_pem);
    let (handle, mut events) = TakClient::start(client_config(&addr, tls)).unwrap();
    assert!(matches!(
        next(&mut events).await,
        ClientEvent::Disconnected { .. }
    ));
    handle.shutdown().await;
}

#[tokio::test]
async fn missing_client_certificate_is_refused_by_the_server() {
    let ca = new_ca("Test CA");
    let server_id = issue(&ca, &["localhost"], ExtendedKeyUsagePurpose::ServerAuth);
    let (listener, acceptor, addr) = tls_server(&server_id, &ca).await;
    let rejected = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        acceptor.accept(tcp).await.is_err()
    });

    let tls = TlsConfig::new()
        .with_ca_pem(&ca.pem)
        .with_server_name("localhost"); // no identity
    let (handle, mut events) = TakClient::start(client_config(&addr, tls)).unwrap();
    // TLS 1.3 reports the refusal on first use of the connection, so Connected may precede Disconnected.
    loop {
        if matches!(next(&mut events).await, ClientEvent::Disconnected { .. }) {
            break;
        }
    }
    assert!(
        timeout(WAIT, rejected).await.unwrap().unwrap(),
        "server must reject the handshake"
    );
    handle.shutdown().await;
}
