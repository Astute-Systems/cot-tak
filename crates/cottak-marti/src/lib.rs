//! A client for TAK Server's Marti REST API.
//!
//! Marti is the HTTP side of TAK Server: missions (data sync), file storage,
//! contacts, groups and server information. [`MartiClient`] speaks to it over
//! HTTPS with mutual TLS (a client certificate, for example one obtained with
//! `cottak-enroll`), authenticating the server by CA or by a pinned fingerprint
//! exactly like `cottak-net`.
//!
//! The request shapes follow the TAK Server source code (`MissionApi`,
//! `ContactsApi`, `GroupsApi`, `VersionApi` and the `sync` servlets). Responses
//! are decoded tolerantly: unknown fields are preserved in `extra` and missing
//! or `null` fields become `None`/empty, so differences between server
//! releases do not break decoding.
//!
//! | Area | Methods |
//! |---|---|
//! | Server | [`version`](MartiClient::version), [`version_info`](MartiClient::version_info), [`node_id`](MartiClient::node_id) |
//! | People | [`contacts`](MartiClient::contacts), [`client_endpoints`](MartiClient::client_endpoints), [`groups`](MartiClient::groups) |
//! | Missions | [`missions`](MartiClient::missions), [`mission`](MartiClient::mission), [`create_mission`](MartiClient::create_mission), [`delete_mission`](MartiClient::delete_mission) |
//! | Content | [`add_content`](MartiClient::add_content), [`remove_content`](MartiClient::remove_content), [`changes`](MartiClient::changes), [`mission_cot`](MartiClient::mission_cot) |
//! | Subscriptions | [`subscribe`](MartiClient::subscribe), [`unsubscribe`](MartiClient::unsubscribe), [`subscribers`](MartiClient::subscribers) |
//! | Files | [`upload_file`](MartiClient::upload_file), [`upload_data_package`](MartiClient::upload_data_package), [`download`](MartiClient::download) |

mod client;
pub mod error;
mod files;
pub mod models;

pub use client::{
    ChangeQuery, ContentRef, EndpointQuery, MartiClient, MartiConfig, MissionQuery,
    MissionRoleKind, NewMission, DEFAULT_PORT,
};
pub use error::{Error, Result};
pub use files::{UploadOptions, UploadedPackage};
pub use models::{
    ClientEndpoint, Contact, Group, Mission, MissionAdd, MissionChange, MissionContent,
    MissionRole, MissionSubscription, Resource, ServerTime,
};
