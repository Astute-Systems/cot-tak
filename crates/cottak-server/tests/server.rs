//! End-to-end tests: the real `TakClient` against the embedded server.

use cottak_core::detail::{Contact, Marti, MartiDest};
use cottak_core::messages::control;
use cottak_core::{Event, Point};
use cottak_net::{
    Backoff, ClientConfig, ClientEvent, ClientEvents, ClientHandle, Connection, Incoming, Protocol,
    ProtocolPreference, TakClient, TlsConfig,
};
use cottak_server::{Error, Server, ServerConfig, ServerEvent, ServerTlsConfig};
use rcgen::{
    BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};

const WAIT: Duration = Duration::from_secs(10);

async fn start(config: ServerConfig) -> Server {
    Server::start(config).await.expect("server must start")
}

fn tcp_config() -> ServerConfig {
    ServerConfig::new().listen_tcp("127.0.0.1:0")
}

fn client_config(addr: SocketAddr, uid: &str, protocol: ProtocolPreference) -> ClientConfig {
    let mut c = ClientConfig::new(addr.to_string(), uid).with_protocol(protocol);
    c.backoff = Backoff {
        min: Duration::from_millis(50),
        max: Duration::from_millis(100),
    };
    c
}

fn connect(
    addr: SocketAddr,
    uid: &str,
    protocol: ProtocolPreference,
) -> (ClientHandle, ClientEvents) {
    TakClient::start(client_config(addr, uid, protocol)).unwrap()
}

fn pli(uid: &str, callsign: &str, lat: f64) -> Event {
    Event::new(
        uid,
        "a-f-G-U-C".parse().unwrap(),
        Point::new(lat, 153.0).unwrap(),
    )
    .with(Contact::new(callsign))
}

async fn wait_until(what: &str, mut cond: impl FnMut() -> bool) {
    let deadline = tokio::time::Instant::now() + WAIT;
    while !cond() {
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for {what}"
        );
        sleep(Duration::from_millis(10)).await;
    }
}

/// Receives events until one has `uid`; returns the UIDs seen, ending with it.
async fn recv_until(events: &mut ClientEvents, uid: &str) -> Vec<String> {
    let mut seen = Vec::new();
    loop {
        let item = timeout(WAIT, events.recv())
            .await
            .unwrap_or_else(|_| panic!("timed out waiting for {uid}; saw {seen:?}"))
            .expect("client stopped");
        if let ClientEvent::Event(ev) = item {
            seen.push(ev.uid.clone());
            if ev.uid == uid {
                return seen;
            }
        }
    }
}

async fn next_event(events: &mut ClientEvents) -> ClientEvent {
    timeout(WAIT, events.recv())
        .await
        .expect("no client event")
        .expect("client stopped")
}

#[tokio::test]
async fn events_reach_other_clients_but_are_not_echoed() {
    let server = start(tcp_config()).await;
    let addr = server.local_addrs()[0];
    let (a, mut a_events) = connect(addr, "A", ProtocolPreference::XmlOnly);
    let (b, mut b_events) = connect(addr, "B", ProtocolPreference::XmlOnly);
    wait_until("two clients", || server.clients().len() == 2).await;

    a.send(pli("A-1", "ALPHA", 1.0)).await.unwrap();
    b.send(pli("B-1", "BRAVO", 2.0)).await.unwrap();

    assert_eq!(recv_until(&mut b_events, "A-1").await, ["A-1"]);
    let seen_by_a = recv_until(&mut a_events, "B-1").await;
    assert_eq!(seen_by_a, ["B-1"], "A must not receive its own event");

    a.shutdown().await;
    b.shutdown().await;
    server.shutdown().await;
}

#[tokio::test]
async fn protobuf_is_negotiated_and_clients_on_both_protocols_interoperate() {
    let server = start(tcp_config()).await;
    let addr = server.local_addrs()[0];
    let (pb, mut pb_events) = connect(addr, "PB", ProtocolPreference::PreferProtobuf);
    let (xml, mut xml_events) = connect(addr, "XML", ProtocolPreference::XmlOnly);

    loop {
        if matches!(
            next_event(&mut pb_events).await,
            ClientEvent::ProtocolChanged(Protocol::V1)
        ) {
            break;
        }
    }
    wait_until("protocol recorded", || {
        server.clients().iter().any(|c| c.protocol == Protocol::V1)
    })
    .await;
    wait_until("two clients", || server.clients().len() == 2).await;

    pb.send(pli("FROM-PB", "PBCALL", 3.0)).await.unwrap();
    xml.send(pli("FROM-XML", "XMLCALL", 4.0)).await.unwrap();

    assert_eq!(recv_until(&mut xml_events, "FROM-PB").await, ["FROM-PB"]);
    assert_eq!(recv_until(&mut pb_events, "FROM-XML").await, ["FROM-XML"]);

    let protocols: Vec<_> = server.clients().iter().map(|c| c.protocol).collect();
    assert!(protocols.contains(&Protocol::V1) && protocols.contains(&Protocol::Xml));

    pb.shutdown().await;
    xml.shutdown().await;
    server.shutdown().await;
}

#[tokio::test]
async fn disabling_versions_keeps_clients_on_xml() {
    let server = start(tcp_config().with_protocol_versions(&[])).await;
    let (pb, mut events) = connect(
        server.local_addrs()[0],
        "PB",
        ProtocolPreference::PreferProtobuf,
    );
    assert!(matches!(
        next_event(&mut events).await,
        ClientEvent::Connected { .. }
    ));
    pb.send(pli("PB", "PBCALL", 1.0)).await.unwrap();
    wait_until("event stored", || !server.stored_events().is_empty()).await;
    assert!(server.clients().iter().all(|c| c.protocol == Protocol::Xml));
    pb.shutdown().await;
    server.shutdown().await;
}

#[tokio::test]
async fn marti_dest_limits_delivery_to_named_callsigns() {
    let server = start(tcp_config()).await;
    let addr = server.local_addrs()[0];
    let (a, _a_events) = connect(addr, "A", ProtocolPreference::XmlOnly);
    let (b, mut b_events) = connect(addr, "B", ProtocolPreference::XmlOnly);
    let (c, mut c_events) = connect(addr, "C", ProtocolPreference::XmlOnly);
    a.send(pli("A", "ALPHA", 1.0)).await.unwrap();
    b.send(pli("B", "BRAVO", 1.0)).await.unwrap();
    c.send(pli("C", "CHARLIE", 1.0)).await.unwrap();
    wait_until("callsigns learned", || {
        server
            .clients()
            .iter()
            .filter(|c| c.callsign.is_some())
            .count()
            == 3
    })
    .await;

    let directed = pli("DIRECTED", "ALPHA", 1.0).with(Marti {
        dests: vec![MartiDest::callsign("BRAVO")],
    });
    a.send(directed).await.unwrap();
    a.send(pli("SENTINEL", "ALPHA", 1.0)).await.unwrap();

    assert!(recv_until(&mut b_events, "SENTINEL")
        .await
        .contains(&"DIRECTED".to_owned()));
    let seen_by_c = recv_until(&mut c_events, "SENTINEL").await;
    assert!(
        !seen_by_c.contains(&"DIRECTED".to_owned()),
        "CHARLIE must not see a message addressed to BRAVO: {seen_by_c:?}"
    );

    for h in [&a, &b, &c] {
        h.shutdown().await;
    }
    server.shutdown().await;
}

#[tokio::test]
async fn new_clients_get_the_latest_positions_and_deletes_remove_them() {
    let server = start(tcp_config()).await;
    let addr = server.local_addrs()[0];
    let (a, _a_events) = connect(addr, "A", ProtocolPreference::XmlOnly);
    a.send(pli("ENTITY", "ALPHA", 1.0)).await.unwrap();
    a.send(pli("ENTITY", "ALPHA", 2.0)).await.unwrap();
    wait_until("latest stored", || {
        server
            .stored_events()
            .iter()
            .any(|e| e.uid == "ENTITY" && (e.point.lat - 2.0).abs() < 1e-9)
    })
    .await;
    assert_eq!(server.stored_events().len(), 1);

    let (b, mut b_events) = connect(addr, "B", ProtocolPreference::XmlOnly);
    loop {
        if let ClientEvent::Event(ev) = next_event(&mut b_events).await {
            assert_eq!(ev.uid, "ENTITY");
            assert!((ev.point.lat - 2.0).abs() < 1e-9, "must replay the latest");
            break;
        }
    }

    a.send(control::delete("ENTITY", "a-f-G-U-C").unwrap())
        .await
        .unwrap();
    wait_until("entity deleted", || server.stored_events().is_empty()).await;

    a.shutdown().await;
    b.shutdown().await;
    server.shutdown().await;
}

#[tokio::test]
async fn replay_can_be_disabled() {
    let mut config = tcp_config();
    config.replay_on_connect = false;
    let server = start(config).await;
    let addr = server.local_addrs()[0];
    let (a, _a_events) = connect(addr, "A", ProtocolPreference::XmlOnly);
    a.send(pli("OLD", "ALPHA", 1.0)).await.unwrap();
    wait_until("stored", || !server.stored_events().is_empty()).await;

    let (b, mut b_events) = connect(addr, "B", ProtocolPreference::XmlOnly);
    wait_until("two clients", || server.clients().len() == 2).await;
    a.send(pli("NEW", "ALPHA", 1.0)).await.unwrap();
    assert_eq!(recv_until(&mut b_events, "NEW").await, ["NEW"]);

    a.shutdown().await;
    b.shutdown().await;
    server.shutdown().await;
}

/// Reads from a raw connection until `want` returns `Some`.
async fn read_raw<T>(
    conn: &mut Connection<TcpStream>,
    mut want: impl FnMut(&Event) -> Option<T>,
) -> T {
    loop {
        match timeout(WAIT, conn.read()).await.expect("timed out") {
            Ok(Incoming::Event(ev)) => {
                if let Some(t) = want(&ev) {
                    return t;
                }
            }
            Ok(_) => {}
            Err(e) => panic!("connection failed: {e}"),
        }
    }
}

#[tokio::test]
async fn ping_is_answered_with_pong_and_not_routed() {
    let server = start(tcp_config()).await;
    let mut observer = server.subscribe();
    let stream = TcpStream::connect(server.local_addrs()[0]).await.unwrap();
    let mut conn = Connection::new(stream, 1 << 20);

    // The server offers protocol versions first.
    read_raw(&mut conn, |e| {
        cottak_proto::negotiate::parse_offer(e).filter(|v| v == &[1])
    })
    .await;
    conn.write_event(&control::ping("RAW").unwrap())
        .await
        .unwrap();
    read_raw(&mut conn, |e| control::is_pong(e).then_some(())).await;

    while let Ok(item) = observer.try_recv() {
        assert!(
            !matches!(item, ServerEvent::Event { .. }),
            "keepalive traffic must not be routed: {item:?}"
        );
    }
    server.shutdown().await;
}

#[tokio::test]
async fn silent_clients_are_disconnected() {
    let server = start(tcp_config().with_read_timeout(Duration::from_millis(200))).await;
    let stream = TcpStream::connect(server.local_addrs()[0]).await.unwrap();
    let mut conn = Connection::new(stream, 1 << 20);
    let result = timeout(WAIT, async {
        loop {
            if let Err(e) = conn.read().await {
                return e;
            }
        }
    })
    .await
    .expect("server must hang up on an idle client");
    assert!(matches!(result, cottak_net::Error::Closed), "{result}");
    wait_until("client removed", || server.clients().is_empty()).await;
    server.shutdown().await;
}

#[tokio::test]
async fn connection_limit_is_enforced() {
    let mut config = tcp_config();
    config.max_clients = 1;
    let server = start(config).await;
    let addr = server.local_addrs()[0];
    let _first = TcpStream::connect(addr).await.unwrap();
    wait_until("first registered", || server.clients().len() == 1).await;

    let second = TcpStream::connect(addr).await.unwrap();
    let mut conn = Connection::new(second, 1 << 20);
    let result = timeout(WAIT, conn.read()).await.expect("must be closed");
    assert!(result.is_err(), "second connection must be refused");
    assert_eq!(server.clients().len(), 1);
    server.shutdown().await;
}

#[tokio::test]
async fn observers_see_connections_and_events() {
    let server = start(tcp_config()).await;
    let mut observer = server.subscribe();
    let (a, _events) = connect(server.local_addrs()[0], "A", ProtocolPreference::XmlOnly);
    a.send(pli("OBSERVED", "ALPHA", 1.0)).await.unwrap();

    let (mut connected, mut routed) = (false, false);
    while !(connected && routed) {
        match timeout(WAIT, observer.recv()).await.unwrap().unwrap() {
            ServerEvent::Connected(info) => {
                assert!(!info.tls && info.protocol == Protocol::Xml);
                connected = true;
            }
            ServerEvent::Event { from, event } => {
                assert!(from.is_some() && event.uid == "OBSERVED");
                routed = true;
            }
            _ => {}
        }
    }
    a.shutdown().await;
    loop {
        if let ServerEvent::Disconnected(info) =
            timeout(WAIT, observer.recv()).await.unwrap().unwrap()
        {
            assert_eq!(info.callsign.as_deref(), Some("ALPHA"));
            break;
        }
    }
    server.shutdown().await;
}

#[tokio::test]
async fn server_can_inject_events() {
    let server = start(tcp_config()).await;
    let (a, mut events) = connect(server.local_addrs()[0], "A", ProtocolPreference::XmlOnly);
    wait_until("client", || server.clients().len() == 1).await;
    server.broadcast(pli("FROM-SERVER", "SERVER", 9.0));
    assert_eq!(
        recv_until(&mut events, "FROM-SERVER").await,
        ["FROM-SERVER"]
    );
    a.shutdown().await;
    server.shutdown().await;
}

#[tokio::test]
async fn shutdown_disconnects_clients() {
    let server = start(tcp_config()).await;
    let (a, mut events) = connect(server.local_addrs()[0], "A", ProtocolPreference::XmlOnly);
    wait_until("client", || server.clients().len() == 1).await;
    timeout(WAIT, server.shutdown())
        .await
        .expect("shutdown must not hang");
    loop {
        if matches!(
            next_event(&mut events).await,
            ClientEvent::Disconnected { .. }
        ) {
            break;
        }
    }
    a.shutdown().await;
}

#[tokio::test]
async fn configuration_errors_are_reported() {
    let err = Server::start(ServerConfig::new()).await.unwrap_err();
    assert!(matches!(err, Error::InvalidConfig(_)), "{err}");

    let err = Server::start(tcp_config().with_protocol_versions(&[7]))
        .await
        .unwrap_err();
    assert!(matches!(err, Error::InvalidConfig(_)), "{err}");

    let ca = new_ca("CA");
    let id = issue(&ca, &["localhost"], ExtendedKeyUsagePurpose::ServerAuth);
    let other = issue(&ca, &["other"], ExtendedKeyUsagePurpose::ServerAuth);
    // Certificate and key from different identities.
    let mismatched = ServerTlsConfig::server_only(id.cert_pem.clone(), other.key_pem);
    let err = Server::start(ServerConfig::new().listen_tls("127.0.0.1:0", mismatched))
        .await
        .unwrap_err();
    assert!(matches!(err, Error::TlsConfig(_)), "{err}");

    // Client authentication must never be silently dropped.
    let no_ca = ServerTlsConfig::mutual(id.cert_pem, id.key_pem, "no certificates here");
    let err = Server::start(ServerConfig::new().listen_tls("127.0.0.1:0", no_ca))
        .await
        .unwrap_err();
    assert!(matches!(err, Error::TlsConfig(_)), "{err}");

    let debug = format!(
        "{:?}",
        ServerTlsConfig::server_only("c", "SECRET-KEY-MATERIAL")
    );
    assert!(!debug.contains("SECRET"), "{debug}");
}

// ---- TLS ----

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

fn tls_client(addr: SocketAddr, uid: &str, trust: &Ca, id: Option<Identity>) -> ClientConfig {
    let mut tls = TlsConfig::new()
        .with_ca_pem(&trust.pem)
        .with_server_name("localhost");
    if let Some(id) = id {
        tls = tls.with_client_identity_pem(id.cert_pem, id.key_pem);
    }
    client_config(addr, uid, ProtocolPreference::PreferProtobuf).with_tls(tls)
}

#[tokio::test]
async fn mutual_tls_admits_trusted_clients_and_rejects_others() {
    let ca = new_ca("Test CA");
    let rogue_ca = new_ca("Rogue CA");
    let server_id = issue(&ca, &["localhost"], ExtendedKeyUsagePurpose::ServerAuth);
    let tls = ServerTlsConfig::mutual(server_id.cert_pem, server_id.key_pem, ca.pem.clone());
    let server = start(ServerConfig::new().listen_tls("127.0.0.1:0", tls)).await;
    let addr = server.local_addrs()[0];

    // A client with a certificate from the right CA is served, over protobuf.
    let good = issue(&ca, &["good"], ExtendedKeyUsagePurpose::ClientAuth);
    let (a, mut a_events) = TakClient::start(tls_client(addr, "GOOD", &ca, Some(good))).unwrap();
    loop {
        if matches!(
            next_event(&mut a_events).await,
            ClientEvent::ProtocolChanged(Protocol::V1)
        ) {
            break;
        }
    }
    wait_until("TLS client registered", || {
        server.clients().iter().any(|c| c.tls)
    })
    .await;

    // A certificate from another CA, and no certificate at all, never get in.
    let bad = issue(&rogue_ca, &["bad"], ExtendedKeyUsagePurpose::ClientAuth);
    for id in [Some(bad), None] {
        let (h, mut events) = TakClient::start(tls_client(addr, "BAD", &ca, id)).unwrap();
        loop {
            if matches!(
                next_event(&mut events).await,
                ClientEvent::Disconnected { .. }
            ) {
                break;
            }
        }
        assert_eq!(server.clients().len(), 1, "untrusted client was admitted");
        h.shutdown().await;
    }

    a.shutdown().await;
    server.shutdown().await;
}

#[tokio::test]
async fn tls_and_tcp_listeners_share_one_hub() {
    let ca = new_ca("Test CA");
    let server_id = issue(&ca, &["localhost"], ExtendedKeyUsagePurpose::ServerAuth);
    let tls = ServerTlsConfig::server_only(server_id.cert_pem, server_id.key_pem);
    let server = start(
        ServerConfig::new()
            .listen_tcp("127.0.0.1:0")
            .listen_tls("127.0.0.1:0", tls),
    )
    .await;
    let (tcp_addr, tls_addr) = (server.local_addrs()[0], server.local_addrs()[1]);

    let (plain, mut plain_events) = connect(tcp_addr, "PLAIN", ProtocolPreference::XmlOnly);
    let (secure, _secure_events) =
        TakClient::start(tls_client(tls_addr, "SECURE", &ca, None)).unwrap();
    wait_until("both listeners in use", || {
        let c = server.clients();
        c.len() == 2 && c.iter().any(|c| c.tls) && c.iter().any(|c| !c.tls)
    })
    .await;

    secure.send(pli("SECURE", "SEC", 1.0)).await.unwrap();
    assert_eq!(recv_until(&mut plain_events, "SECURE").await, ["SECURE"]);

    plain.shutdown().await;
    secure.shutdown().await;
    server.shutdown().await;
}

#[tokio::test]
async fn forwarding_other_entities_does_not_change_who_a_client_is() {
    let server = start(tcp_config()).await;
    let (a, _events) = connect(server.local_addrs()[0], "A", ProtocolPreference::XmlOnly);
    a.send(pli("A", "ALPHA", 1.0)).await.unwrap();
    wait_until("identified", || {
        server
            .clients()
            .iter()
            .any(|c| c.uid.as_deref() == Some("A"))
    })
    .await;

    a.send(pli("OTHER-ENTITY", "SOMEONE-ELSE", 2.0))
        .await
        .unwrap();
    a.send(pli("A", "ALPHA", 3.0)).await.unwrap();
    wait_until("both processed", || {
        server
            .stored_events()
            .iter()
            .any(|e| e.uid == "A" && (e.point.lat - 3.0).abs() < 1e-9)
    })
    .await;
    let info = &server.clients()[0];
    assert_eq!(
        (info.uid.as_deref(), info.callsign.as_deref()),
        (Some("A"), Some("ALPHA"))
    );

    a.shutdown().await;
    server.shutdown().await;
}
