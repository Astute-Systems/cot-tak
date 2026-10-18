//! Marti API routes.

use super::http::{Request, Response};
use super::{json, multipart};
use crate::config::ServerConfig;
use crate::hub::{now_ms, Hub, State};
use crate::registry::{
    Addition, ContentRef, Failure, FileRecord, Mission, NewMission, ROLE_READ_ONLY,
};
use base64::Engine as _;
use cottak_core::Event;
use serde_json::{json as j, Value};
use sha2::{Digest, Sha256};

/// Longest accepted mission name or UID in a URL or query.
const MAX_NAME: usize = 255;

pub(crate) struct Context<'a> {
    pub(crate) hub: &'a Hub,
    pub(crate) config: &'a ServerConfig,
    pub(crate) tls: bool,
}

/// Dispatches one request. Never panics on hostile input.
pub(crate) fn handle(ctx: &Context<'_>, req: &Request) -> Response {
    let user = match authenticate(ctx.config, req) {
        Ok(u) => u,
        Err(resp) => return resp,
    };
    let seg: Vec<&str> = req.segments.iter().map(String::as_str).collect();
    let m = req.method.as_str();
    let node = ctx.config.server_uid.as_str();

    match (m, seg.as_slice()) {
        ("GET", ["Marti", "api", "version"]) => {
            Response::text(200, format!("cottak-server {}", env!("CARGO_PKG_VERSION")))
        }
        ("GET", ["Marti", "api", "version", "info"]) => Response::json(
            200,
            &json::envelope(
                "VersionInfo",
                j!({"version": env!("CARGO_PKG_VERSION"), "apiVersion": json::API_VERSION, "nodeId": node}),
                node,
            ),
        ),
        ("GET", ["Marti", "api", "node", "id"]) => Response::text(200, node),
        ("GET", ["Marti", "api", "contacts", "all"]) => {
            let list: Vec<Value> = ctx
                .hub
                .clients()
                .iter()
                .filter(|c| c.uid.is_some())
                .map(json::contact_json)
                .collect();
            Response::json(200, &Value::from(list))
        }
        ("GET", ["Marti", "api", "clientEndPoints"]) => {
            let list: Vec<Value> = ctx
                .hub
                .clients()
                .iter()
                .filter(|c| c.uid.is_some())
                .map(json::endpoint_json)
                .collect();
            Response::json(
                200,
                &json::envelope("ClientEndpoint", Value::from(list), node),
            )
        }
        ("GET", ["Marti", "api", "groups", "all"]) => {
            Response::json(200, &json::envelope("Group", json::groups_json(), node))
        }

        ("GET", ["Marti", "api", "missions"]) => list_missions(ctx, req),
        ("GET", ["Marti", "api", "missions", name]) => get_mission(ctx, req, name),
        ("PUT", ["Marti", "api", "missions", name]) => create_mission(ctx, req, name),
        ("DELETE", ["Marti", "api", "missions", name]) => delete_mission(ctx, req, name),
        ("PUT", ["Marti", "api", "missions", name, "contents"]) => add_content(ctx, req, name),
        ("DELETE", ["Marti", "api", "missions", name, "contents"]) => {
            remove_content(ctx, req, name)
        }
        ("GET", ["Marti", "api", "missions", name, "changes"]) => changes(ctx, req, name),
        ("PUT", ["Marti", "api", "missions", name, "subscription"]) => {
            subscribe(ctx, req, name, user.as_deref())
        }
        ("DELETE", ["Marti", "api", "missions", name, "subscription"]) => {
            unsubscribe(ctx, req, name)
        }
        ("GET", ["Marti", "api", "missions", name, "subscriptions"]) => subscribers(ctx, name),
        ("GET", ["Marti", "api", "missions", name, "cot"]) => mission_cot(ctx, name),

        ("POST", ["Marti", "sync", "upload"]) => upload(ctx, req, user.as_deref(), false),
        ("POST", ["Marti", "sync", "missionupload"]) => upload(ctx, req, user.as_deref(), true),
        ("GET", ["Marti", "sync", "content"]) => download(ctx, req),

        (_, path) if known_path(path) => Response::text(405, "method not allowed"),
        _ => Response::text(404, "not found"),
    }
}

fn known_path(seg: &[&str]) -> bool {
    matches!(
        seg,
        ["Marti", "api", "version" | "clientEndPoints" | "missions"]
            | ["Marti", "api", "version", "info"]
            | ["Marti", "api", "node", "id"]
            | ["Marti", "api", "contacts" | "groups", "all"]
            | ["Marti", "api", "missions", _]
            | [
                "Marti",
                "api",
                "missions",
                _,
                "contents" | "changes" | "subscription" | "subscriptions" | "cot"
            ]
            | ["Marti", "sync", "upload" | "missionupload" | "content"]
    )
}

// ----- authentication -----------------------------------------------------------------------

/// Checks HTTP Basic credentials when the server requires them. Returns the user name, if any.
fn authenticate(config: &ServerConfig, req: &Request) -> Result<Option<String>, Response> {
    let given = req
        .header("authorization")
        .and_then(|h| {
            h.strip_prefix("Basic ")
                .or_else(|| h.strip_prefix("basic "))
        })
        .and_then(|b| {
            base64::engine::general_purpose::STANDARD
                .decode(b.trim())
                .ok()
        })
        .and_then(|raw| String::from_utf8(raw).ok())
        .and_then(|s| s.split_once(':').map(|(u, p)| (u.to_owned(), p.to_owned())));
    let Some(required) = &config.marti_basic_auth else {
        return Ok(given.map(|(u, _)| u));
    };
    match given {
        Some((u, p)) if required.matches(&u, &p) => Ok(Some(u)),
        _ => Err(Response::text(401, "authentication required")),
    }
}

// ----- helpers ------------------------------------------------------------------------------

fn failure(f: Failure) -> Response {
    match f {
        Failure::NotFound => Response::text(404, "not found"),
        Failure::Conflict => Response::text(409, "already exists"),
        Failure::Forbidden => Response::text(403, "forbidden"),
        Failure::Invalid(why) => Response::text(400, why),
        Failure::Full => Response::text(507, "server storage limit reached"),
    }
}

fn bad(why: &'static str) -> Response {
    Response::text(400, why)
}

fn valid_name(s: &str) -> bool {
    !s.is_empty() && s.len() <= MAX_NAME && !s.chars().any(char::is_control)
}

/// A required, sane query parameter.
fn required<'r>(req: &'r Request, name: &str) -> Result<&'r str, Response> {
    match req.param_ci(name) {
        Some(v) if valid_name(v) => Ok(v),
        _ => Err(bad("missing or invalid query parameter")),
    }
}

fn optional(req: &Request, name: &str) -> Option<String> {
    req.param_ci(name)
        .filter(|v| v.len() <= 4096 && !v.chars().any(char::is_control))
        .map(str::to_owned)
}

/// The mission named `name` wrapped as the one-element list the API returns.
fn mission_response(state: &State, name: &str, status: u16, node: &str) -> Response {
    match state.registry.mission(name) {
        Ok(m) => Response::json(
            status,
            &json::envelope(
                "Mission",
                j!([json::mission_json(m, true, &state.registry.files)]),
                node,
            ),
        ),
        Err(f) => failure(f),
    }
}

fn can_write(m: &Mission, uid: &str) -> bool {
    uid == m.creator_uid || (m.subscribers.contains_key(uid) && m.default_role != ROLE_READ_ONLY)
}

// ----- missions -----------------------------------------------------------------------------

fn list_missions(ctx: &Context<'_>, req: &Request) -> Response {
    let include_protected = req.flag("passwordProtected");
    let tool = req.param("tool");
    let st = ctx.hub.lock();
    let list: Vec<Value> = st
        .registry
        .missions
        .values()
        .filter(|m| include_protected || !m.password_protected())
        .filter(|m| tool.is_none_or(|t| m.tool == t))
        .map(|m| json::mission_json(m, false, &st.registry.files))
        .collect();
    Response::json(
        200,
        &json::envelope("Mission", Value::from(list), &ctx.config.server_uid),
    )
}

fn get_mission(ctx: &Context<'_>, req: &Request, name: &str) -> Response {
    let st = ctx.hub.lock();
    match st.registry.mission(name) {
        Ok(m) if !m.password_ok(req.param("password")) => failure(Failure::Forbidden),
        Ok(_) => mission_response(&st, name, 200, &ctx.config.server_uid),
        Err(f) => failure(f),
    }
}

fn create_mission(ctx: &Context<'_>, req: &Request, name: &str) -> Response {
    if !valid_name(name) {
        return bad("invalid mission name");
    }
    let creator = match required(req, "creatorUid") {
        Ok(c) => c.to_owned(),
        Err(r) => return r,
    };
    let params = NewMission {
        name: name.to_owned(),
        description: optional(req, "description"),
        chat_room: optional(req, "chatRoom"),
        base_layer: optional(req, "baseLayer"),
        bbox: optional(req, "bbox"),
        path: optional(req, "path"),
        classification: optional(req, "classification"),
        tool: optional(req, "tool"),
        creator_uid: creator,
        password: optional(req, "password"),
        default_role: optional(req, "defaultRole"),
        invite_only: req.flag("inviteOnly"),
        expiration: req
            .param("expiration")
            .and_then(|e| e.parse().ok())
            .unwrap_or(-1),
    };
    let mut st = ctx.hub.lock();
    match st.registry.create(params, now_ms(), ctx.hub.limits()) {
        Ok(_) => mission_response(&st, name, 201, &ctx.config.server_uid),
        Err(f) => failure(f),
    }
}

fn delete_mission(ctx: &Context<'_>, req: &Request, name: &str) -> Response {
    let creator = match required(req, "creatorUid") {
        Ok(c) => c,
        Err(r) => return r,
    };
    let mut st = ctx.hub.lock();
    match st.registry.mission(name) {
        Ok(m) if m.creator_uid != creator => return failure(Failure::Forbidden),
        Ok(_) => {}
        Err(f) => return failure(f),
    }
    match st.registry.delete(name, req.flag("deepDelete")) {
        Ok(()) => Response::empty(200),
        Err(f) => failure(f),
    }
}

fn add_content(ctx: &Context<'_>, req: &Request, name: &str) -> Response {
    let creator = match required(req, "creatorUid") {
        Ok(c) => c,
        Err(r) => return r,
    };
    let Ok(body) = serde_json::from_slice::<Value>(&req.body) else {
        return bad("body must be JSON");
    };
    let strings = |key: &str| -> Option<Vec<String>> {
        match body.get(key) {
            None | Some(Value::Null) => Some(Vec::new()),
            Some(Value::Array(a)) => a
                .iter()
                .map(|v| v.as_str().filter(|s| valid_name(s)).map(str::to_owned))
                .collect(),
            Some(_) => None,
        }
    };
    let (Some(hashes), Some(uids)) = (strings("hashes"), strings("uids")) else {
        return bad("hashes and uids must be arrays of strings");
    };
    if hashes.is_empty() && uids.is_empty() {
        return bad("nothing to add");
    }

    let add = Addition {
        hashes: &hashes,
        uids: &uids,
        creator,
    };
    let limits = ctx.hub.limits();
    let mut st = ctx.hub.lock();
    match st.registry.mission(name) {
        Ok(m) if !can_write(m, creator) => return failure(Failure::Forbidden),
        Ok(_) => {}
        Err(f) => return failure(f),
    }
    let State {
        store, registry, ..
    } = &mut *st;
    let result = registry.add_content(name, &add, now_ms(), limits, |uid| store.get(uid).cloned());
    match result {
        Ok(_) => mission_response(&st, name, 200, &ctx.config.server_uid),
        Err(f) => failure(f),
    }
}

fn remove_content(ctx: &Context<'_>, req: &Request, name: &str) -> Response {
    let creator = match required(req, "creatorUid") {
        Ok(c) => c,
        Err(r) => return r,
    };
    let item = match (req.param("hash"), req.param("uid")) {
        (Some(h), None) if valid_name(h) => ContentRef::Hash(h),
        (None, Some(u)) if valid_name(u) => ContentRef::Uid(u),
        _ => return bad("give exactly one of hash or uid"),
    };
    let mut st = ctx.hub.lock();
    match st.registry.mission(name) {
        Ok(m) if !can_write(m, creator) => return failure(Failure::Forbidden),
        Ok(_) => {}
        Err(f) => return failure(f),
    }
    match st.registry.remove_content(name, &item, creator, now_ms()) {
        Ok(_) => mission_response(&st, name, 200, &ctx.config.server_uid),
        Err(f) => failure(f),
    }
}

fn changes(ctx: &Context<'_>, req: &Request, name: &str) -> Response {
    let now = now_ms();
    let parse_time = |key: &str| {
        req.param(key)
            .and_then(|s| cottak_core::timestamp::parse(s).ok())
            .map(|t| i64::try_from(t.unix_timestamp_nanos() / 1_000_000).unwrap_or(0))
    };
    let from = req
        .param("secago")
        .and_then(|s| s.parse::<i64>().ok())
        .map(|s| now.saturating_sub(s.saturating_mul(1000)))
        .or_else(|| parse_time("start"));
    let to = parse_time("end");

    let st = ctx.hub.lock();
    let m = match st.registry.mission(name) {
        Ok(m) => m,
        Err(f) => return failure(f),
    };
    let list: Vec<Value> = m
        .changes
        .iter()
        .filter(|c| from.is_none_or(|f| c.ts >= f) && to.is_none_or(|t| c.ts <= t))
        .map(|c| json::change_json(m, c, &st.registry.files))
        .collect();
    Response::json(
        200,
        &json::envelope("MissionChange", Value::from(list), &ctx.config.server_uid),
    )
}

fn subscribe(ctx: &Context<'_>, req: &Request, name: &str, user: Option<&str>) -> Response {
    let uid = match required(req, "uid") {
        Ok(u) => u,
        Err(r) => return r,
    };
    let mut st = ctx.hub.lock();
    match st
        .registry
        .subscribe(name, uid, req.param("password"), now_ms())
    {
        Ok(m) => Response::json(
            201,
            &json::envelope(
                "MissionSubscription",
                json::subscription_json(m, uid, user),
                &ctx.config.server_uid,
            ),
        ),
        Err(f) => failure(f),
    }
}

fn unsubscribe(ctx: &Context<'_>, req: &Request, name: &str) -> Response {
    let uid = match required(req, "uid") {
        Ok(u) => u,
        Err(r) => return r,
    };
    match ctx.hub.lock().registry.unsubscribe(name, uid) {
        Ok(()) => Response::empty(200),
        Err(f) => failure(f),
    }
}

fn subscribers(ctx: &Context<'_>, name: &str) -> Response {
    let st = ctx.hub.lock();
    match st.registry.mission(name) {
        Ok(m) => {
            let uids: Vec<&String> = m.subscribers.keys().collect();
            Response::json(
                200,
                &json::envelope("String", j!(uids), &ctx.config.server_uid),
            )
        }
        Err(f) => failure(f),
    }
}

fn mission_cot(ctx: &Context<'_>, name: &str) -> Response {
    let events: Vec<std::sync::Arc<Event>> = {
        let st = ctx.hub.lock();
        match st.registry.mission(name) {
            Ok(m) => {
                let mut v: Vec<_> = m.latest.values().cloned().collect();
                v.sort_by(|a, b| a.uid.cmp(&b.uid));
                v
            }
            Err(f) => return failure(f),
        }
    };
    let mut xml =
        String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><events>");
    for ev in events {
        match ev.to_xml_string() {
            Ok(s) => xml.push_str(&s),
            Err(_) => return Response::text(500, "cannot serialise event"),
        }
    }
    xml.push_str("</events>");
    Response::new(200, "application/xml", xml)
}

// ----- files --------------------------------------------------------------------------------

fn upload(ctx: &Context<'_>, req: &Request, user: Option<&str>, package: bool) -> Response {
    let Some(boundary) = req.header("content-type").and_then(multipart::boundary) else {
        return bad("expected multipart/form-data");
    };
    let Some(part) = multipart::asset_file(&req.body, boundary) else {
        return bad("no `assetfile` part");
    };
    let (name_key, mime_key) = if package {
        ("filename", "mimetype")
    } else {
        ("name", "MIME")
    };
    let Some(name) = req
        .param_ci(name_key)
        .map(str::to_owned)
        .or(part.filename)
        .filter(|n| valid_name(n))
    else {
        return bad("missing file name");
    };
    let mime = optional(req, mime_key)
        .or(part.content_type)
        .unwrap_or_else(|| "application/octet-stream".to_owned());
    let keywords: Vec<String> = req
        .params_ci(if package { "keyword" } else { "Keywords" })
        .into_iter()
        .filter(|k| valid_name(k))
        .map(str::to_owned)
        .collect();
    let hash = hex(&Sha256::digest(&part.data));

    let record = FileRecord {
        id: 0,
        hash: hash.clone(),
        name,
        mime,
        uid: Event::random_uid(),
        submitter: user.map(str::to_owned),
        creator_uid: optional(req, if package { "creatorUid" } else { "CreatorUid" }),
        ts: now_ms(),
        keywords,
        tool: optional(req, if package { "tool" } else { "Tool" })
            .unwrap_or_else(|| "public".to_owned()),
        data: std::sync::Arc::new(part.data),
    };
    let mut st = ctx.hub.lock();
    match st.registry.add_file(record, ctx.hub.limits()) {
        Ok(f) if package => {
            let scheme = if ctx.tls { "https" } else { "http" };
            let host = req
                .header("host")
                .filter(|h| h.len() <= 255 && h.chars().all(|c| c.is_ascii_graphic()))
                .unwrap_or("localhost");
            Response::text(
                200,
                format!("{scheme}://{host}/Marti/sync/content?hash={}", f.hash),
            )
        }
        Ok(f) => Response::json(200, &json::upload_json(f)),
        Err(f) => failure(f),
    }
}

fn download(ctx: &Context<'_>, req: &Request) -> Response {
    let Some(hash) = req.param("hash") else {
        return bad("missing hash");
    };
    let found = ctx
        .hub
        .lock()
        .registry
        .files
        .get(hash)
        .map(|f| (f.mime.clone(), std::sync::Arc::clone(&f.data)));
    match found {
        Some((mime, data)) => Response::new(200, safe_content_type(&mime), data.as_ref().clone()),
        None => failure(Failure::NotFound),
    }
}

/// Content types seen in practice; anything else is served as opaque bytes, so an
/// uploaded MIME string can never inject header content.
fn safe_content_type(mime: &str) -> &'static str {
    const KNOWN: &[&str] = &[
        "application/zip",
        "application/x-zip-compressed",
        "application/xml",
        "application/json",
        "text/plain",
        "text/xml",
        "image/png",
        "image/jpeg",
        "application/pdf",
    ];
    KNOWN
        .iter()
        .find(|k| k.eq_ignore_ascii_case(mime))
        .copied()
        .unwrap_or("application/octet-stream")
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
}
