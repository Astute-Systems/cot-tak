//! JSON models for the Marti API.
//!
//! The models follow the TAK Server source (`com.bbn.marti.sync.model.*`).
//! They are deliberately tolerant: every field is optional, `null` is treated
//! as absent, and unknown fields are kept in `extra`, so a newer or older
//! server never breaks decoding.

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use time::OffsetDateTime;

fn null_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

/// A server timestamp. Jackson emits either epoch milliseconds or an
/// ISO-8601 string depending on the field, so both are accepted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerTime {
    /// Milliseconds since the Unix epoch.
    Millis(i64),
    /// Text such as `2025-01-31T12:00:00.000Z`.
    Text(String),
}

impl ServerTime {
    /// The instant, if it can be interpreted.
    pub fn to_datetime(&self) -> Option<OffsetDateTime> {
        match self {
            Self::Millis(ms) => {
                OffsetDateTime::from_unix_timestamp_nanos(i128::from(*ms) * 1_000_000).ok()
            }
            Self::Text(s) => cottak_core::timestamp::parse(s).ok(),
        }
    }
}

/// The envelope TAK Server wraps most JSON answers in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "T: Deserialize<'de>"))]
pub struct ApiResponse<T> {
    /// API version string.
    #[serde(default)]
    pub version: Option<String>,
    /// Name of the payload type, for example `Mission`.
    #[serde(default, rename = "type")]
    pub kind: Option<String>,
    /// The payload.
    #[serde(default)]
    pub data: Option<T>,
    /// Informational messages from the server.
    #[serde(default, deserialize_with = "null_default")]
    pub messages: Vec<String>,
    /// Id of the server node that answered.
    #[serde(default)]
    pub node_id: Option<String>,
}

/// A role within a mission (owner, subscriber, read-only, ...).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MissionRole {
    /// Role name, for example `MISSION_OWNER`.
    #[serde(rename = "type")]
    pub role_type: Option<String>,
    /// Permissions granted by the role (strings or objects depending on server version).
    #[serde(deserialize_with = "null_default")]
    pub permissions: Vec<Value>,
    /// Fields this crate does not model.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// An item added to a mission: UID or file, with who added it and when.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MissionAdd<T: Default> {
    /// The UID string or the file metadata.
    pub data: T,
    /// When it was added.
    pub timestamp: Option<ServerTime>,
    /// UID of the client that added it.
    pub creator_uid: Option<String>,
    /// Fields this crate does not model (for example `details`).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Metadata of a file in Enterprise Sync (a mission content item).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Resource {
    /// Server-side id.
    pub id: Option<i64>,
    /// Original file name.
    pub filename: Option<String>,
    /// Display name.
    pub name: Option<String>,
    /// SHA-256 hash that identifies the content.
    pub hash: Option<String>,
    /// Server UID of the file.
    pub uid: Option<String>,
    /// MIME type.
    pub mime_type: Option<String>,
    /// Size in bytes.
    pub size: Option<i64>,
    /// User who submitted it.
    pub submitter: Option<String>,
    /// UID of the submitting client.
    pub creator_uid: Option<String>,
    /// When it was submitted.
    pub submission_time: Option<ServerTime>,
    /// Keywords.
    #[serde(deserialize_with = "null_default")]
    pub keywords: Vec<String>,
    /// Tool that owns the file (`public`, `vbm`, ...).
    pub tool: Option<String>,
    /// Fields this crate does not model.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A mission (data sync feed).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Mission {
    /// Mission name (its identifier in URLs).
    pub name: String,
    /// Description.
    pub description: Option<String>,
    /// Associated chat room.
    pub chat_room: Option<String>,
    /// Base map layer.
    pub base_layer: Option<String>,
    /// Bounding box.
    pub bbox: Option<String>,
    /// Bounding polygon.
    pub bounding_polygon: Option<String>,
    /// Path.
    pub path: Option<String>,
    /// Classification marking.
    pub classification: Option<String>,
    /// Owning tool (`public` by default).
    pub tool: Option<String>,
    /// UID of the creating client.
    pub creator_uid: Option<String>,
    /// When it was created.
    pub create_time: Option<ServerTime>,
    /// When it was last edited.
    pub last_edited: Option<ServerTime>,
    /// Keywords.
    #[serde(deserialize_with = "null_default")]
    pub keywords: Vec<String>,
    /// CoT UIDs in the mission.
    #[serde(deserialize_with = "null_default")]
    pub uids: Vec<MissionAdd<String>>,
    /// Files in the mission.
    #[serde(deserialize_with = "null_default")]
    pub contents: Vec<MissionAdd<Resource>>,
    /// Groups (channels) the mission belongs to.
    #[serde(deserialize_with = "null_default")]
    pub groups: Vec<String>,
    /// Whether a password is required to join.
    pub password_protected: Option<bool>,
    /// Access token, when the server issued one.
    pub token: Option<String>,
    /// Whether joining needs an invitation.
    pub invite_only: Option<bool>,
    /// Expiration in epoch milliseconds (`-1` for none).
    pub expiration: Option<i64>,
    /// Globally unique id.
    pub guid: Option<String>,
    /// Role given to new subscribers.
    pub default_role: Option<MissionRole>,
    /// The caller's role.
    pub owner_role: Option<MissionRole>,
    /// Fields this crate does not model (children, feeds, map layers, ...).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// What to add to a mission with [`MartiClient::add_content`](crate::MartiClient::add_content).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct MissionContent {
    /// Hashes of files already uploaded.
    pub hashes: Vec<String>,
    /// CoT UIDs.
    pub uids: Vec<String>,
}

/// One entry in a mission's change log.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MissionChange {
    /// Kind, for example `ADD_CONTENT`, `REMOVE_CONTENT`, `CREATE_MISSION`.
    #[serde(rename = "type")]
    pub change_type: Option<String>,
    /// Hash of the file added or removed.
    pub content_hash: Option<String>,
    /// UID added or removed.
    pub content_uid: Option<String>,
    /// Mission name.
    pub mission_name: Option<String>,
    /// Mission guid.
    pub mission_guid: Option<String>,
    /// Client-side time of the change.
    pub timestamp: Option<ServerTime>,
    /// Server-side time of the change.
    pub servertime: Option<ServerTime>,
    /// UID of the client that made the change.
    pub creator_uid: Option<String>,
    /// File metadata for content changes.
    pub content_resource: Option<Resource>,
    /// Fields this crate does not model.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A mission subscription as returned when subscribing.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MissionSubscription {
    /// Mission name.
    pub mission_name: Option<String>,
    /// Subscribing client UID.
    pub client_uid: Option<String>,
    /// Username.
    pub username: Option<String>,
    /// Role in the mission.
    pub role: Option<MissionRole>,
    /// Access token for private missions.
    pub token: Option<String>,
    /// Fields this crate does not model.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A contact as listed by `/contacts/all`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Contact {
    /// Client UID.
    pub uid: Option<String>,
    /// Callsign.
    pub callsign: Option<String>,
    /// Team colour.
    pub team: Option<String>,
    /// Team role.
    pub role: Option<String>,
    /// Client software description.
    pub takv: Option<String>,
    /// Notes.
    pub notes: Option<String>,
    /// Groups the contact is filtered to.
    #[serde(deserialize_with = "null_default")]
    pub filter_groups: Vec<String>,
    /// Fields this crate does not model.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A client endpoint as listed by `/clientEndPoints`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ClientEndpoint {
    /// Callsign.
    pub callsign: Option<String>,
    /// Client UID.
    pub uid: Option<String>,
    /// Username the client authenticated as.
    pub username: Option<String>,
    /// Team colour.
    pub team: Option<String>,
    /// Team role.
    pub role: Option<String>,
    /// Time of the last event received from the client.
    pub last_event_time: Option<ServerTime>,
    /// Connection status text.
    pub last_status: Option<String>,
    /// Groups, as a display string.
    pub groups: Option<String>,
    /// Fields this crate does not model.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A TAK Server group (channel).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Group {
    /// Group name.
    pub name: String,
    /// `IN` or `OUT`.
    pub direction: Option<String>,
    /// Group type, for example `SYSTEM` or `LDAP`.
    #[serde(rename = "type")]
    pub group_type: Option<String>,
    /// Position in the group bit vector.
    pub bitpos: Option<i64>,
    /// Whether the group is active for the caller.
    pub active: Option<bool>,
    /// Creation time.
    pub created: Option<ServerTime>,
    /// Description.
    pub description: Option<String>,
    /// Fields this crate does not model.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mission_tolerates_nulls_unknown_fields_and_both_time_formats() {
        let json = r#"{
            "name": "alpha", "description": "d", "chatRoom": null, "keywords": null,
            "createTime": "2025-01-31T12:00:00.000Z", "lastEdited": 1738324800000,
            "uids": [{"data": "UID-1", "timestamp": "2025-01-31T12:00:00.000Z", "creatorUid": "c", "details": {"type": "a-f-G"}}],
            "contents": [{"data": {"hash": "abc", "filename": "a.zip", "size": 10, "keywords": null}, "creatorUid": "c"}],
            "groups": ["__ANON__"], "passwordProtected": false, "expiration": -1,
            "defaultRole": {"type": "MISSION_SUBSCRIBER", "permissions": ["MISSION_READ"]},
            "children": [], "someFutureField": {"x": 1}
        }"#;
        let m: Mission = serde_json::from_str(json).unwrap();
        assert_eq!(m.name, "alpha");
        assert!(m.chat_room.is_none() && m.keywords.is_empty());
        assert_eq!(m.uids[0].data, "UID-1");
        assert_eq!(m.uids[0].extra["details"]["type"], "a-f-G");
        assert_eq!(m.contents[0].data.hash.as_deref(), Some("abc"));
        assert_eq!(
            m.default_role.unwrap().permissions,
            [Value::from("MISSION_READ")]
        );
        assert!(m.extra.contains_key("someFutureField"));
        assert_eq!(
            m.create_time.unwrap().to_datetime(),
            m.last_edited.unwrap().to_datetime()
        );
    }

    #[test]
    fn api_response_envelope() {
        let r: ApiResponse<Vec<Group>> = serde_json::from_str(
            r#"{"version":"3","type":"Group","data":[{"name":"g","direction":"OUT","bitpos":3,"active":true}],"nodeId":"n1","messages":null}"#,
        )
        .unwrap();
        assert_eq!(r.node_id.as_deref(), Some("n1"));
        let g = &r.data.unwrap()[0];
        assert_eq!(
            (g.name.as_str(), g.bitpos, g.active),
            ("g", Some(3), Some(true))
        );

        let empty: ApiResponse<Vec<Group>> = serde_json::from_str(r#"{"data":null}"#).unwrap();
        assert!(empty.data.is_none());
    }

    #[test]
    fn mission_round_trips_through_json() {
        let mut m = Mission {
            name: "m".into(),
            ..Mission::default()
        };
        m.uids.push(MissionAdd {
            data: "U".into(),
            ..MissionAdd::default()
        });
        m.extra.insert("future".into(), Value::Bool(true));
        let back: Mission = serde_json::from_str(&serde_json::to_string(&m).unwrap()).unwrap();
        assert_eq!(back, m);
    }

    #[test]
    fn server_time_formats() {
        assert!(ServerTime::Millis(0).to_datetime().is_some());
        assert!(ServerTime::Text("2025-01-31T12:00:00.000Z".into())
            .to_datetime()
            .is_some());
        assert!(ServerTime::Text("yesterday".into()).to_datetime().is_none());
    }
}
