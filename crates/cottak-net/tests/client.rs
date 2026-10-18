//! End-to-end tests of `TakClient` against in-process mock TAK Servers.

use bytes::BytesMut;
use cottak_core::messages::control;
use cottak_core::{Event, Point};
use cottak_net::{
    ClientConfig, ClientEvent, ClientEvents, Protocol, ProtocolPreference, TakClient,
};
use cottak_proto::{frame, negotiate, TakFrame};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

const WAIT: Duration = Duration::from_secs(10);

fn ev(uid: &str) -> Event {
    Event::new(
        uid,
        "a-f-G-U-C".parse().unwrap(),
        Point::new(1.0, 2.0).unwrap(),
    )
}

/// The server's end of a connection, able to speak both wire formats.
struct Peer<S> {
    io: S,
    buf: BytesMut,
}

impl<S: AsyncRead + AsyncWrite + Unpin> Peer<S> {
    fn new(io: S) -> Self {
        Self {
            io,
            buf: BytesMut::new(),
        }
    }

    async fn read_xml(&mut self) -> (String, Event) {
        const END: &[u8] = b"</event>";
        loop {
            if let Some(i) = self.buf.windows(END.len()).position(|w| w == END) {
                let raw = self.buf.split_to(i + END.len());
                let text = String::from_utf8(raw.to_vec()).unwrap();
                let event = Event::from_xml_str(&text).unwrap();
                return (text, event);
            }
            let n = timeout(WAIT, self.io.read_buf(&mut self.buf))
                .await
                .expect("read timed out")
                .unwrap();
            assert!(n > 0, "client closed the connection");
        }
    }

    async fn write_xml(&mut self, event: &Event) {
        self.io
            .write_all(event.to_xml_string().unwrap().as_bytes())
            .await
            .unwrap();
    }

    async fn read_frame(&mut self) -> Event {
        loop {
            if let Some(f) =
                TakFrame::decode_stream(&mut self.buf, frame::DEFAULT_MAX_FRAME_LEN).unwrap()
            {
                return f.event.expect("frame without event");
            }
            let n = timeout(WAIT, self.io.read_buf(&mut self.buf))
                .await
                .expect("read timed out")
                .unwrap();
            assert!(n > 0, "client closed the connection");
        }
    }
}

async fn listener() -> (TcpListener, String) {
    let l = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = l.local_addr().unwrap().to_string();
    (l, addr)
}

async fn accept(l: &TcpListener) -> Peer<TcpStream> {
    Peer::new(
        timeout(WAIT, l.accept())
            .await
            .expect("no client connected")
            .unwrap()
            .0,
    )
}

fn config(addr: &str) -> ClientConfig {
    let mut c = ClientConfig::new(addr, "CLIENT");
    c.backoff = cottak_net::Backoff {
        min: Duration::from_millis(20),
        max: Duration::from_millis(60),
    };
    c
}

async fn next(events: &mut ClientEvents) -> ClientEvent {
    timeout(WAIT, events.recv())
        .await
        .expect("no client event")
        .expect("client stopped")
}

/// Skips events until `pick` accepts one.
async fn until<T>(events: &mut ClientEvents, mut pick: impl FnMut(ClientEvent) -> Option<T>) -> T {
    loop {
        if let Some(v) = pick(next(events).await) {
            return v;
        }
    }
}

#[tokio::test]
async fn xml_exchange() {
    let (l, addr) = listener().await;
    let (handle, mut events) =
        TakClient::start(config(&addr).with_protocol(ProtocolPreference::XmlOnly)).unwrap();
    let mut server = accept(&l).await;
    assert_eq!(
        next(&mut events).await,
        ClientEvent::Connected {
            protocol: Protocol::Xml
        }
    );

    handle.send(ev("FROM-CLIENT")).await.unwrap();
    let (raw, got) = server.read_xml().await;
    assert_eq!(got.uid, "FROM-CLIENT");
    assert!(
        raw.starts_with("<?xml version=\"1.0\""),
        "TAK Server requires the XML header"
    );

    server.write_xml(&ev("FROM-SERVER")).await;
    let got = until(&mut events, |e| match e {
        ClientEvent::Event(ev) => Some(*ev),
        _ => None,
    })
    .await;
    assert_eq!(got.uid, "FROM-SERVER");
    handle.shutdown().await;
}

#[tokio::test]
async fn upgrades_to_protobuf_and_pauses_sending_during_negotiation() {
    let (l, addr) = listener().await;
    let (handle, mut events) = TakClient::start(config(&addr)).unwrap();
    let mut server = accept(&l).await;
    server
        .write_xml(&negotiate::offer("neg-1", &[1]).unwrap())
        .await;

    assert!(matches!(
        next(&mut events).await,
        ClientEvent::Connected { .. }
    ));
    let (_, request) = server.read_xml().await;
    assert_eq!(negotiate::parse_request(&request), Some(1));
    assert_eq!(
        request.uid, "neg-1",
        "client must reuse the server's negotiation uid"
    );

    // The request is out, so the client is now paused: this event is queued during
    // negotiation and must arrive as protobuf, never as XML.
    handle.send(ev("QUEUED")).await.unwrap();

    // Response and the first protobuf frame arrive back to back.
    let mut wire = BytesMut::new();
    wire.extend_from_slice(
        negotiate::response("neg-1", true)
            .unwrap()
            .to_xml_string()
            .unwrap()
            .as_bytes(),
    );
    TakFrame::from_event(ev("PB-FROM-SERVER"))
        .encode_stream(&mut wire)
        .unwrap();
    server.io.write_all(&wire).await.unwrap();

    assert_eq!(server.read_frame().await.uid, "QUEUED");
    until(&mut events, |e| {
        matches!(e, ClientEvent::ProtocolChanged(Protocol::V1)).then_some(())
    })
    .await;
    let got = until(&mut events, |e| match e {
        ClientEvent::Event(ev) => Some(*ev),
        _ => None,
    })
    .await;
    assert_eq!(got.uid, "PB-FROM-SERVER");
    handle.shutdown().await;
}

#[tokio::test]
async fn declined_upgrade_stays_on_xml() {
    let (l, addr) = listener().await;
    let (handle, events) = TakClient::start(config(&addr)).unwrap();
    let mut server = accept(&l).await;
    server
        .write_xml(&negotiate::offer("neg-2", &[1]).unwrap())
        .await;
    let (_, request) = server.read_xml().await;
    assert_eq!(negotiate::parse_request(&request), Some(1));
    server
        .write_xml(&negotiate::response("neg-2", false).unwrap())
        .await;

    handle.send(ev("STILL-XML")).await.unwrap();
    assert_eq!(server.read_xml().await.1.uid, "STILL-XML");
    handle.shutdown().await;
    drop(events);
}

#[tokio::test]
async fn xml_only_ignores_offers() {
    let (l, addr) = listener().await;
    let (handle, _events) =
        TakClient::start(config(&addr).with_protocol(ProtocolPreference::XmlOnly)).unwrap();
    let mut server = accept(&l).await;
    server
        .write_xml(&negotiate::offer("neg-3", &[1]).unwrap())
        .await;
    handle.send(ev("PLAIN")).await.unwrap();
    assert_eq!(
        server.read_xml().await.1.uid,
        "PLAIN",
        "no upgrade request may be sent"
    );
    handle.shutdown().await;
}

#[tokio::test]
async fn keepalive_pings_are_sent_and_pongs_are_swallowed() {
    let (l, addr) = listener().await;
    let (handle, mut events) = TakClient::start(
        config(&addr)
            .with_protocol(ProtocolPreference::XmlOnly)
            .with_keepalive(Duration::from_millis(100)),
    )
    .unwrap();
    let mut server = accept(&l).await;
    for _ in 0..3 {
        let (_, ping) = server.read_xml().await;
        assert!(control::is_ping(&ping));
        assert_eq!(ping.uid, "CLIENT");
        server.write_xml(&control::pong("server").unwrap()).await;
    }
    // Only Connected was reported; pongs never reach the application.
    assert!(matches!(
        next(&mut events).await,
        ClientEvent::Connected { .. }
    ));
    assert!(timeout(Duration::from_millis(250), events.recv())
        .await
        .is_err());
    handle.shutdown().await;
}

#[tokio::test]
async fn silent_server_is_detected_and_reconnected() {
    let (l, addr) = listener().await;
    let mut cfg = config(&addr).with_protocol(ProtocolPreference::XmlOnly);
    cfg.ping_interval = Duration::from_millis(50);
    cfg.read_timeout = Duration::from_millis(200);
    let (handle, mut events) = TakClient::start(cfg).unwrap();

    let _silent = accept(&l).await; // accepts, then says nothing
    assert!(matches!(
        next(&mut events).await,
        ClientEvent::Connected { .. }
    ));
    let reason = until(&mut events, |e| match e {
        ClientEvent::Disconnected { reason, .. } => Some(reason),
        _ => None,
    })
    .await;
    assert!(reason.contains("read timeout"), "{reason}");

    let _again = accept(&l).await;
    assert!(matches!(
        next(&mut events).await,
        ClientEvent::Connected { .. }
    ));
    handle.shutdown().await;
}

#[tokio::test]
async fn reconnects_and_delivers_events_queued_while_down() {
    let (l, addr) = listener().await;
    let (handle, mut events) =
        TakClient::start(config(&addr).with_protocol(ProtocolPreference::XmlOnly)).unwrap();

    drop(accept(&l).await); // server hangs up immediately
    until(&mut events, |e| {
        matches!(e, ClientEvent::Disconnected { .. }).then_some(())
    })
    .await;
    handle.send(ev("QUEUED-WHILE-DOWN")).await.unwrap();

    let mut server = accept(&l).await;
    assert_eq!(server.read_xml().await.1.uid, "QUEUED-WHILE-DOWN");
    handle.shutdown().await;
}

#[tokio::test]
async fn connection_refused_is_retried_with_backoff() {
    // Nothing listens on port 1 (tcpmux). Do not bind-and-drop an ephemeral port instead: tests
    // run in parallel, and another test's listener can be given the freed port, so this client
    // would connect to it, steal its accept(), and make *that* test time out.
    let addr = "127.0.0.1:1";
    let (handle, mut events) = TakClient::start(config(addr)).unwrap();
    let mut failures = 0;
    while failures < 3 {
        if let ClientEvent::Disconnected { retry_in, .. } = next(&mut events).await {
            assert!(retry_in <= Duration::from_millis(60));
            failures += 1;
        }
    }
    handle.shutdown().await;
}

#[tokio::test]
async fn stale_events_are_dropped_not_sent() {
    let (l, addr) = listener().await;
    let (handle, _events) =
        TakClient::start(config(&addr).with_protocol(ProtocolPreference::XmlOnly)).unwrap();
    let mut server = accept(&l).await;

    let mut old = ev("OLD");
    old.start -= time::Duration::minutes(10);
    old.time = old.start;
    old.stale = old.start + time::Duration::seconds(30);
    handle.send(old).await.unwrap();
    handle.send(ev("FRESH")).await.unwrap();
    assert_eq!(server.read_xml().await.1.uid, "FRESH");
    handle.shutdown().await;
}

#[tokio::test]
async fn dropping_the_event_receiver_stops_the_client() {
    let (l, addr) = listener().await;
    let (handle, events) = TakClient::start(config(&addr)).unwrap();
    let _server = accept(&l).await;
    drop(events);
    // Once the client notices, sending fails.
    let mut stopped = false;
    for _ in 0..100 {
        if handle.send(ev("X")).await.is_err() {
            stopped = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(stopped, "client kept running without a receiver");
}

#[tokio::test]
async fn invalid_addresses_and_tls_are_rejected_up_front() {
    assert!(TakClient::start(ClientConfig::new("no-port", "u")).is_err());
    let tls = cottak_net::TlsConfig::new(); // no CA: refuses
    assert!(TakClient::start(ClientConfig::new("localhost:8089", "u").with_tls(tls)).is_err());
}
