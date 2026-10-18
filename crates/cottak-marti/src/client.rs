//! The Marti REST client.

use crate::error::{Error, Result};
use crate::models::{
    ApiResponse, ClientEndpoint, Contact, Group, Mission, MissionChange, MissionContent,
    MissionSubscription,
};
use cottak_core::xml::XmlElement;
use cottak_core::Event;
use cottak_net::TlsConfig;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;
use url::Url;

/// Default HTTPS port of the Marti API (client-certificate authentication).
pub const DEFAULT_PORT: u16 = 8443;

const MAX_JSON: usize = 16 * 1024 * 1024;
const MAX_ERROR_BODY: usize = 512;

/// Settings for [`MartiClient`].
#[derive(Clone)]
pub struct MartiConfig {
    /// TAK Server host name or IP address.
    pub host: String,
    /// HTTPS port.
    pub port: u16,
    /// Server authentication (CA and/or pin) and the client certificate for mutual TLS.
    pub tls: TlsConfig,
    /// Optional HTTP Basic credentials, for servers/ports that use password login.
    pub basic_auth: Option<(String, String)>,
    /// Limit for each request (uploads and downloads included).
    pub timeout: Duration,
    /// Largest accepted download, in bytes.
    pub max_download: usize,
    /// Largest accepted upload, in bytes (TAK Server's own limit is at most 550 MiB).
    pub max_upload: usize,
}

impl fmt::Debug for MartiConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MartiConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("tls", &self.tls)
            .field(
                "basic_auth",
                &self.basic_auth.as_ref().map(|(u, _)| (u, "<redacted>")),
            )
            .finish_non_exhaustive()
    }
}

impl MartiConfig {
    /// Defaults: port 8443, 60 s timeout, 512 MiB transfer limits.
    pub fn new(host: impl Into<String>, tls: TlsConfig) -> Self {
        Self {
            host: host.into(),
            port: DEFAULT_PORT,
            tls,
            basic_auth: None,
            timeout: Duration::from_secs(60),
            max_download: 512 * 1024 * 1024,
            max_upload: 512 * 1024 * 1024,
        }
    }

    /// Uses HTTP Basic authentication in addition to TLS.
    #[must_use]
    pub fn with_basic_auth(mut self, user: impl Into<String>, password: impl Into<String>) -> Self {
        self.basic_auth = Some((user.into(), password.into()));
        self
    }
}

/// Client for TAK Server's Marti REST API.
///
/// ```no_run
/// use cottak_marti::{MartiClient, MartiConfig, NewMission};
/// use cottak_net::TlsConfig;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let tls = TlsConfig::from_files(
///     "ca.pem",
///     Some(("client.pem".as_ref(), "client.key".as_ref())),
/// )?;
/// let marti = MartiClient::new(MartiConfig::new("tak.example.com", tls)).await?;
///
/// println!("server {}", marti.version().await?);
/// marti.create_mission("RECON-1", "ALPHA-1", &NewMission::default()).await?;
/// for m in marti.missions(&Default::default()).await? {
///     println!("{}", m.name);
/// }
/// # Ok(()) }
/// ```
#[derive(Debug, Clone)]
pub struct MartiClient {
    http: reqwest::Client,
    base: Url,
    basic_auth: Option<(String, String)>,
    max_download: usize,
    pub(crate) max_upload: usize,
}

/// Query for [`MartiClient::missions`].
#[derive(Debug, Clone, Default)]
pub struct MissionQuery {
    /// Include password-protected missions.
    pub password_protected: bool,
    /// Include each mission's default role.
    pub default_role: bool,
    /// Only missions of this tool (default `public`).
    pub tool: Option<String>,
}

/// Parameters for creating a mission.
#[derive(Debug, Clone, Default)]
pub struct NewMission {
    /// Groups (channels) the mission belongs to; the server defaults to `__ANON__`.
    pub groups: Vec<String>,
    /// Description.
    pub description: Option<String>,
    /// Chat room name.
    pub chat_room: Option<String>,
    /// Base map layer.
    pub base_layer: Option<String>,
    /// Bounding box.
    pub bbox: Option<String>,
    /// Path.
    pub path: Option<String>,
    /// Classification marking.
    pub classification: Option<String>,
    /// Owning tool; the server defaults to `public`.
    pub tool: Option<String>,
    /// Password required to join.
    pub password: Option<String>,
    /// Role for new subscribers.
    pub default_role: Option<MissionRoleKind>,
    /// Expiration in epoch milliseconds.
    pub expiration: Option<i64>,
    /// Require an invitation to join.
    pub invite_only: Option<bool>,
}

/// The roles a mission can give to subscribers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionRoleKind {
    /// `MISSION_OWNER`
    Owner,
    /// `MISSION_SUBSCRIBER`
    Subscriber,
    /// `MISSION_READONLY_SUBSCRIBER`
    ReadOnlySubscriber,
}

impl MissionRoleKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "MISSION_OWNER",
            Self::Subscriber => "MISSION_SUBSCRIBER",
            Self::ReadOnlySubscriber => "MISSION_READONLY_SUBSCRIBER",
        }
    }
}

/// Item to remove from a mission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentRef {
    /// A file, by hash.
    Hash(String),
    /// A CoT UID.
    Uid(String),
}

/// Query for [`MartiClient::changes`].
#[derive(Debug, Clone, Default)]
pub struct ChangeQuery {
    /// Only changes from the last N seconds.
    pub seconds_ago: Option<u64>,
    /// Only changes after this ISO-8601 time.
    pub start: Option<String>,
    /// Only changes before this ISO-8601 time.
    pub end: Option<String>,
    /// Collapse repeated changes to the same item (server default: true).
    pub squashed: Option<bool>,
}

/// Query for [`MartiClient::client_endpoints`].
#[derive(Debug, Clone, Default)]
pub struct EndpointQuery {
    /// Only clients seen in the last N seconds.
    pub seconds_ago: Option<u64>,
    /// Only clients connected right now.
    pub currently_connected: bool,
    /// Only the most recent entry per client.
    pub most_recent_only: bool,
    /// Restrict to these groups.
    pub groups: Vec<String>,
}

fn check_name(what: &str, name: &str) -> Result<()> {
    if name.is_empty() || name.len() > 255 || name.chars().any(char::is_control) {
        return Err(Error::InvalidArgument(format!(
            "{what} must be 1-255 printable characters"
        )));
    }
    Ok(())
}

impl MartiClient {
    /// Builds the HTTPS client. Async because a TLS server-name override needs name resolution.
    pub async fn new(config: MartiConfig) -> Result<Self> {
        let tls = (*config.tls.client_config()?).clone();
        let (host, resolved): (String, Option<SocketAddr>) = match config.tls.server_name() {
            Some(name) => {
                let addr = tokio::net::lookup_host((config.host.as_str(), config.port))
                    .await
                    .map_err(|e| {
                        Error::InvalidArgument(format!("cannot resolve {}: {e}", config.host))
                    })?
                    .next()
                    .ok_or_else(|| {
                        Error::InvalidArgument(format!("cannot resolve {}", config.host))
                    })?;
                (name.to_owned(), Some(addr))
            }
            None => (config.host.clone(), None),
        };
        let host_part = if host.contains(':') && !host.starts_with('[') {
            format!("[{host}]")
        } else {
            host.clone()
        };
        let base = Url::parse(&format!("https://{host_part}:{}/", config.port))
            .map_err(|e| Error::InvalidArgument(e.to_string()))?;

        let mut builder = reqwest::Client::builder()
            .use_preconfigured_tls(tls)
            .https_only(true)
            // Redirects could send credentials to another host.
            .redirect(reqwest::redirect::Policy::none())
            .timeout(config.timeout)
            .user_agent(concat!("cottak-marti/", env!("CARGO_PKG_VERSION")));
        if let Some(addr) = resolved {
            builder = builder.resolve(&host, addr);
        }
        Ok(Self {
            http: builder.build()?,
            base,
            basic_auth: config.basic_auth,
            max_download: config.max_download,
            max_upload: config.max_upload,
        })
    }

    // ----- plumbing ---------------------------------------------------------------------------

    pub(crate) fn url(&self, segments: &[&str], query: &[(&str, &str)]) -> Result<Url> {
        let mut url = self.base.clone();
        url.path_segments_mut()
            .map_err(|()| Error::InvalidArgument("base URL cannot carry a path".into()))?
            .pop_if_empty()
            .extend(segments);
        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query);
        }
        Ok(url)
    }

    fn api_url(&self, segments: &[&str], query: &[(&str, &str)]) -> Result<Url> {
        let mut all = vec!["Marti", "api"];
        all.extend_from_slice(segments);
        self.url(&all, query)
    }

    pub(crate) fn request(&self, method: Method, url: Url) -> RequestBuilder {
        let req = self.http.request(method, url);
        match &self.basic_auth {
            Some((u, p)) => req.basic_auth(u, Some(p)),
            None => req,
        }
    }

    /// Reads a successful body (capped at `limit`) or maps the failure status.
    pub(crate) async fn body(&self, mut resp: Response, limit: usize) -> Result<Vec<u8>> {
        let status = resp.status();
        if !status.is_success() {
            let mut snippet = Vec::new();
            while snippet.len() < MAX_ERROR_BODY {
                match resp.chunk().await {
                    Ok(Some(c)) => snippet.extend_from_slice(&c),
                    _ => break,
                }
            }
            return Err(map_status(status.as_u16(), &snippet));
        }
        if resp.content_length().is_some_and(|l| l > limit as u64) {
            return Err(Error::TooLarge(limit));
        }
        let mut buf = Vec::new();
        while let Some(chunk) = resp.chunk().await? {
            if buf.len() + chunk.len() > limit {
                return Err(Error::TooLarge(limit));
            }
            buf.extend_from_slice(&chunk);
        }
        Ok(buf)
    }

    async fn api<T: DeserializeOwned>(&self, req: RequestBuilder) -> Result<Option<T>> {
        let resp = req.send().await?;
        let bytes = self.body(resp, MAX_JSON).await?;
        let env: ApiResponse<T> = serde_json::from_slice(&bytes)
            .map_err(|e| Error::BadResponse(format!("not a Marti JSON response: {e}")))?;
        Ok(env.data)
    }

    async fn api_data<T: DeserializeOwned>(&self, req: RequestBuilder) -> Result<T> {
        self.api(req)
            .await?
            .ok_or_else(|| Error::BadResponse("response has no data".into()))
    }

    async fn text(&self, req: RequestBuilder) -> Result<String> {
        let resp = req.send().await?;
        let bytes = self.body(resp, MAX_JSON).await?;
        String::from_utf8(bytes).map_err(|_| Error::BadResponse("response is not UTF-8".into()))
    }

    // ----- server -----------------------------------------------------------------------------

    /// The server version string, for example `5.2-RELEASE-39`.
    pub async fn version(&self) -> Result<String> {
        let url = self.api_url(&["version"], &[])?;
        Ok(self
            .text(self.request(Method::GET, url))
            .await?
            .trim()
            .to_owned())
    }

    /// Detailed version information (kept as JSON; its shape varies by release).
    pub async fn version_info(&self) -> Result<serde_json::Value> {
        let url = self.api_url(&["version", "info"], &[])?;
        let bytes = self
            .body(self.request(Method::GET, url).send().await?, MAX_JSON)
            .await?;
        serde_json::from_slice(&bytes).map_err(|e| Error::BadResponse(e.to_string()))
    }

    /// The id of the server node that answered.
    pub async fn node_id(&self) -> Result<String> {
        let url = self.api_url(&["node", "id"], &[])?;
        Ok(self
            .text(self.request(Method::GET, url))
            .await?
            .trim()
            .to_owned())
    }

    // ----- people and groups ------------------------------------------------------------------

    /// All contacts visible to the caller.
    pub async fn contacts(&self) -> Result<Vec<Contact>> {
        let url = self.api_url(&["contacts", "all"], &[])?;
        let bytes = self
            .body(self.request(Method::GET, url).send().await?, MAX_JSON)
            .await?;
        serde_json::from_slice(&bytes).map_err(|e| Error::BadResponse(e.to_string()))
    }

    /// Clients known to the server.
    pub async fn client_endpoints(&self, q: &EndpointQuery) -> Result<Vec<ClientEndpoint>> {
        let secs = q.seconds_ago.map(|s| s.to_string());
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(s) = &secs {
            query.push(("secAgo", s));
        }
        if q.currently_connected {
            query.push(("showCurrentlyConnectedClients", "true"));
        }
        if q.most_recent_only {
            query.push(("showMostRecentOnly", "true"));
        }
        query.extend(q.groups.iter().map(|g| ("group", g.as_str())));
        let url = self.api_url(&["clientEndPoints"], &query)?;
        self.api_data(self.request(Method::GET, url)).await
    }

    /// Groups (channels) visible to the caller.
    pub async fn groups(&self) -> Result<Vec<Group>> {
        let url = self.api_url(&["groups", "all"], &[])?;
        self.api_data(self.request(Method::GET, url)).await
    }

    // ----- missions ---------------------------------------------------------------------------

    /// Lists missions.
    pub async fn missions(&self, q: &MissionQuery) -> Result<Vec<Mission>> {
        let mut query = vec![
            (
                "passwordProtected",
                if q.password_protected {
                    "true"
                } else {
                    "false"
                },
            ),
            ("defaultRole", if q.default_role { "true" } else { "false" }),
        ];
        if let Some(t) = &q.tool {
            query.push(("tool", t));
        }
        let url = self.api_url(&["missions"], &query)?;
        self.api_data(self.request(Method::GET, url)).await
    }

    /// Fetches one mission, with its contents. `password` is needed for protected missions.
    pub async fn mission(&self, name: &str, password: Option<&str>) -> Result<Mission> {
        check_name("mission name", name)?;
        let mut query = Vec::new();
        if let Some(p) = password {
            query.push(("password", p));
        }
        let url = self.api_url(&["missions", name], &query)?;
        let missions: Vec<Mission> = self.api_data(self.request(Method::GET, url)).await?;
        missions.into_iter().next().ok_or(Error::NotFound)
    }

    /// Creates a mission. Fails with [`Error::Conflict`] if the name is taken.
    ///
    /// `creator_uid` is the UID of the client creating it.
    pub async fn create_mission(
        &self,
        name: &str,
        creator_uid: &str,
        new: &NewMission,
    ) -> Result<Mission> {
        check_name("mission name", name)?;
        let exp = new.expiration.map(|e| e.to_string());
        let invite = new.invite_only.map(|b| b.to_string());
        let mut query: Vec<(&str, &str)> = vec![("creatorUid", creator_uid)];
        query.extend(new.groups.iter().map(|g| ("group", g.as_str())));
        for (key, value) in [
            ("description", &new.description),
            ("chatRoom", &new.chat_room),
            ("baseLayer", &new.base_layer),
            ("bbox", &new.bbox),
            ("path", &new.path),
            ("classification", &new.classification),
            ("tool", &new.tool),
            ("password", &new.password),
        ] {
            if let Some(v) = value {
                query.push((key, v));
            }
        }
        if let Some(r) = new.default_role {
            query.push(("defaultRole", r.as_str()));
        }
        if let Some(e) = &exp {
            query.push(("expiration", e));
        }
        if let Some(i) = &invite {
            query.push(("inviteOnly", i));
        }
        let url = self.api_url(&["missions", name], &query)?;
        let missions: Vec<Mission> = self.api_data(self.request(Method::PUT, url)).await?;
        missions
            .into_iter()
            .next()
            .ok_or_else(|| Error::BadResponse("no mission in response".into()))
    }

    /// Deletes a mission. `deep` also deletes the files and CoT it references.
    pub async fn delete_mission(&self, name: &str, creator_uid: &str, deep: bool) -> Result<()> {
        check_name("mission name", name)?;
        let url = self.api_url(
            &["missions", name],
            &[
                ("creatorUid", creator_uid),
                ("deepDelete", if deep { "true" } else { "false" }),
            ],
        )?;
        let resp = self.request(Method::DELETE, url).send().await?;
        self.body(resp, MAX_JSON).await.map(drop)
    }

    /// Adds files (by hash) and/or CoT UIDs to a mission. Returns the updated mission.
    pub async fn add_content(
        &self,
        name: &str,
        content: &MissionContent,
        creator_uid: &str,
    ) -> Result<Mission> {
        check_name("mission name", name)?;
        if content.hashes.is_empty() && content.uids.is_empty() {
            return Err(Error::InvalidArgument(
                "add at least one hash or uid".into(),
            ));
        }
        let url = self.api_url(
            &["missions", name, "contents"],
            &[("creatorUid", creator_uid)],
        )?;
        let body =
            serde_json::to_vec(content).map_err(|e| Error::InvalidArgument(e.to_string()))?;
        let req = self
            .request(Method::PUT, url)
            .header(CONTENT_TYPE, "application/json")
            .body(body);
        let missions: Vec<Mission> = self.api_data(req).await?;
        missions
            .into_iter()
            .next()
            .ok_or_else(|| Error::BadResponse("no mission in response".into()))
    }

    /// Removes a file or UID from a mission. Returns the updated mission.
    pub async fn remove_content(
        &self,
        name: &str,
        item: &ContentRef,
        creator_uid: &str,
    ) -> Result<Mission> {
        check_name("mission name", name)?;
        let (key, value) = match item {
            ContentRef::Hash(h) => ("hash", h.as_str()),
            ContentRef::Uid(u) => ("uid", u.as_str()),
        };
        let url = self.api_url(
            &["missions", name, "contents"],
            &[(key, value), ("creatorUid", creator_uid)],
        )?;
        let missions: Vec<Mission> = self.api_data(self.request(Method::DELETE, url)).await?;
        missions
            .into_iter()
            .next()
            .ok_or_else(|| Error::BadResponse("no mission in response".into()))
    }

    /// The mission's change log.
    pub async fn changes(&self, name: &str, q: &ChangeQuery) -> Result<Vec<MissionChange>> {
        check_name("mission name", name)?;
        let secs = q.seconds_ago.map(|s| s.to_string());
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(s) = &secs {
            query.push(("secago", s));
        }
        if let Some(s) = &q.start {
            query.push(("start", s));
        }
        if let Some(e) = &q.end {
            query.push(("end", e));
        }
        if let Some(sq) = q.squashed {
            query.push(("squashed", if sq { "true" } else { "false" }));
        }
        let url = self.api_url(&["missions", name, "changes"], &query)?;
        self.api_data(self.request(Method::GET, url)).await
    }

    /// Subscribes the client `uid` to a mission.
    pub async fn subscribe(
        &self,
        name: &str,
        uid: &str,
        password: Option<&str>,
    ) -> Result<MissionSubscription> {
        check_name("mission name", name)?;
        let mut query = vec![("uid", uid)];
        if let Some(p) = password {
            query.push(("password", p));
        }
        let url = self.api_url(&["missions", name, "subscription"], &query)?;
        self.api_data(self.request(Method::PUT, url)).await
    }

    /// Removes a subscription. With `disconnect_only` the server keeps the subscription record.
    pub async fn unsubscribe(&self, name: &str, uid: &str, disconnect_only: bool) -> Result<()> {
        check_name("mission name", name)?;
        let url = self.api_url(
            &["missions", name, "subscription"],
            &[
                ("uid", uid),
                (
                    "disconnectOnly",
                    if disconnect_only { "true" } else { "false" },
                ),
            ],
        )?;
        let resp = self.request(Method::DELETE, url).send().await?;
        self.body(resp, MAX_JSON).await.map(drop)
    }

    /// UIDs of the clients subscribed to a mission.
    pub async fn subscribers(&self, name: &str) -> Result<Vec<String>> {
        check_name("mission name", name)?;
        let url = self.api_url(&["missions", name, "subscriptions"], &[])?;
        self.api_data(self.request(Method::GET, url)).await
    }

    /// The latest CoT event for each UID in a mission.
    pub async fn mission_cot(&self, name: &str) -> Result<Vec<Event>> {
        check_name("mission name", name)?;
        let url = self.api_url(&["missions", name, "cot"], &[])?;
        let xml = self.text(self.request(Method::GET, url)).await?;
        parse_events(&xml)
    }

    // ----- downloads (the upload side lives in `files`) ---------------------------------------

    /// Downloads a file from Enterprise Sync by hash.
    pub async fn download(&self, hash: &str) -> Result<Vec<u8>> {
        check_name("hash", hash)?;
        let url = self.url(&["Marti", "sync", "content"], &[("hash", hash)])?;
        let resp = self.request(Method::GET, url).send().await?;
        self.body(resp, self.max_download).await
    }
}

/// Parses a document of CoT events: `<events>` with `<event>` children, or one `<event>`.
pub(crate) fn parse_events(xml: &str) -> Result<Vec<Event>> {
    let root = XmlElement::parse(xml)?;
    match root.name.as_str() {
        "event" => Ok(vec![Event::from_element(&root)?]),
        "events" => root
            .elements()
            .map(|e| Event::from_element(e).map_err(Error::from))
            .collect(),
        other => Err(Error::BadResponse(format!(
            "expected <events>, found <{other}>"
        ))),
    }
}

fn map_status(status: u16, body: &[u8]) -> Error {
    match status {
        401 => Error::Unauthorized,
        403 => Error::Forbidden,
        404 => Error::NotFound,
        409 => Error::Conflict,
        _ => {
            let text: String = String::from_utf8_lossy(body)
                .chars()
                .filter(|c| !c.is_control())
                .take(200)
                .collect();
            Error::Status { status, body: text }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_percent_encode_path_segments_and_queries() {
        let tls = TlsConfig::new().with_pinned_sha256([0; 32]);
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let c = rt
            .block_on(MartiClient::new(MartiConfig::new("tak.example.com", tls)))
            .unwrap();
        let u = c
            .api_url(
                &["missions", "a b/c?d#e", "contents"],
                &[("creatorUid", "x&y=z")],
            )
            .unwrap();
        assert_eq!(u.path(), "/Marti/api/missions/a%20b%2Fc%3Fd%23e/contents");
        assert_eq!(u.query(), Some("creatorUid=x%26y%3Dz"));
        assert_eq!(u.host_str(), Some("tak.example.com"));
        assert_eq!(u.port_or_known_default(), Some(8443));
    }

    #[test]
    fn invalid_names_are_rejected_locally() {
        for bad in ["", "a\nb", &"x".repeat(300)] {
            assert!(check_name("n", bad).is_err());
        }
        assert!(check_name("n", "RECON 1").is_ok());
    }

    #[test]
    fn events_documents_parse() {
        let ev = r#"<event version="2.0" uid="U" type="a-f-G" how="m-g" time="2025-01-01T00:00:00Z" start="2025-01-01T00:00:00Z" stale="2025-01-01T00:05:00Z"><point lat="1" lon="2"/></event>"#;
        assert_eq!(
            parse_events(&format!("<events>{ev}{ev}</events>"))
                .unwrap()
                .len(),
            2
        );
        assert_eq!(parse_events(ev).unwrap().len(), 1);
        assert_eq!(parse_events("<events/>").unwrap().len(), 0);
        assert!(parse_events("<html/>").is_err());
        assert!(parse_events("<events><event/></events>").is_err());
    }

    #[test]
    fn status_mapping() {
        assert!(matches!(map_status(401, b""), Error::Unauthorized));
        assert!(matches!(map_status(403, b""), Error::Forbidden));
        assert!(matches!(map_status(404, b""), Error::NotFound));
        assert!(matches!(map_status(409, b""), Error::Conflict));
        assert!(matches!(
            map_status(500, b"boom"),
            Error::Status { status: 500, .. }
        ));
    }

    #[test]
    fn debug_hides_the_password() {
        let cfg = MartiConfig::new("h", TlsConfig::new()).with_basic_auth("alice", "TOPSECRET");
        assert!(!format!("{cfg:?}").contains("TOPSECRET"));
    }
}
