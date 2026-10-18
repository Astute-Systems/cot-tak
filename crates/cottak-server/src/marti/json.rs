//! JSON shapes of the Marti API, as TAK Server produces them.

use crate::hub::ClientInfo;
use crate::registry::{Change, FileRecord, Mission, ROLE_OWNER, ROLE_READ_ONLY, ROLE_SUBSCRIBER};
use cottak_core::detail::Contact;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;

pub(crate) const API_VERSION: &str = "3";

/// Wraps `data` the way most TAK Server answers are wrapped.
pub(crate) fn envelope(kind: &str, data: Value, node_id: &str) -> Value {
    json!({"version": API_VERSION, "type": kind, "data": data, "nodeId": node_id})
}

/// ISO-8601 text for epoch milliseconds.
pub(crate) fn iso(ms: i64) -> String {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(ms) * 1_000_000)
        .ok()
        .and_then(|t| cottak_core::timestamp::format(t).ok())
        .unwrap_or_default()
}

fn system_ms(t: SystemTime) -> i64 {
    t.duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|d| i64::try_from(d.as_millis()).ok())
        .unwrap_or(0)
}

pub(crate) fn role(kind: &str) -> Value {
    let permissions: &[&str] = match kind {
        ROLE_OWNER => &[
            "MISSION_WRITE",
            "MISSION_READ",
            "MISSION_DELETE",
            "MISSION_SET_ROLE",
            "MISSION_SET_PASSWORD",
            "MISSION_UPDATE_GROUPS",
        ],
        ROLE_READ_ONLY => &["MISSION_READ"],
        _ => &["MISSION_WRITE", "MISSION_READ"],
    };
    json!({"type": kind, "permissions": permissions})
}

/// The upload response (`Metadata` in the TAK Server source; note the capitalised keys).
pub(crate) fn upload_json(f: &FileRecord) -> Value {
    json!({
        "UID": f.uid,
        "SubmissionDateTime": iso(f.ts),
        "Keywords": f.keywords,
        "MIMEType": f.mime,
        "SubmissionUser": f.submitter,
        "PrimaryKey": f.id.to_string(),
        "Hash": f.hash,
        "CreatorUid": f.creator_uid,
        "Name": f.name,
        "Size": f.data.len(),
        "Tool": f.tool,
    })
}

/// A file as it appears inside a mission (`Resource`).
pub(crate) fn resource_json(f: &FileRecord) -> Value {
    json!({
        "id": f.id,
        "filename": f.name,
        "name": f.name,
        "hash": f.hash,
        "uid": f.uid,
        "mimeType": f.mime,
        "size": f.data.len(),
        "submitter": f.submitter,
        "creatorUid": f.creator_uid,
        "submissionTime": iso(f.ts),
        "keywords": f.keywords,
        "tool": f.tool,
    })
}

/// A mission. Without `full`, the content lists are left empty, as in the list endpoint.
pub(crate) fn mission_json(m: &Mission, full: bool, files: &HashMap<String, FileRecord>) -> Value {
    let uids: Vec<Value> = if full {
        m.uids
            .iter()
            .map(|u| {
                let details = m.latest.get(&u.uid).map(|e| {
                    json!({
                        "type": e.cot_type.as_str(),
                        "callsign": e.detail.get::<Contact>().and_then(|c| c.callsign),
                    })
                });
                json!({"data": u.uid, "timestamp": iso(u.ts), "creatorUid": u.creator, "details": details})
            })
            .collect()
    } else {
        Vec::new()
    };
    let contents: Vec<Value> = if full {
        m.contents
            .iter()
            .filter_map(|c| {
                files.get(&c.hash).map(|f| {
                    json!({"data": resource_json(f), "timestamp": iso(c.ts), "creatorUid": c.creator})
                })
            })
            .collect()
    } else {
        Vec::new()
    };
    json!({
        "name": m.name,
        "description": m.description,
        "chatRoom": m.chat_room,
        "baseLayer": m.base_layer,
        "bbox": m.bbox,
        "path": m.path,
        "classification": m.classification,
        "tool": m.tool,
        "creatorUid": m.creator_uid,
        "createTime": iso(m.created),
        "lastEdited": iso(m.last_edited),
        "keywords": Vec::<String>::new(),
        "uids": uids,
        "contents": contents,
        "groups": ["__ANON__"],
        "passwordProtected": m.password_protected(),
        "inviteOnly": m.invite_only,
        "expiration": m.expiration,
        "guid": m.guid,
        "defaultRole": role(m.default_role),
        "ownerRole": role(ROLE_OWNER),
    })
}

pub(crate) fn change_json(m: &Mission, c: &Change, files: &HashMap<String, FileRecord>) -> Value {
    json!({
        "type": c.kind,
        "contentHash": c.hash,
        "contentUid": c.uid,
        "missionName": m.name,
        "missionGuid": m.guid,
        "timestamp": iso(c.ts),
        "servertime": iso(c.ts),
        "creatorUid": c.creator,
        "contentResource": c.hash.as_ref().and_then(|h| files.get(h)).map(resource_json),
    })
}

pub(crate) fn subscription_json(m: &Mission, uid: &str, username: Option<&str>) -> Value {
    json!({
        "missionName": m.name,
        "clientUid": uid,
        "username": username,
        "role": role(subscriber_role(m.default_role)),
    })
}

/// The role a new subscriber gets: the mission default, never owner.
fn subscriber_role(default: &'static str) -> &'static str {
    if default == ROLE_OWNER {
        ROLE_SUBSCRIBER
    } else {
        default
    }
}

pub(crate) fn contact_json(c: &ClientInfo) -> Value {
    json!({
        "uid": c.uid,
        "callsign": c.callsign,
        "team": c.team,
        "role": c.role,
        "takv": null,
        "notes": null,
        "filterGroups": Vec::<String>::new(),
    })
}

pub(crate) fn endpoint_json(c: &ClientInfo) -> Value {
    json!({
        "callsign": c.callsign,
        "uid": c.uid,
        "username": null,
        "team": c.team,
        "role": c.role,
        "lastEventTime": c.last_event.map(system_ms),
        "lastStatus": "Connected",
        "groups": "[__ANON__]",
    })
}

pub(crate) fn groups_json() -> Value {
    json!([
        {"name": "__ANON__", "direction": "IN", "type": "SYSTEM", "bitpos": 2, "active": true, "created": null, "description": null},
        {"name": "__ANON__", "direction": "OUT", "type": "SYSTEM", "bitpos": 2, "active": true, "created": null, "description": null},
    ])
}
