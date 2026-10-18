//! Enrollment against an in-process HTTPS server that behaves like TAK Server's :8446 service.

use base64::Engine as _;
use cottak_enroll::{Credentials, EnrollConfig, Enroller, Error};
use cottak_net::TlsConfig;
use rcgen::{
    BasicConstraints, CertificateParams, CertificateSigningRequestParams, DnType,
    ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair, KeyUsagePurpose,
};
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

/// The CA that signs both the server's TLS certificate and issued client certificates.
struct Ca {
    pem: String,
    der: Vec<u8>,
    params: CertificateParams,
    key: KeyPair,
}

fn new_ca(name: &str) -> Ca {
    let key = KeyPair::generate().unwrap();
    let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.distinguished_name.push(DnType::CommonName, name);
    params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
    let cert = params.self_signed(&key).unwrap();
    Ca {
        pem: cert.pem(),
        der: cert.der().to_vec(),
        params,
        key,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Behavior {
    Normal,
    NoConfigEndpoint,
    WrongKeyInCertificate,
    Redirect,
    HugeResponse,
}

#[derive(Debug, Default, Clone)]
struct Seen {
    method: String,
    target: String,
    headers: HashMap<String, String>,
    body: String,
}

struct Server {
    port: u16,
    seen: Arc<Mutex<Vec<Seen>>>,
}

async fn start(ca: &Ca, behavior: Behavior) -> Server {
    // TLS certificate for "localhost", signed by the CA.
    let key = KeyPair::generate().unwrap();
    let mut params = CertificateParams::new(vec!["localhost".to_owned()]).unwrap();
    params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
    let issuer = Issuer::from_params(&ca.params, &ca.key);
    let cert = params.signed_by(&key, &issuer).unwrap();

    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let tls = rustls::ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(cert.der().to_vec())],
            PrivateKeyDer::from_pem_slice(key.serialize_pem().as_bytes()).unwrap(),
        )
        .unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(tls));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let seen = Arc::new(Mutex::new(Vec::new()));

    let ca_der = ca.der.clone();
    let ca_params = ca.params.clone();
    let ca_key_pem = ca.key.serialize_pem();
    let seen2 = seen.clone();
    tokio::spawn(async move {
        loop {
            let Ok((tcp, _)) = listener.accept().await else {
                return;
            };
            let acceptor = acceptor.clone();
            let (ca_der, ca_params, ca_key_pem, seen) = (
                ca_der.clone(),
                ca_params.clone(),
                ca_key_pem.clone(),
                seen2.clone(),
            );
            tokio::spawn(async move {
                let Ok(mut stream) = acceptor.accept(tcp).await else {
                    return;
                };
                let Some(req) = read_request(&mut stream).await else {
                    return;
                };
                seen.lock().unwrap().push(req.clone());
                let ca_key = KeyPair::from_pem(&ca_key_pem).unwrap();
                let reply = respond(&req, behavior, &ca_der, &ca_params, &ca_key);
                let _ = stream.write_all(reply.as_bytes()).await;
                let _ = stream.shutdown().await;
            });
        }
    });
    Server { port, seen }
}

async fn read_request<S: AsyncReadExt + Unpin>(stream: &mut S) -> Option<Seen> {
    let mut buf = Vec::new();
    let head_end = loop {
        let mut chunk = [0u8; 2048];
        let n = stream.read(&mut chunk).await.ok()?;
        if n == 0 {
            return None;
        }
        buf.extend_from_slice(&chunk[..n]);
        if let Some(i) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            break i;
        }
    };
    let head = String::from_utf8_lossy(&buf[..head_end]).into_owned();
    let mut lines = head.lines();
    let mut first = lines.next()?.split_whitespace();
    let (method, target) = (first.next()?.to_owned(), first.next()?.to_owned());
    let headers: HashMap<String, String> = lines
        .filter_map(|l| l.split_once(':'))
        .map(|(k, v)| (k.trim().to_ascii_lowercase(), v.trim().to_owned()))
        .collect();
    let len: usize = headers
        .get("content-length")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let mut body = buf[head_end + 4..].to_vec();
    while body.len() < len {
        let mut chunk = [0u8; 2048];
        let n = stream.read(&mut chunk).await.ok()?;
        if n == 0 {
            return None;
        }
        body.extend_from_slice(&chunk[..n]);
    }
    Some(Seen {
        method,
        target,
        headers,
        body: String::from_utf8_lossy(&body).into_owned(),
    })
}

fn http(status: &str, content_type: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

fn respond(
    req: &Seen,
    behavior: Behavior,
    ca_der: &[u8],
    ca_params: &CertificateParams,
    ca_key: &KeyPair,
) -> String {
    let expected = format!("Basic {}", B64.encode("alice:s3cret"));
    if req.headers.get("authorization") != Some(&expected) {
        return http("401 Unauthorized", "text/plain", "bad credentials");
    }
    if behavior == Behavior::Redirect {
        return "HTTP/1.1 302 Found\r\nLocation: https://evil.example/\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".into();
    }
    if req.target.starts_with("/Marti/api/tls/config") {
        if behavior == Behavior::NoConfigEndpoint {
            return http("404 Not Found", "text/plain", "no such endpoint");
        }
        let xml = r#"<?xml version="1.0"?><ns2:certificateConfig xmlns:ns2="http://bbn.com/marti/xml/config">
            <ns2:nameEntries><ns2:nameEntry name="O" value="Acme"/><ns2:nameEntry name="OU" value="Ops"/></ns2:nameEntries>
            </ns2:certificateConfig>"#;
        return http("200 OK", "application/xml", xml);
    }
    if req.method == "POST" && req.target.starts_with("/Marti/api/tls/signClient/v2") {
        if behavior == Behavior::HugeResponse {
            return http("200 OK", "application/json", &"x".repeat(2 * 1024 * 1024));
        }
        let issuer = Issuer::from_params(ca_params, ca_key);
        let csr =
            CertificateSigningRequestParams::from_pem(&req.body).expect("client sent a bad CSR");
        let cert = if behavior == Behavior::WrongKeyInCertificate {
            let other = KeyPair::generate().unwrap();
            CertificateParams::new(vec!["x".to_owned()])
                .unwrap()
                .signed_by(&other, &issuer)
                .unwrap()
        } else {
            csr.signed_by(&issuer).unwrap()
        };
        // Like the real service: bare base64 DER, no PEM headers.
        let json = serde_json::json!({
            "signedCert": B64.encode(cert.der()),
            "ca0": B64.encode(ca_der),
        });
        return http("200 OK", "application/json", &json.to_string());
    }
    http("404 Not Found", "text/plain", "unknown")
}

fn enroller_config(port: u16, ca: &Ca) -> EnrollConfig {
    // The certificate is for "localhost" while we dial 127.0.0.1: exercises the server-name override.
    let trust = TlsConfig::new()
        .with_ca_pem(&ca.pem)
        .with_server_name("localhost");
    let mut cfg = EnrollConfig::new("127.0.0.1", trust);
    cfg.port = port;
    cfg
}

#[tokio::test]
async fn full_enrollment_produces_a_working_identity() {
    let ca = new_ca("Test CA");
    let server = start(&ca, Behavior::Normal).await;
    let enroller = Enroller::new(enroller_config(server.port, &ca))
        .await
        .unwrap();

    let identity = enroller
        .enroll(&Credentials::new("alice", "s3cret"), "ANDROID-1234")
        .await
        .unwrap();

    identity.verify_key_matches().unwrap();
    assert_eq!(identity.ca_pem.len(), 1);
    assert!(
        identity
            .certificate_pem
            .starts_with("-----BEGIN CERTIFICATE-----"),
        "bare base64 must be wrapped"
    );
    identity.tls_config().unwrap().validate().unwrap();

    // What the server saw.
    let seen = server.seen.lock().unwrap().clone();
    assert_eq!(seen.len(), 2);
    assert_eq!(
        (seen[0].method.as_str(), seen[0].target.as_str()),
        ("GET", "/Marti/api/tls/config")
    );
    let sign = &seen[1];
    assert_eq!(sign.method, "POST");
    assert!(
        sign.target.starts_with("/Marti/api/tls/signClient/v2?"),
        "{}",
        sign.target
    );
    assert!(
        sign.target.contains("clientUid=ANDROID-1234"),
        "{}",
        sign.target
    );
    assert_eq!(
        sign.headers.get("content-type").map(String::as_str),
        Some("application/pkcs10")
    );
    assert!(sign.body.starts_with("-----BEGIN CERTIFICATE REQUEST-----"));
    let csr = CertificateSigningRequestParams::from_pem(&sign.body).unwrap();
    let dn = &csr.params.distinguished_name;
    assert_eq!(
        dn.get(&DnType::CommonName),
        Some(&rcgen::DnValue::Utf8String("alice".into()))
    );
    assert_eq!(
        dn.get(&DnType::OrganizationName),
        Some(&rcgen::DnValue::Utf8String("Acme".into()))
    );
    assert_eq!(
        dn.get(&DnType::OrganizationalUnitName),
        Some(&rcgen::DnValue::Utf8String("Ops".into()))
    );
}

#[tokio::test]
async fn wrong_password_is_reported_as_unauthorized() {
    let ca = new_ca("Test CA");
    let server = start(&ca, Behavior::Normal).await;
    let enroller = Enroller::new(enroller_config(server.port, &ca))
        .await
        .unwrap();
    let err = enroller
        .enroll(&Credentials::new("alice", "wrong"), "U")
        .await
        .unwrap_err();
    assert!(matches!(err, Error::Unauthorized(401)), "{err:?}");
}

#[tokio::test]
async fn untrusted_server_is_refused_before_credentials_are_sent() {
    let ca = new_ca("Test CA");
    let other = new_ca("Some Other CA");
    let server = start(&ca, Behavior::Normal).await;
    let enroller = Enroller::new(enroller_config(server.port, &other))
        .await
        .unwrap();
    let err = enroller
        .enroll(&Credentials::new("alice", "s3cret"), "U")
        .await
        .unwrap_err();
    assert!(matches!(err, Error::Http(_)), "{err:?}");
    assert!(
        server.seen.lock().unwrap().is_empty(),
        "no HTTP request may reach an untrusted server"
    );
}

#[tokio::test]
async fn old_servers_without_the_config_endpoint_still_work() {
    let ca = new_ca("Test CA");
    let server = start(&ca, Behavior::NoConfigEndpoint).await;
    let enroller = Enroller::new(enroller_config(server.port, &ca))
        .await
        .unwrap();
    let identity = enroller
        .enroll(&Credentials::new("alice", "s3cret"), "U")
        .await
        .unwrap();
    identity.verify_key_matches().unwrap();

    let seen = server.seen.lock().unwrap().clone();
    let csr = CertificateSigningRequestParams::from_pem(&seen[1].body).unwrap();
    assert!(csr
        .params
        .distinguished_name
        .get(&DnType::OrganizationName)
        .is_none());
}

#[tokio::test]
async fn a_certificate_for_another_key_is_rejected() {
    let ca = new_ca("Test CA");
    let server = start(&ca, Behavior::WrongKeyInCertificate).await;
    let enroller = Enroller::new(enroller_config(server.port, &ca))
        .await
        .unwrap();
    let err = enroller
        .enroll(&Credentials::new("alice", "s3cret"), "U")
        .await
        .unwrap_err();
    assert!(matches!(err, Error::KeyMismatch), "{err:?}");
}

#[tokio::test]
async fn redirects_are_not_followed() {
    let ca = new_ca("Test CA");
    let server = start(&ca, Behavior::Redirect).await;
    let enroller = Enroller::new(enroller_config(server.port, &ca))
        .await
        .unwrap();
    let err = enroller
        .enroll(&Credentials::new("alice", "s3cret"), "U")
        .await
        .unwrap_err();
    assert!(matches!(err, Error::Status { status: 302, .. }), "{err:?}");
}

#[tokio::test]
async fn oversized_responses_are_rejected() {
    let ca = new_ca("Test CA");
    let server = start(&ca, Behavior::HugeResponse).await;
    let enroller = Enroller::new(enroller_config(server.port, &ca))
        .await
        .unwrap();
    // The config response is fine; the signing response is too large.
    let err = enroller
        .enroll(&Credentials::new("alice", "s3cret"), "U")
        .await
        .unwrap_err();
    assert!(matches!(err, Error::ResponseTooLarge(_)), "{err:?}");
}

/// The point of enrollment: the issued identity must work for mutual TLS with the streaming client.
#[tokio::test]
async fn issued_identity_authenticates_to_a_streaming_server() {
    use cottak_net::{ClientConfig, ClientEvent, ProtocolPreference, TakClient};
    use rustls::server::WebPkiClientVerifier;
    use std::time::Duration;
    use tokio::time::timeout;

    let ca = new_ca("Test CA");
    let enroll_server = start(&ca, Behavior::Normal).await;
    let enroller = Enroller::new(enroller_config(enroll_server.port, &ca))
        .await
        .unwrap();
    let identity = enroller
        .enroll(&Credentials::new("alice", "s3cret"), "U")
        .await
        .unwrap();

    // A streaming server that requires client certificates issued by the same CA.
    let server_key = KeyPair::generate().unwrap();
    let mut params = CertificateParams::new(vec!["localhost".to_owned()]).unwrap();
    params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
    let server_cert = params
        .signed_by(&server_key, &Issuer::from_params(&ca.params, &ca.key))
        .unwrap();
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut roots = rustls::RootCertStore::empty();
    roots.add(CertificateDer::from(ca.der.clone())).unwrap();
    let verifier = WebPkiClientVerifier::builder_with_provider(Arc::new(roots), provider.clone())
        .build()
        .unwrap();
    let tls = rustls::ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_client_cert_verifier(verifier)
        .with_single_cert(
            vec![CertificateDer::from(server_cert.der().to_vec())],
            PrivateKeyDer::from_pem_slice(server_key.serialize_pem().as_bytes()).unwrap(),
        )
        .unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(tls));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let accepted = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let stream = acceptor
            .accept(tcp)
            .await
            .expect("mutual TLS handshake failed");
        let has_cert = stream.get_ref().1.peer_certificates().is_some();
        // Keep the connection open until the test finishes.
        tokio::time::sleep(Duration::from_secs(2)).await;
        has_cert
    });

    let trust = identity.tls_config().unwrap().with_server_name("localhost");
    let config = ClientConfig::new(addr, "CLIENT")
        .with_tls(trust)
        .with_protocol(ProtocolPreference::XmlOnly);
    let (handle, mut events) = TakClient::start(config).unwrap();
    let first = timeout(Duration::from_secs(10), events.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(first, ClientEvent::Connected { .. }), "{first:?}");
    assert!(accepted.await.unwrap(), "server saw no client certificate");
    handle.shutdown().await;
}

// ---- bring-your-own key (RSA) ----

/// Throw-away keys generated with `openssl genpkey`, used only by these tests.
const RSA_2048: &str = include_str!("rsa2048-test-key.pem");
const RSA_1024: &str = include_str!("rsa1024-test-key.pem");

#[tokio::test]
async fn rsa_keys_can_be_used_for_enrollment() {
    let ca = new_ca("Test CA");
    let server = start(&ca, Behavior::Normal).await;
    let enroller = Enroller::new(enroller_config(server.port, &ca))
        .await
        .unwrap();

    let identity = enroller
        .enroll_with_key(
            &Credentials::new("alice", "s3cret"),
            "ANDROID-1234",
            RSA_2048,
        )
        .await
        .unwrap();
    identity.verify_key_matches().unwrap();
    identity.tls_config().unwrap().validate().unwrap();

    // The identity carries the key we supplied, and the request was signed with RSA.
    let supplied = KeyPair::from_pem(RSA_2048).unwrap();
    assert_eq!(identity.private_key_pem, supplied.serialize_pem());
    let seen = server.seen.lock().unwrap().clone();
    let csr = CertificateSigningRequestParams::from_pem(&seen[1].body).unwrap();
    assert_eq!(csr.public_key.algorithm(), &rcgen::PKCS_RSA_SHA256);
    // The private key is never sent.
    assert!(!seen.iter().any(|r| r.body.contains("PRIVATE KEY")));
}

#[tokio::test]
async fn supplied_ecdsa_keys_work_too() {
    let ca = new_ca("Test CA");
    let server = start(&ca, Behavior::Normal).await;
    let enroller = Enroller::new(enroller_config(server.port, &ca))
        .await
        .unwrap();
    let key = KeyPair::generate().unwrap();
    let identity = enroller
        .enroll_with_key(
            &Credentials::new("alice", "s3cret"),
            "U",
            &key.serialize_pem(),
        )
        .await
        .unwrap();
    assert_eq!(identity.private_key_pem, key.serialize_pem());
}

#[tokio::test]
async fn unusable_keys_fail_before_any_request_is_made() {
    let ca = new_ca("Test CA");
    let server = start(&ca, Behavior::Normal).await;
    let enroller = Enroller::new(enroller_config(server.port, &ca))
        .await
        .unwrap();
    let creds = Credentials::new("alice", "s3cret");

    let pkcs1 = "-----BEGIN RSA PRIVATE KEY-----\nAAAA\n-----END RSA PRIVATE KEY-----\n";
    for bad in [
        "",
        "not a key",
        pkcs1,
        RSA_1024,
        "-----BEGIN CERTIFICATE-----\nAAAA\n-----END CERTIFICATE-----\n",
    ] {
        assert!(
            enroller.enroll_with_key(&creds, "U", bad).await.is_err(),
            "{}",
            bad.lines().next().unwrap_or("")
        );
    }
    assert!(
        server.seen.lock().unwrap().is_empty(),
        "a bad key must be rejected locally, before contacting the server"
    );
}
