//! The Marti mock, exercised by the real `MartiClient` and by raw HTTP.
#![allow(
    clippy::too_many_lines,
    clippy::many_single_char_names,
    clippy::assert_is_empty
)]

use cottak_core::detail::{Contact, Group, Marti, MartiDest};
use cottak_core::{Event, Point};
use cottak_marti::{
    ChangeQuery, ContentRef, EndpointQuery, Error as MartiError, MartiClient, MartiConfig,
    MissionContent, MissionQuery, NewMission, UploadOptions,
};
use cottak_net::{
    Backoff, ClientConfig, ClientEvent, ClientEvents, ClientHandle, ProtocolPreference, TakClient,
    TlsConfig,
};
use cottak_server::{BasicAuth, Server, ServerConfig, ServerTlsConfig};
use rcgen::{
    BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};

const WAIT: Duration = Duration::from_secs(10);

// ---- PKI ----

struct Ca {
    pem: String,
    params: CertificateParams,
    key: KeyPair,
}

struct Identity {
    cert_pem: String,
    key_pem: String,
}

fn new_ca() -> Ca {
    let key = KeyPair::generate().unwrap();
    let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params
        .distinguished_name
        .push(DnType::CommonName, "Test CA");
    params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
    let pem = params.self_signed(&key).unwrap().pem();
    Ca { pem, params, key }
}

fn issue(ca: &Ca, name: &str, usage: ExtendedKeyUsagePurpose) -> Identity {
    let key = KeyPair::generate().unwrap();
    let mut params = CertificateParams::new(vec![name.to_owned()]).unwrap();
    params.distinguished_name.push(DnType::CommonName, name);
    params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    params.extended_key_usages = vec![usage];
    let cert = params
        .signed_by(&key, &Issuer::from_params(&ca.params, &ca.key))
        .unwrap();
    Identity {
        cert_pem: cert.pem(),
        key_pem: key.serialize_pem(),
    }
}

// ---- fixtures ----

struct Fixture {
    server: Server,
    ca: Ca,
    tak: SocketAddr,
    marti: SocketAddr,
}

async fn fixture_with(configure: impl FnOnce(ServerConfig) -> ServerConfig) -> Fixture {
    let ca = new_ca();
    let id = issue(&ca, "localhost", ExtendedKeyUsagePurpose::ServerAuth);
    let tls = ServerTlsConfig::mutual(id.cert_pem, id.key_pem, ca.pem.clone());
    let config = configure(
        ServerConfig::new()
            .listen_tcp("127.0.0.1:0")
            .listen_marti("127.0.0.1:0", Some(tls)),
    );
    let server = Server::start(config).await.unwrap();
    let (tak, marti) = (server.local_addrs()[0], server.local_addrs()[1]);
    Fixture {
        server,
        ca,
        tak,
        marti,
    }
}

async fn fixture() -> Fixture {
    fixture_with(|c| c).await
}

impl Fixture {
    async fn marti_client(&self, basic: Option<(&str, &str)>) -> MartiClient {
        let id = issue(&self.ca, "alice", ExtendedKeyUsagePurpose::ClientAuth);
        let tls = TlsConfig::new()
            .with_ca_pem(&self.ca.pem)
            .with_client_identity_pem(id.cert_pem, id.key_pem)
            .with_server_name("localhost");
        let mut cfg = MartiConfig::new("127.0.0.1", tls);
        cfg.port = self.marti.port();
        if let Some((u, p)) = basic {
            cfg = cfg.with_basic_auth(u, p);
        }
        MartiClient::new(cfg).await.unwrap()
    }

    fn tak_client(&self, uid: &str) -> (ClientHandle, ClientEvents) {
        let mut c =
            ClientConfig::new(self.tak.to_string(), uid).with_protocol(ProtocolPreference::XmlOnly);
        c.backoff = Backoff {
            min: Duration::from_millis(50),
            max: Duration::from_millis(100),
        };
        TakClient::start(c).unwrap()
    }
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

// ---- tests ----

#[tokio::test]
async fn server_and_people_endpoints_reflect_connected_clients() {
    let f = fixture().await;
    let marti = f.marti_client(None).await;
    assert!(marti.version().await.unwrap().starts_with("cottak-server "));
    assert_eq!(marti.node_id().await.unwrap(), "cottak-server");
    assert!(marti.version_info().await.unwrap()["data"]["apiVersion"].is_string());

    let groups = marti.groups().await.unwrap();
    assert!(groups.iter().all(|g| g.name == "__ANON__") && groups.len() == 2);

    assert!(
        marti.contacts().await.unwrap().is_empty(),
        "nobody has reported in yet"
    );
    let (a, _events) = f.tak_client("ALPHA-UID");
    a.send(pli("ALPHA-UID", "ALPHA", 1.0).with(Group {
        name: Some("Cyan".into()),
        role: Some("Team Lead".into()),
        ..Group::default()
    }))
    .await
    .unwrap();
    wait_until("contact listed", || {
        !f.server.clients().iter().all(|c| c.uid.is_none())
    })
    .await;

    let contacts = marti.contacts().await.unwrap();
    assert_eq!(contacts.len(), 1);
    assert_eq!(contacts[0].callsign.as_deref(), Some("ALPHA"));
    assert_eq!(contacts[0].team.as_deref(), Some("Cyan"));
    assert_eq!(contacts[0].role.as_deref(), Some("Team Lead"));

    let endpoints = marti
        .client_endpoints(&EndpointQuery::default())
        .await
        .unwrap();
    assert_eq!(endpoints.len(), 1);
    assert_eq!(endpoints[0].uid.as_deref(), Some("ALPHA-UID"));
    assert_eq!(endpoints[0].last_status.as_deref(), Some("Connected"));
    assert!(endpoints[0].last_event_time.is_some());

    a.shutdown().await;
    // An idle keep-alive connection from the client must not block shutdown.
    timeout(WAIT, f.server.shutdown())
        .await
        .expect("shutdown must not hang");
}

#[tokio::test]
async fn mission_lifecycle_with_files_and_change_log() {
    let f = fixture().await;
    let marti = f.marti_client(None).await;

    let created = marti
        .create_mission(
            "RECON 1/a",
            "OWNER",
            &NewMission {
                description: Some("recon".into()),
                ..NewMission::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(
        created.name, "RECON 1/a",
        "names with spaces and slashes survive the URL"
    );
    assert_eq!(created.creator_uid.as_deref(), Some("OWNER"));
    assert_eq!(created.tool.as_deref(), Some("public"));
    assert_eq!(created.description.as_deref(), Some("recon"));
    assert!(created.guid.is_some() && created.create_time.is_some());
    assert!(matches!(
        marti
            .create_mission("RECON 1/a", "OWNER", &NewMission::default())
            .await,
        Err(MartiError::Conflict)
    ));

    let list = marti.missions(&MissionQuery::default()).await.unwrap();
    assert_eq!(
        list.iter().map(|m| m.name.as_str()).collect::<Vec<_>>(),
        ["RECON 1/a"]
    );
    assert!(
        list[0].contents.is_empty(),
        "the list endpoint omits contents"
    );

    let file = marti
        .upload_file(
            "plan.zip",
            b"PK-data",
            &UploadOptions {
                mime: Some("application/zip".into()),
                keywords: vec!["plan".into()],
                creator_uid: Some("OWNER".into()),
                ..UploadOptions::default()
            },
        )
        .await
        .unwrap();
    let hash = file["Hash"].as_str().unwrap().to_owned();
    assert_eq!(hash.len(), 64);
    assert_eq!(file["Name"], "plan.zip");
    assert_eq!(file["Size"], 7);

    let mission = marti
        .add_content(
            "RECON 1/a",
            &MissionContent {
                hashes: vec![hash.clone()],
                uids: vec!["UID-1".into()],
            },
            "OWNER",
        )
        .await
        .unwrap();
    assert_eq!(mission.contents.len(), 1);
    assert_eq!(
        mission.contents[0].data.hash.as_deref(),
        Some(hash.as_str())
    );
    assert_eq!(mission.contents[0].data.size, Some(7));
    assert_eq!(mission.contents[0].data.keywords, ["plan"]);
    assert_eq!(mission.uids[0].data, "UID-1");

    let fetched = marti.mission("RECON 1/a", None).await.unwrap();
    assert_eq!((fetched.contents.len(), fetched.uids.len()), (1, 1));

    let changes = marti
        .changes("RECON 1/a", &ChangeQuery::default())
        .await
        .unwrap();
    let kinds: Vec<_> = changes
        .iter()
        .filter_map(|c| c.change_type.as_deref())
        .collect();
    assert_eq!(kinds, ["CREATE_MISSION", "ADD_CONTENT", "ADD_CONTENT"]);
    assert_eq!(changes[1].content_hash.as_deref(), Some(hash.as_str()));
    assert!(changes[1].content_resource.is_some());
    let none = marti
        .changes(
            "RECON 1/a",
            &ChangeQuery {
                seconds_ago: Some(0),
                ..ChangeQuery::default()
            },
        )
        .await
        .unwrap();
    assert!(none.len() <= changes.len());

    marti.subscribe("RECON 1/a", "SUB-1", None).await.unwrap();
    assert_eq!(marti.subscribers("RECON 1/a").await.unwrap(), ["SUB-1"]);
    marti
        .unsubscribe("RECON 1/a", "SUB-1", false)
        .await
        .unwrap();
    assert!(marti.subscribers("RECON 1/a").await.unwrap().is_empty());

    let m = marti
        .remove_content("RECON 1/a", &ContentRef::Uid("UID-1".into()), "OWNER")
        .await
        .unwrap();
    assert!(m.uids.is_empty() && m.contents.len() == 1);
    let m = marti
        .remove_content("RECON 1/a", &ContentRef::Hash(hash.clone()), "OWNER")
        .await
        .unwrap();
    assert!(m.contents.is_empty());

    marti
        .delete_mission("RECON 1/a", "OWNER", true)
        .await
        .unwrap();
    assert!(matches!(
        marti.mission("RECON 1/a", None).await,
        Err(MartiError::NotFound)
    ));
    // The file had been detached before the deep delete, so it is still downloadable.
    assert_eq!(marti.download(&hash).await.unwrap(), b"PK-data");
    f.server.shutdown().await;
}

#[tokio::test]
async fn files_round_trip_byte_for_byte() {
    let f = fixture().await;
    let marti = f.marti_client(None).await;

    // Binary data that contains multipart-looking text and every byte value.
    let mut data: Vec<u8> = (0..=255).collect();
    data.extend_from_slice(
        b"\r\n------cottak\r\nContent-Disposition: form-data; name=\"x\"\r\n\r\n",
    );
    data.extend_from_slice(&vec![0xAB; 100_000]);

    let meta = marti
        .upload_file("blob.bin", &data, &UploadOptions::default())
        .await
        .unwrap();
    let hash = meta["Hash"].as_str().unwrap();
    assert_eq!(marti.download(hash).await.unwrap(), data);

    // Identical content is stored once and has one hash.
    let again = marti
        .upload_file("copy.bin", &data, &UploadOptions::default())
        .await
        .unwrap();
    assert_eq!(again["Hash"], meta["Hash"]);

    let package = marti
        .upload_data_package("pkg.zip", "OWNER", b"PK\x03\x04", &UploadOptions::default())
        .await
        .unwrap();
    assert!(
        package.url.starts_with("https://localhost:"),
        "{}",
        package.url
    );
    assert_eq!(
        marti
            .download(package.hash.as_deref().unwrap())
            .await
            .unwrap(),
        b"PK\x03\x04"
    );

    assert!(matches!(
        marti.download(&"0".repeat(64)).await,
        Err(MartiError::NotFound)
    ));
    f.server.shutdown().await;
}

#[tokio::test]
async fn access_rules_and_error_statuses() {
    let f = fixture().await;
    let marti = f.marti_client(None).await;
    marti
        .create_mission("m", "OWNER", &NewMission::default())
        .await
        .unwrap();

    // Only the creator may delete; strangers may not add content.
    assert!(matches!(
        marti.delete_mission("m", "STRANGER", false).await,
        Err(MartiError::Forbidden)
    ));
    let uids = MissionContent {
        hashes: vec![],
        uids: vec!["U".into()],
    };
    assert!(matches!(
        marti.add_content("m", &uids, "STRANGER").await,
        Err(MartiError::Forbidden)
    ));
    marti.add_content("m", &uids, "OWNER").await.unwrap();

    // Unknown things are 404, unknown hashes cannot be attached.
    assert!(matches!(
        marti.mission("nope", None).await,
        Err(MartiError::NotFound)
    ));
    assert!(matches!(
        marti.add_content("nope", &uids, "OWNER").await,
        Err(MartiError::NotFound)
    ));
    let bad = MissionContent {
        hashes: vec!["f".repeat(64)],
        uids: vec![],
    };
    assert!(matches!(
        marti.add_content("m", &bad, "OWNER").await,
        Err(MartiError::NotFound)
    ));
    assert!(matches!(
        marti.delete_mission("nope", "OWNER", false).await,
        Err(MartiError::NotFound)
    ));

    // Subscribed writers may add content, read-only subscribers may not.
    marti.subscribe("m", "WRITER", None).await.unwrap();
    marti
        .add_content(
            "m",
            &MissionContent {
                hashes: vec![],
                uids: vec!["W".into()],
            },
            "WRITER",
        )
        .await
        .unwrap();
    marti
        .create_mission(
            "ro",
            "OWNER",
            &NewMission {
                default_role: Some(cottak_marti::MissionRoleKind::ReadOnlySubscriber),
                ..NewMission::default()
            },
        )
        .await
        .unwrap();
    marti.subscribe("ro", "READER", None).await.unwrap();
    assert!(matches!(
        marti.add_content("ro", &uids, "READER").await,
        Err(MartiError::Forbidden)
    ));

    // Password-protected missions hide from the default list and gate reading and joining.
    marti
        .create_mission(
            "secret",
            "OWNER",
            &NewMission {
                password: Some("pw".into()),
                ..NewMission::default()
            },
        )
        .await
        .unwrap();
    let names = |l: Vec<cottak_marti::Mission>| l.into_iter().map(|m| m.name).collect::<Vec<_>>();
    assert!(
        !names(marti.missions(&MissionQuery::default()).await.unwrap())
            .contains(&"secret".to_owned())
    );
    let with = MissionQuery {
        password_protected: true,
        ..MissionQuery::default()
    };
    assert!(names(marti.missions(&with).await.unwrap()).contains(&"secret".to_owned()));
    assert!(matches!(
        marti.mission("secret", None).await,
        Err(MartiError::Forbidden)
    ));
    assert!(matches!(
        marti.mission("secret", Some("wrong")).await,
        Err(MartiError::Forbidden)
    ));
    assert!(matches!(
        marti.subscribe("secret", "U", None).await,
        Err(MartiError::Forbidden)
    ));
    let m = marti.mission("secret", Some("pw")).await.unwrap();
    assert_eq!(m.password_protected, Some(true));
    assert!(
        m.extra.get("password").is_none(),
        "the password is never sent back"
    );
    marti.subscribe("secret", "U", Some("pw")).await.unwrap();

    f.server.shutdown().await;
}

#[tokio::test]
async fn mission_events_reach_subscribers_and_the_mission_keeps_the_latest() {
    let f = fixture().await;
    let marti = f.marti_client(None).await;
    let (a, _a_events) = f.tak_client("A");
    let (b, mut b_events) = f.tak_client("B");
    let (c, mut c_events) = f.tak_client("C");
    for (h, uid, call) in [(&a, "A", "ALPHA"), (&b, "B", "BRAVO"), (&c, "C", "CHARLIE")] {
        h.send(pli(uid, call, 0.0)).await.unwrap();
    }
    wait_until("all identified", || {
        f.server
            .clients()
            .iter()
            .filter(|c| c.uid.is_some())
            .count()
            == 3
    })
    .await;

    marti
        .create_mission("ops", "A", &NewMission::default())
        .await
        .unwrap();
    marti.subscribe("ops", "B", None).await.unwrap();

    let to_mission = |uid: &str, lat: f64| {
        pli(uid, "ALPHA", lat).with(Marti {
            dests: vec![MartiDest {
                mission: Some("ops".into()),
                ..MartiDest::default()
            }],
        })
    };
    a.send(to_mission("TRACK-1", 1.0)).await.unwrap();
    a.send(pli("SENTINEL", "ALPHA", 0.0)).await.unwrap();

    assert!(
        recv_until(&mut b_events, "SENTINEL")
            .await
            .contains(&"TRACK-1".to_owned()),
        "subscriber B must receive the mission event"
    );
    let seen_by_c = recv_until(&mut c_events, "SENTINEL").await;
    assert!(
        !seen_by_c.contains(&"TRACK-1".to_owned()),
        "C is not subscribed: {seen_by_c:?}"
    );

    // The mission now lists the UID and holds its latest state, which follows later updates.
    let m = marti.mission("ops", None).await.unwrap();
    assert_eq!(
        m.uids.iter().map(|u| u.data.as_str()).collect::<Vec<_>>(),
        ["TRACK-1"]
    );
    assert_eq!(m.uids[0].creator_uid.as_deref(), Some("A"));
    let cot = marti.mission_cot("ops").await.unwrap();
    assert_eq!((cot.len(), cot[0].uid.as_str()), (1, "TRACK-1"));
    assert!((cot[0].point.lat - 1.0).abs() < 1e-9);

    a.send(pli("TRACK-1", "ALPHA", 5.0)).await.unwrap();
    wait_until("mission copy follows the entity", || {
        f.server
            .stored_events()
            .iter()
            .any(|e| e.uid == "TRACK-1" && (e.point.lat - 5.0).abs() < 1e-9)
    })
    .await;
    let cot = marti.mission_cot("ops").await.unwrap();
    assert!(
        (cot[0].point.lat - 5.0).abs() < 1e-9,
        "latest position is kept"
    );

    // An event for a mission that does not exist reaches nobody and creates nothing.
    a.send(pli("LOST", "ALPHA", 0.0).with(Marti {
        dests: vec![MartiDest {
            mission: Some("ghost".into()),
            ..MartiDest::default()
        }],
    }))
    .await
    .unwrap();
    a.send(pli("SENTINEL-2", "ALPHA", 0.0)).await.unwrap();
    assert!(!recv_until(&mut b_events, "SENTINEL-2")
        .await
        .contains(&"LOST".to_owned()));

    for h in [&a, &b, &c] {
        h.shutdown().await;
    }
    f.server.shutdown().await;
}

#[tokio::test]
async fn basic_auth_can_be_required() {
    let f = fixture_with(|c| c.with_marti_basic_auth(BasicAuth::new("alice", "s3cret"))).await;
    let anonymous = f.marti_client(None).await;
    assert!(matches!(
        anonymous.version().await,
        Err(MartiError::Unauthorized)
    ));
    let wrong = f.marti_client(Some(("alice", "nope"))).await;
    assert!(matches!(
        wrong.version().await,
        Err(MartiError::Unauthorized)
    ));
    let wrong_user = f.marti_client(Some(("mallory", "s3cret"))).await;
    assert!(matches!(
        wrong_user.version().await,
        Err(MartiError::Unauthorized)
    ));
    let ok = f.marti_client(Some(("alice", "s3cret"))).await;
    assert!(ok.version().await.is_ok());

    // The authenticated user is recorded as the submitter.
    let meta = ok
        .upload_file("f.txt", b"x", &UploadOptions::default())
        .await
        .unwrap();
    assert_eq!(meta["SubmissionUser"], "alice");
    assert!(!format!(
        "{:?}",
        ServerConfig::new().with_marti_basic_auth(BasicAuth::new("u", "SECRET-PW"))
    )
    .contains("SECRET-PW"));
    f.server.shutdown().await;
}

#[tokio::test]
async fn storage_limits_are_enforced() {
    let f = fixture_with(|mut c| {
        c.max_files = 1;
        c.max_missions = 1;
        c.marti_max_body = 4096;
        c
    })
    .await;
    let marti = f.marti_client(None).await;
    marti
        .upload_file("a", b"one", &UploadOptions::default())
        .await
        .unwrap();
    assert!(matches!(
        marti
            .upload_file("b", b"two", &UploadOptions::default())
            .await,
        Err(MartiError::Status { status: 507, .. })
    ));
    marti
        .create_mission("m1", "O", &NewMission::default())
        .await
        .unwrap();
    assert!(matches!(
        marti
            .create_mission("m2", "O", &NewMission::default())
            .await,
        Err(MartiError::Status { status: 507, .. })
    ));
    f.server.shutdown().await;
}

// ---- raw HTTP over a plain listener ----

async fn plain() -> (Server, SocketAddr) {
    let server = Server::start(
        ServerConfig::new()
            .listen_marti("127.0.0.1:0", None)
            .tap_config(|c| {
                c.marti_max_body = 1024;
                c.marti_request_timeout = Duration::from_millis(500);
            }),
    )
    .await
    .unwrap();
    let addr = server.local_addrs()[0];
    (server, addr)
}

trait Tap: Sized {
    fn tap_config(self, f: impl FnOnce(&mut Self)) -> Self;
}

impl Tap for ServerConfig {
    fn tap_config(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }
}

/// Sends `raw` and returns everything the server answers until it closes the connection.
async fn exchange(addr: SocketAddr, raw: &[u8]) -> String {
    let mut s = TcpStream::connect(addr).await.unwrap();
    s.write_all(raw).await.unwrap();
    let mut out = Vec::new();
    timeout(WAIT, s.read_to_end(&mut out))
        .await
        .expect("server must close")
        .ok();
    String::from_utf8_lossy(&out).into_owned()
}

#[tokio::test]
async fn raw_http_framing_is_strict() {
    let (server, addr) = plain().await;

    let ok = exchange(
        addr,
        b"GET /Marti/api/node/id HTTP/1.1\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(
        ok.starts_with("HTTP/1.1 200 OK\r\n") && ok.ends_with("cottak-server"),
        "{ok}"
    );

    for (raw, status) in [
        (&b"GET /nowhere HTTP/1.1\r\nConnection: close\r\n\r\n"[..], "404"),
        (b"POST /Marti/api/version HTTP/1.1\r\nConnection: close\r\nContent-Length: 0\r\n\r\n", "405"),
        (b"POST /Marti/sync/upload HTTP/1.1\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n\r\n", "501"),
        (b"POST /Marti/sync/upload HTTP/1.1\r\nContent-Length: 5000\r\n\r\n", "413"),
        (b"POST /Marti/sync/upload HTTP/1.1\r\nContent-Length: 1\r\nContent-Length: 2\r\n\r\nab", "400"),
        (b"GET /x HTTP/1.1\r\nBad Header\r\n\r\n", "400"),
        (b"GARBAGE\r\n\r\n", "400"),
        (b"POST /Marti/sync/upload HTTP/1.1\r\nContent-Type: text/plain\r\nContent-Length: 2\r\nConnection: close\r\n\r\nhi", "400"),
    ] {
        let got = exchange(addr, raw).await;
        assert!(got.starts_with(&format!("HTTP/1.1 {status} ")), "want {status}: {got}");
    }
    server.shutdown().await;
}

#[tokio::test]
async fn keep_alive_pipelining_and_idle_timeout() {
    let (server, addr) = plain().await;
    let mut s = TcpStream::connect(addr).await.unwrap();
    // Two requests in one write; the second one asks to close.
    s.write_all(
        b"GET /Marti/api/node/id HTTP/1.1\r\n\r\nGET /Marti/api/version HTTP/1.1\r\nConnection: close\r\n\r\n",
    )
    .await
    .unwrap();
    let mut out = Vec::new();
    timeout(WAIT, s.read_to_end(&mut out))
        .await
        .unwrap()
        .unwrap();
    let text = String::from_utf8_lossy(&out);
    assert_eq!(text.matches("HTTP/1.1 200 OK").count(), 2, "{text}");
    assert!(text.contains("Connection: keep-alive") && text.contains("Connection: close"));

    // A connection that sends nothing is dropped after the request timeout.
    let mut idle = TcpStream::connect(addr).await.unwrap();
    let mut buf = [0u8; 1];
    let n = timeout(WAIT, idle.read(&mut buf))
        .await
        .expect("must hang up")
        .unwrap();
    assert_eq!(n, 0);
    server.shutdown().await;
}

#[tokio::test]
async fn hostile_paths_and_parameters_are_harmless() {
    let (server, addr) = plain().await;
    for target in [
        "/Marti/api/missions/%00",
        "/Marti/api/missions/..%2f..%2fetc%2fpasswd",
        "/Marti/api/missions/%ff",
        "/Marti/sync/content?hash=../../etc/passwd",
        "/Marti/api/missions?tool=%00&passwordProtected=%ff",
        &format!("/Marti/api/missions/{}", "a".repeat(5000)),
    ] {
        let got = exchange(
            addr,
            format!("GET {target} HTTP/1.1\r\nConnection: close\r\n\r\n").as_bytes(),
        )
        .await;
        let status: u16 = got
            .split(' ')
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        assert!([200, 400, 404].contains(&status), "{target}: {got}");
    }
    // Control characters in a mission name or UID are refused outright.
    let put = exchange(
        addr,
        b"PUT /Marti/api/missions/a%0Ab?creatorUid=x HTTP/1.1\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(put.starts_with("HTTP/1.1 400 "), "{put}");
    server.shutdown().await;
}
