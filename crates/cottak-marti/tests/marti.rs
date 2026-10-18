//! `MartiClient` against an in-process mock of TAK Server's HTTPS API (mutual TLS required).
#![allow(
    clippy::too_many_lines,
    clippy::struct_field_names,
    clippy::unreadable_literal,
    clippy::assert_is_empty
)]

use cottak_marti::{
    ChangeQuery, ContentRef, EndpointQuery, Error, MartiClient, MartiConfig, MissionContent,
    MissionQuery, MissionRoleKind, NewMission, UploadOptions,
};
use cottak_net::TlsConfig;
use rcgen::{
    BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::WebPkiClientVerifier;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

// ---------------------------------------------------------------------------------------------
// Test PKI
// ---------------------------------------------------------------------------------------------

struct Pki {
    ca_pem: String,
    ca_der: Vec<u8>,
    ca_params: CertificateParams,
    ca_key: KeyPair,
}

fn new_pki(name: &str) -> Pki {
    let key = KeyPair::generate().unwrap();
    let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.distinguished_name.push(DnType::CommonName, name);
    params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
    let cert = params.self_signed(&key).unwrap();
    Pki {
        ca_pem: cert.pem(),
        ca_der: cert.der().to_vec(),
        ca_params: params,
        ca_key: key,
    }
}

fn issue(pki: &Pki, name: &str, usage: ExtendedKeyUsagePurpose) -> (String, String, Vec<u8>) {
    let key = KeyPair::generate().unwrap();
    let mut params = CertificateParams::new(vec![name.to_owned()]).unwrap();
    params.distinguished_name.push(DnType::CommonName, name);
    params.extended_key_usages = vec![usage];
    let cert = params
        .signed_by(&key, &Issuer::from_params(&pki.ca_params, &pki.ca_key))
        .unwrap();
    (cert.pem(), key.serialize_pem(), cert.der().to_vec())
}

// ---------------------------------------------------------------------------------------------
// Mock server
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct Req {
    method: String,
    /// Path segments, percent-decoded.
    segments: Vec<String>,
    raw_path: String,
    query: Vec<(String, String)>,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Req {
    fn q(&self, key: &str) -> Option<&str> {
        self.query
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
    fn all(&self, key: &str) -> Vec<&str> {
        self.query
            .iter()
            .filter(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .collect()
    }
}

struct Resp {
    status: u16,
    content_type: &'static str,
    body: Vec<u8>,
}

fn json_resp(status: u16, v: &Value) -> Resp {
    Resp {
        status,
        content_type: "application/json",
        body: v.to_string().into_bytes(),
    }
}

fn envelope(kind: &str, data: Value) -> Value {
    json!({"version": "3", "type": kind, "data": data, "nodeId": "node-1"})
}

#[derive(Default)]
struct State {
    missions: BTreeMap<String, Value>,
    files: HashMap<String, (String, Vec<u8>)>,
    subscribers: HashMap<String, Vec<String>>,
    seen: Vec<Req>,
}

struct Server {
    port: u16,
    state: Arc<Mutex<State>>,
}

fn pct_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

async fn read_req<S: AsyncReadExt + Unpin>(s: &mut S) -> Option<Req> {
    let mut buf = Vec::new();
    let head_end = loop {
        let mut chunk = [0u8; 4096];
        let n = s.read(&mut chunk).await.ok()?;
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
        let mut chunk = [0u8; 8192];
        let n = s.read(&mut chunk).await.ok()?;
        if n == 0 {
            return None;
        }
        body.extend_from_slice(&chunk[..n]);
    }
    let (path, query) = target.split_once('?').unwrap_or((target.as_str(), ""));
    Some(Req {
        method,
        segments: path
            .trim_start_matches('/')
            .split('/')
            .map(pct_decode)
            .collect(),
        raw_path: path.to_owned(),
        query: url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect(),
        headers,
        body,
    })
}

fn mission_json(name: &str, req: &Req) -> Value {
    json!({
        "name": name,
        "description": req.q("description"),
        "creatorUid": req.q("creatorUid"),
        "groups": if req.all("group").is_empty() { vec!["__ANON__"] } else { req.all("group") },
        "tool": req.q("tool").unwrap_or("public"),
        "keywords": [], "uids": [], "contents": [],
        "passwordProtected": req.q("password").is_some(),
        "defaultRole": req.q("defaultRole").map(|r| json!({"type": r})),
        "guid": format!("guid-{name}"),
    })
}

/// Parses the single `assetfile` part from a multipart body.
fn parse_multipart(req: &Req) -> Option<(String, Vec<u8>)> {
    let ct = req.headers.get("content-type")?;
    let boundary = ct.split("boundary=").nth(1)?;
    let delim = format!("--{boundary}");
    let body = &req.body;
    let start = body
        .windows(delim.len())
        .position(|w| w == delim.as_bytes())?
        + delim.len();
    let head_end = start + body[start..].windows(4).position(|w| w == b"\r\n\r\n")?;
    let head = String::from_utf8_lossy(&body[start..head_end]).into_owned();
    assert!(
        head.contains("name=\"assetfile\""),
        "part must be named assetfile: {head}"
    );
    let filename = head
        .split("filename=\"")
        .nth(1)?
        .split('"')
        .next()?
        .to_owned();
    let data_start = head_end + 4;
    let close = format!("\r\n{delim}--");
    let data_end = data_start
        + body[data_start..]
            .windows(close.len())
            .position(|w| w == close.as_bytes())?;
    Some((filename, body[data_start..data_end].to_vec()))
}

fn fake_hash(data: &[u8]) -> String {
    // Not a real digest: the mock only needs a stable id.
    let sum: u64 = data.iter().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    });
    format!("{sum:016x}{:048x}", data.len())
}

fn handle(state: &Arc<Mutex<State>>, req: &Req) -> Resp {
    let mut st = state.lock().unwrap();
    st.seen.push(req.clone());
    let seg: Vec<&str> = req.segments.iter().map(String::as_str).collect();
    let m = req.method.as_str();

    match (m, seg.as_slice()) {
        ("GET", ["Marti", "api", "version"]) => {
            Resp { status: 200, content_type: "text/plain", body: b"5.2-RELEASE-39\n".to_vec() }
        }
        ("GET", ["Marti", "api", "contacts", "all"]) => json_resp(
            200,
            &json!([{"uid": "U1", "callsign": "ALPHA", "team": "Cyan", "role": "Team Lead", "takv": "ATAK-CIV 5.2", "filterGroups": null}]),
        ),
        ("GET", ["Marti", "api", "clientEndPoints"]) => json_resp(
            200,
            &envelope(
                "ClientEndpoint",
                json!([{"callsign": "ALPHA", "uid": "U1", "username": "alice", "lastEventTime": 1738324800000_i64, "lastStatus": "Connected", "groups": "__ANON__"}]),
            ),
        ),
        ("GET", ["Marti", "api", "groups", "all"]) => json_resp(
            200,
            &envelope("Group", json!([{"name": "__ANON__", "direction": "OUT", "type": "SYSTEM", "bitpos": 0, "active": true}])),
        ),
        ("GET", ["Marti", "api", "missions"]) => {
            json_resp(200, &envelope("Mission", Value::Array(st.missions.values().cloned().collect())))
        }
        ("PUT", ["Marti", "api", "missions", name]) => {
            if st.missions.contains_key(*name) {
                return json_resp(409, &json!({"error": "exists"}));
            }
            let mission = mission_json(name, req);
            st.missions.insert((*name).to_owned(), mission.clone());
            json_resp(201, &envelope("Mission", json!([mission])))
        }
        ("GET", ["Marti", "api", "missions", "forbidden"]) => json_resp(403, &json!({})),
        ("GET", ["Marti", "api", "missions", name]) => match st.missions.get(*name) {
            Some(mission) => json_resp(200, &envelope("Mission", json!([mission]))),
            None => json_resp(404, &json!({})),
        },
        ("DELETE", ["Marti", "api", "missions", name]) => match st.missions.remove(*name) {
            Some(mission) => json_resp(200, &envelope("Mission", json!([mission]))),
            None => json_resp(404, &json!({})),
        },
        ("PUT", ["Marti", "api", "missions", name, "contents"]) => {
            assert_eq!(req.headers.get("content-type").map(String::as_str), Some("application/json"));
            let body: Value = serde_json::from_slice(&req.body).expect("body must be JSON");
            let creator = req.q("creatorUid").unwrap_or_default().to_owned();
            let Some(mission) = st.missions.get_mut(*name) else { return json_resp(404, &json!({})) };
            for uid in body["uids"].as_array().unwrap() {
                mission["uids"].as_array_mut().unwrap().push(json!({"data": uid, "creatorUid": creator}));
            }
            for hash in body["hashes"].as_array().unwrap() {
                mission["contents"].as_array_mut().unwrap().push(json!({"data": {"hash": hash}, "creatorUid": creator}));
            }
            json_resp(200, &envelope("Mission", json!([mission])))
        }
        ("DELETE", ["Marti", "api", "missions", name, "contents"]) => {
            let (hash, uid) = (req.q("hash").map(str::to_owned), req.q("uid").map(str::to_owned));
            let Some(mission) = st.missions.get_mut(*name) else { return json_resp(404, &json!({})) };
            if let Some(uid) = uid {
                mission["uids"].as_array_mut().unwrap().retain(|a| a["data"] != uid.as_str());
            }
            if let Some(hash) = hash {
                mission["contents"].as_array_mut().unwrap().retain(|a| a["data"]["hash"] != hash.as_str());
            }
            json_resp(200, &envelope("Mission", json!([mission])))
        }
        ("GET", ["Marti", "api", "missions", _, "changes"]) => json_resp(
            200,
            &envelope("MissionChange", json!([{"type": "ADD_CONTENT", "contentUid": "UID-1", "missionName": "m", "timestamp": "2025-01-31T12:00:00.000Z", "creatorUid": "U1"}])),
        ),
        ("PUT", ["Marti", "api", "missions", name, "subscription"]) => {
            let uid = req.q("uid").unwrap_or_default().to_owned();
            st.subscribers.entry((*name).to_owned()).or_default().push(uid.clone());
            json_resp(200, &envelope("MissionSubscription", json!({"missionName": name, "clientUid": uid, "role": {"type": "MISSION_SUBSCRIBER"}})))
        }
        ("DELETE", ["Marti", "api", "missions", name, "subscription"]) => {
            let uid = req.q("uid").unwrap_or_default().to_owned();
            st.subscribers.entry((*name).to_owned()).or_default().retain(|u| *u != uid);
            json_resp(200, &json!({}))
        }
        ("GET", ["Marti", "api", "missions", name, "subscriptions"]) => {
            json_resp(200, &envelope("MissionSubscription", json!(st.subscribers.get(*name).cloned().unwrap_or_default())))
        }
        ("GET", ["Marti", "api", "missions", _, "cot"]) => Resp {
            status: 200,
            content_type: "application/xml",
            body: br#"<?xml version="1.0"?><events><event version="2.0" uid="UID-1" type="a-f-G-U-C" how="m-g" time="2025-01-01T00:00:00.000Z" start="2025-01-01T00:00:00.000Z" stale="2025-01-01T00:05:00.000Z"><point lat="1" lon="2" hae="0" ce="5" le="5"/><detail><contact callsign="ALPHA"/></detail></event></events>"#.to_vec(),
        },
        ("POST", ["Marti", "sync", "upload"]) => {
            let Some((filename, data)) = parse_multipart(req) else { return json_resp(400, &json!({})) };
            let hash = fake_hash(&data);
            st.files.insert(hash.clone(), (filename.clone(), data.clone()));
            json_resp(200, &json!({"UID": hash, "Hash": hash, "Name": filename, "Size": data.len()}))
        }
        ("POST", ["Marti", "sync", "missionupload"]) => {
            let (Some(filename_q), Some(_)) = (req.q("filename"), req.q("creatorUid")) else {
                return Resp { status: 400, content_type: "text/plain", body: b"missing parameter".to_vec() };
            };
            let Some((filename, data)) = parse_multipart(req) else { return json_resp(400, &json!({})) };
            assert_eq!(filename, filename_q);
            let hash = fake_hash(&data);
            st.files.insert(hash.clone(), (filename, data));
            Resp {
                status: 200,
                content_type: "text/plain",
                body: format!("https://tak.example.com:8443/Marti/sync/content?hash={hash}").into_bytes(),
            }
        }
        ("GET", ["Marti", "sync", "content"]) if req.q("hash") == Some("redirect") => Resp {
            status: 302,
            content_type: "text/plain",
            body: vec![],
        },
        ("GET", ["Marti", "sync", "content"]) => match req.q("hash").and_then(|h| st.files.get(h)) {
            Some((_, data)) => Resp { status: 200, content_type: "application/octet-stream", body: data.clone() },
            None => json_resp(404, &json!({})),
        },
        _ => json_resp(404, &json!({"error": format!("no route {m} {}", req.raw_path)})),
    }
}

async fn start(pki: &Pki) -> Server {
    let (cert_pem, key_pem, cert_der) =
        issue(pki, "localhost", ExtendedKeyUsagePurpose::ServerAuth);
    let _ = cert_pem;
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut roots = rustls::RootCertStore::empty();
    roots.add(CertificateDer::from(pki.ca_der.clone())).unwrap();
    let verifier = WebPkiClientVerifier::builder_with_provider(Arc::new(roots), provider.clone())
        .build()
        .unwrap();
    let tls = rustls::ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_client_cert_verifier(verifier)
        .with_single_cert(
            vec![CertificateDer::from(cert_der)],
            PrivateKeyDer::from_pem_slice(key_pem.as_bytes()).unwrap(),
        )
        .unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(tls));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let state = Arc::new(Mutex::new(State::default()));
    let state2 = state.clone();
    tokio::spawn(async move {
        loop {
            let Ok((tcp, _)) = listener.accept().await else {
                return;
            };
            let (acceptor, state) = (acceptor.clone(), state2.clone());
            tokio::spawn(async move {
                let Ok(mut stream) = acceptor.accept(tcp).await else {
                    return;
                };
                while let Some(req) = read_req(&mut stream).await {
                    let r = handle(&state, &req);
                    let head = format!(
                        "HTTP/1.1 {} X\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                        r.status,
                        r.content_type,
                        r.body.len()
                    );
                    if stream.write_all(head.as_bytes()).await.is_err()
                        || stream.write_all(&r.body).await.is_err()
                    {
                        return;
                    }
                }
            });
        }
    });
    Server { port, state }
}

async fn client_for(pki: &Pki, server: &Server) -> MartiClient {
    let (cert, key, _) = issue(pki, "alice", ExtendedKeyUsagePurpose::ClientAuth);
    let tls = TlsConfig::new()
        .with_ca_pem(&pki.ca_pem)
        .with_client_identity_pem(cert, key)
        .with_server_name("localhost");
    let mut cfg = MartiConfig::new("127.0.0.1", tls);
    cfg.port = server.port;
    MartiClient::new(cfg).await.unwrap()
}

// ---------------------------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------------------------

#[tokio::test]
async fn server_and_people_endpoints() {
    let pki = new_pki("Test CA");
    let server = start(&pki).await;
    let marti = client_for(&pki, &server).await;

    assert_eq!(marti.version().await.unwrap(), "5.2-RELEASE-39");

    let contacts = marti.contacts().await.unwrap();
    assert_eq!(contacts[0].callsign.as_deref(), Some("ALPHA"));
    assert!(
        contacts[0].filter_groups.is_empty(),
        "null list must decode as empty"
    );

    let q = EndpointQuery {
        seconds_ago: Some(60),
        currently_connected: true,
        groups: vec!["g1".into(), "g2".into()],
        ..Default::default()
    };
    let eps = marti.client_endpoints(&q).await.unwrap();
    assert_eq!(eps[0].username.as_deref(), Some("alice"));
    assert!(eps[0]
        .last_event_time
        .as_ref()
        .unwrap()
        .to_datetime()
        .is_some());

    assert_eq!(marti.groups().await.unwrap()[0].bitpos, Some(0));

    let seen = server.state.lock().unwrap().seen.clone();
    let ep = seen
        .iter()
        .find(|r| r.segments.last().map(String::as_str) == Some("clientEndPoints"))
        .unwrap();
    assert_eq!(ep.q("secAgo"), Some("60"));
    assert_eq!(ep.q("showCurrentlyConnectedClients"), Some("true"));
    assert_eq!(ep.all("group"), ["g1", "g2"]);
}

#[tokio::test]
async fn mission_lifecycle() {
    let pki = new_pki("Test CA");
    let server = start(&pki).await;
    let marti = client_for(&pki, &server).await;

    let new = NewMission {
        groups: vec!["Ops".into(), "Recon".into()],
        description: Some("Recon of the north".into()),
        default_role: Some(MissionRoleKind::ReadOnlySubscriber),
        ..Default::default()
    };
    let created = marti.create_mission("RECON-1", "U1", &new).await.unwrap();
    assert_eq!(created.name, "RECON-1");
    assert_eq!(created.groups, ["Ops", "Recon"]);

    // What the server received.
    let put = server
        .state
        .lock()
        .unwrap()
        .seen
        .iter()
        .find(|r| r.method == "PUT")
        .cloned()
        .unwrap();
    assert_eq!(put.q("creatorUid"), Some("U1"));
    assert_eq!(put.q("description"), Some("Recon of the north"));
    assert_eq!(put.q("defaultRole"), Some("MISSION_READONLY_SUBSCRIBER"));
    assert_eq!(put.all("group"), ["Ops", "Recon"]);

    assert!(matches!(
        marti.create_mission("RECON-1", "U1", &new).await,
        Err(Error::Conflict)
    ));
    assert_eq!(
        marti
            .missions(&MissionQuery::default())
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        marti
            .mission("RECON-1", None)
            .await
            .unwrap()
            .creator_uid
            .as_deref(),
        Some("U1")
    );

    // Content.
    let content = MissionContent {
        hashes: vec!["abc123".into()],
        uids: vec!["UID-1".into(), "UID-2".into()],
    };
    let m = marti.add_content("RECON-1", &content, "U1").await.unwrap();
    assert_eq!(m.uids.len(), 2);
    assert_eq!(m.contents[0].data.hash.as_deref(), Some("abc123"));
    assert!(matches!(
        marti
            .add_content("RECON-1", &MissionContent::default(), "U1")
            .await,
        Err(Error::InvalidArgument(_))
    ));
    let m = marti
        .remove_content("RECON-1", &ContentRef::Uid("UID-1".into()), "U1")
        .await
        .unwrap();
    assert_eq!(m.uids.len(), 1);
    let m = marti
        .remove_content("RECON-1", &ContentRef::Hash("abc123".into()), "U1")
        .await
        .unwrap();
    assert!(m.contents.is_empty());

    let changes = marti
        .changes(
            "RECON-1",
            &ChangeQuery {
                seconds_ago: Some(3600),
                squashed: Some(false),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(changes[0].change_type.as_deref(), Some("ADD_CONTENT"));
    let seen = server.state.lock().unwrap().seen.clone();
    let ch = seen
        .iter()
        .find(|r| r.segments.last().map(String::as_str) == Some("changes"))
        .unwrap();
    assert_eq!(
        (ch.q("secago"), ch.q("squashed")),
        (Some("3600"), Some("false"))
    );

    // Subscriptions.
    let sub = marti.subscribe("RECON-1", "U9", None).await.unwrap();
    assert_eq!(sub.client_uid.as_deref(), Some("U9"));
    assert_eq!(marti.subscribers("RECON-1").await.unwrap(), ["U9"]);
    marti.unsubscribe("RECON-1", "U9", true).await.unwrap();
    assert!(marti.subscribers("RECON-1").await.unwrap().is_empty());

    // Delete.
    marti.delete_mission("RECON-1", "U1", false).await.unwrap();
    assert!(matches!(
        marti.mission("RECON-1", None).await,
        Err(Error::NotFound)
    ));
    assert!(matches!(
        marti.delete_mission("RECON-1", "U1", false).await,
        Err(Error::NotFound)
    ));
}

#[tokio::test]
async fn names_are_percent_encoded_on_the_wire() {
    let pki = new_pki("Test CA");
    let server = start(&pki).await;
    let marti = client_for(&pki, &server).await;
    marti
        .create_mission("Op A/B #1?", "U1", &NewMission::default())
        .await
        .unwrap();
    let put = server
        .state
        .lock()
        .unwrap()
        .seen
        .iter()
        .find(|r| r.method == "PUT")
        .cloned()
        .unwrap();
    assert_eq!(put.raw_path, "/Marti/api/missions/Op%20A%2FB%20%231%3F");
    assert_eq!(put.segments.last().unwrap(), "Op A/B #1?");
    // The slash did not create an extra path segment.
    assert_eq!(put.segments.len(), 4);
}

#[tokio::test]
async fn mission_cot_returns_typed_events() {
    let pki = new_pki("Test CA");
    let server = start(&pki).await;
    let marti = client_for(&pki, &server).await;
    let events = marti.mission_cot("any").await.unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].uid, "UID-1");
    assert_eq!(
        events[0]
            .detail
            .get::<cottak_core::detail::Contact>()
            .unwrap()
            .callsign
            .as_deref(),
        Some("ALPHA")
    );
}

#[tokio::test]
async fn file_upload_and_download_round_trip() {
    let pki = new_pki("Test CA");
    let server = start(&pki).await;
    let marti = client_for(&pki, &server).await;

    // Binary data that includes CRLF and boundary-looking text.
    let data: Vec<u8> = b"PK\x03\x04\r\n--not-a-boundary--\r\n\x00\xff"
        .iter()
        .copied()
        .cycle()
        .take(5000)
        .collect();

    let opts = UploadOptions {
        mime: Some("application/zip".into()),
        keywords: vec!["recon".into(), "north".into()],
        creator_uid: Some("U1".into()),
        tool: Some("public".into()),
        ..Default::default()
    };
    let meta = marti.upload_file("plan.zip", &data, &opts).await.unwrap();
    let hash = meta["Hash"].as_str().unwrap().to_owned();
    assert_eq!(marti.download(&hash).await.unwrap(), data);

    let seen = server.state.lock().unwrap().seen.clone();
    let up = seen
        .iter()
        .find(|r| r.segments.last().map(String::as_str) == Some("upload"))
        .unwrap();
    assert_eq!(up.q("name"), Some("plan.zip"));
    assert_eq!(up.q("MIME"), Some("application/zip"));
    assert_eq!(up.all("Keywords"), ["recon", "north"]);
    assert_eq!(up.q("CreatorUid"), Some("U1"));

    // Data package endpoint.
    let pkg = marti
        .upload_data_package("dp.zip", "U1", &data, &UploadOptions::default())
        .await
        .unwrap();
    assert_eq!(pkg.hash.as_deref(), Some(fake_hash(&data).as_str()));
    assert!(pkg.url.contains("/Marti/sync/content?hash="));
    assert!(matches!(
        marti
            .upload_data_package("dp.zip", "", &data, &UploadOptions::default())
            .await,
        Err(Error::InvalidArgument(_))
    ));

    assert!(matches!(marti.download("0000").await, Err(Error::NotFound)));
}

#[tokio::test]
async fn size_limits_are_enforced_on_both_directions() {
    let pki = new_pki("Test CA");
    let server = start(&pki).await;
    let (cert, key, _) = issue(&pki, "alice", ExtendedKeyUsagePurpose::ClientAuth);
    let tls = TlsConfig::new()
        .with_ca_pem(&pki.ca_pem)
        .with_client_identity_pem(cert, key)
        .with_server_name("localhost");
    let mut cfg = MartiConfig::new("127.0.0.1", tls);
    cfg.port = server.port;
    cfg.max_upload = 100;
    cfg.max_download = 100;
    let marti = MartiClient::new(cfg).await.unwrap();

    let before = server.state.lock().unwrap().seen.len();
    assert!(matches!(
        marti
            .upload_file("f", &[0u8; 101], &UploadOptions::default())
            .await,
        Err(Error::TooLarge(100))
    ));
    assert_eq!(
        server.state.lock().unwrap().seen.len(),
        before,
        "oversized uploads must not reach the server"
    );

    marti
        .upload_file("small", &[7u8; 50], &UploadOptions::default())
        .await
        .unwrap();
    let hash = fake_hash(&[7u8; 50]);
    assert_eq!(marti.download(&hash).await.unwrap().len(), 50);
    server
        .state
        .lock()
        .unwrap()
        .files
        .insert("big".into(), ("big".into(), vec![0u8; 500]));
    assert!(matches!(
        marti.download("big").await,
        Err(Error::TooLarge(100))
    ));
}

#[tokio::test]
async fn http_errors_are_typed_and_redirects_are_not_followed() {
    let pki = new_pki("Test CA");
    let server = start(&pki).await;
    let marti = client_for(&pki, &server).await;
    assert!(matches!(
        marti.mission("forbidden", None).await,
        Err(Error::Forbidden)
    ));
    assert!(matches!(
        marti.mission("missing", None).await,
        Err(Error::NotFound)
    ));
    // A redirect (which could carry credentials elsewhere) is surfaced, never followed.
    assert!(matches!(
        marti.download("redirect").await,
        Err(Error::Status { status: 302, .. })
    ));
    // Invalid names never reach the network.
    let before = server.state.lock().unwrap().seen.len();
    assert!(matches!(
        marti.mission("", None).await,
        Err(Error::InvalidArgument(_))
    ));
    assert_eq!(server.state.lock().unwrap().seen.len(), before);
}

#[tokio::test]
async fn mutual_tls_is_required() {
    let pki = new_pki("Test CA");
    let server = start(&pki).await;
    // Trusts the server but presents no client certificate.
    let tls = TlsConfig::new()
        .with_ca_pem(&pki.ca_pem)
        .with_server_name("localhost");
    let mut cfg = MartiConfig::new("127.0.0.1", tls);
    cfg.port = server.port;
    let marti = MartiClient::new(cfg).await.unwrap();
    assert!(matches!(marti.version().await, Err(Error::Http(_))));
    assert!(server.state.lock().unwrap().seen.is_empty());
}

#[tokio::test]
async fn untrusted_server_is_refused() {
    let pki = new_pki("Test CA");
    let other = new_pki("Other CA");
    let server = start(&pki).await;
    let (cert, key, _) = issue(&pki, "alice", ExtendedKeyUsagePurpose::ClientAuth);
    let tls = TlsConfig::new()
        .with_ca_pem(&other.ca_pem)
        .with_client_identity_pem(cert, key)
        .with_server_name("localhost");
    let mut cfg = MartiConfig::new("127.0.0.1", tls);
    cfg.port = server.port;
    let marti = MartiClient::new(cfg).await.unwrap();
    assert!(matches!(marti.version().await, Err(Error::Http(_))));
}
