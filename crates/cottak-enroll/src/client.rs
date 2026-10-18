//! The enrollment client for the TAK Server certificate service (port 8446).

use crate::csr::{build_csr, parse_name_entries};
use crate::error::{Error, Result};
use crate::identity::Identity;
use crate::onboarding::EnrollmentUrl;
use crate::pem::normalize_certificates;
use cottak_net::TlsConfig;
use rcgen::KeyPair;
use reqwest::header::CONTENT_TYPE;
use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;
use url::Url;

/// Default port of the TAK Server certificate enrollment service.
pub const DEFAULT_PORT: u16 = 8446;

const CONFIG_PATH: &str = "/Marti/api/tls/config";
const SIGN_PATH: &str = "/Marti/api/tls/signClient/v2";
const MAX_RESPONSE: usize = 1024 * 1024;

/// Login for the enrollment service.
#[derive(Clone)]
pub struct Credentials {
    username: String,
    secret: String,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("secret", &"<redacted>")
            .finish()
    }
}

impl Credentials {
    /// A user name and password (or one-time token).
    pub fn new(username: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            secret: secret.into(),
        }
    }

    /// Credentials carried by an onboarding URL, if it has both parts.
    pub fn from_url(url: &EnrollmentUrl) -> Option<Self> {
        Some(Self::new(url.username.clone()?, url.token.clone()?))
    }

    /// The user name.
    pub fn username(&self) -> &str {
        &self.username
    }
}

/// Settings for [`Enroller`].
#[derive(Debug, Clone)]
pub struct EnrollConfig {
    /// TAK Server host name or IP address.
    pub host: String,
    /// Enrollment service port.
    pub port: u16,
    /// How to authenticate the server: a CA and/or a pinned fingerprint.
    /// Any client identity set here is ignored.
    pub trust: TlsConfig,
    /// Limit for each HTTP request.
    pub timeout: Duration,
    /// Client software version reported to the server (optional).
    pub client_version: Option<String>,
}

impl EnrollConfig {
    /// Enrollment against `host` on the default port.
    pub fn new(host: impl Into<String>, trust: TlsConfig) -> Self {
        Self {
            host: host.into(),
            port: DEFAULT_PORT,
            trust,
            timeout: Duration::from_secs(30),
            client_version: None,
        }
    }

    /// Enrollment for the server named in an onboarding URL.
    pub fn from_url(url: &EnrollmentUrl, trust: TlsConfig) -> Self {
        Self::new(url.hostname(), trust)
    }
}

/// Requests a client certificate from a TAK Server.
///
/// ```no_run
/// use cottak_enroll::{Credentials, EnrollConfig, Enroller};
/// use cottak_net::TlsConfig;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let trust = TlsConfig::from_files("server-ca.pem", None)?;
/// let enroller = Enroller::new(EnrollConfig::new("tak.example.com", trust)).await?;
/// let identity = enroller.enroll(&Credentials::new("alice", "password"), "ANDROID-1234").await?;
/// identity.save("./certs", "alice")?;
/// # Ok(()) }
/// ```
#[derive(Debug, Clone)]
pub struct Enroller {
    client: reqwest::Client,
    base: Url,
    client_version: Option<String>,
}

impl Enroller {
    /// Builds the HTTPS client. The server must be authenticated by `config.trust`.
    ///
    /// Async because a server-name override requires resolving the host.
    pub async fn new(config: EnrollConfig) -> Result<Self> {
        let (name_override, resolved) = match config.trust.server_name() {
            Some(name) => {
                let addr = tokio::net::lookup_host((config.host.as_str(), config.port))
                    .await?
                    .next()
                    .ok_or_else(|| Error::InvalidUrl(format!("cannot resolve {}", config.host)))?;
                (Some(name.to_owned()), Some(addr))
            }
            None => (None, None),
        };
        Self::build(&config, name_override.as_deref(), resolved)
    }

    fn build(
        config: &EnrollConfig,
        name: Option<&str>,
        resolved: Option<SocketAddr>,
    ) -> Result<Self> {
        let tls = (*config.trust.client_config()?).clone();
        let host = name.unwrap_or(&config.host);
        let host_part = if host.contains(':') && !host.starts_with('[') {
            format!("[{host}]")
        } else {
            host.to_owned()
        };
        let base = Url::parse(&format!("https://{host_part}:{}/", config.port))
            .map_err(|e| Error::InvalidUrl(e.to_string()))?;

        let mut builder = reqwest::Client::builder()
            .use_preconfigured_tls(tls)
            .https_only(true)
            // Never follow redirects: credentials must only go to the server we trust.
            .redirect(reqwest::redirect::Policy::none())
            .timeout(config.timeout)
            .user_agent(concat!("cottak-enroll/", env!("CARGO_PKG_VERSION")));
        if let (Some(n), Some(addr)) = (name, resolved) {
            builder = builder.resolve(n, addr);
        }
        Ok(Self {
            client: builder.build()?,
            base,
            client_version: config.client_version.clone(),
        })
    }

    /// Fetches the subject attributes (`O`, `OU`, ...) the server wants in certificate requests.
    pub async fn name_entries(&self, creds: &Credentials) -> Result<Vec<(String, String)>> {
        let url = self.url(CONFIG_PATH, &[])?;
        let resp = self
            .client
            .get(url)
            .basic_auth(&creds.username, Some(&creds.secret))
            .send()
            .await?;
        let body = read_limited(resp, true).await?;
        parse_name_entries(&body)
    }

    /// Generates an ECDSA P-256 key pair, has the server sign a request for it, and returns the
    /// identity.
    ///
    /// `client_uid` identifies this installation to the server (for example
    /// the UID the client uses in CoT).
    ///
    /// For a server that only accepts RSA, use [`enroll_with_key`](Self::enroll_with_key).
    pub async fn enroll(&self, creds: &Credentials, client_uid: &str) -> Result<Identity> {
        self.enroll_with(creds, client_uid, KeyPair::generate()?)
            .await
    }

    /// Like [`enroll`](Self::enroll), but uses a private key you already have.
    ///
    /// This is how to enroll with RSA: key generation for RSA is not built in
    /// (the TLS backend cannot generate RSA keys), so create one yourself and pass it here.
    /// `private_key_pem` must be an unencrypted PKCS#8 key (`BEGIN PRIVATE KEY`); RSA keys must
    /// be at least 2048 bits. ECDSA P-256/P-384 and Ed25519 keys work too.
    ///
    /// ```sh
    /// openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:3072 -out client.key
    /// ```
    ///
    /// A key in the older `BEGIN RSA PRIVATE KEY` (PKCS#1) form must first be converted with
    /// `openssl pkcs8 -topk8 -nocrypt`. The key never leaves this process; only a certificate
    /// request signed with it is sent.
    pub async fn enroll_with_key(
        &self,
        creds: &Credentials,
        client_uid: &str,
        private_key_pem: &str,
    ) -> Result<Identity> {
        let key = KeyPair::from_pem(private_key_pem)?;
        self.enroll_with(creds, client_uid, key).await
    }

    async fn enroll_with(
        &self,
        creds: &Credentials,
        client_uid: &str,
        key: KeyPair,
    ) -> Result<Identity> {
        let entries = match self.name_entries(creds).await {
            Ok(e) => e,
            // Very old servers have no template endpoint: use the user name alone.
            Err(Error::Status { status: 404, .. }) => Vec::new(),
            Err(e) => return Err(e),
        };

        let csr = build_csr(&entries, &creds.username, &key)?;

        let mut query = vec![("clientUid", client_uid)];
        if let Some(v) = &self.client_version {
            query.push(("version", v.as_str()));
        }
        let url = self.url(SIGN_PATH, &query)?;
        let resp = self
            .client
            .post(url)
            .basic_auth(&creds.username, Some(&creds.secret))
            .header(CONTENT_TYPE, "application/pkcs10")
            .body(csr)
            .send()
            .await?;
        let body = read_limited(resp, true).await?;

        let identity = parse_sign_response(&body, key.serialize_pem())?;
        identity.verify_key_matches()?;
        Ok(identity)
    }

    fn url(&self, path: &str, query: &[(&str, &str)]) -> Result<Url> {
        let mut url = self
            .base
            .join(path.trim_start_matches('/'))
            .map_err(|e| Error::InvalidUrl(e.to_string()))?;
        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query);
        }
        Ok(url)
    }
}

/// Reads at most [`MAX_RESPONSE`] bytes and maps HTTP errors.
async fn read_limited(mut resp: reqwest::Response, check_status: bool) -> Result<String> {
    let status = resp.status();
    let mut buf: Vec<u8> = Vec::new();
    while let Some(chunk) = resp.chunk().await? {
        if buf.len() + chunk.len() > MAX_RESPONSE {
            return Err(Error::ResponseTooLarge(MAX_RESPONSE));
        }
        buf.extend_from_slice(&chunk);
    }
    let text = String::from_utf8_lossy(&buf).into_owned();
    if check_status && !status.is_success() {
        if matches!(status.as_u16(), 401 | 403) {
            return Err(Error::Unauthorized(status.as_u16()));
        }
        let snippet: String = text.chars().filter(|c| !c.is_control()).take(200).collect();
        return Err(Error::Status {
            status: status.as_u16(),
            body: snippet,
        });
    }
    Ok(text)
}

/// Turns the `signClient/v2` JSON (`signedCert`, `ca0`, `ca1`, ...) into an [`Identity`].
fn parse_sign_response(body: &str, private_key_pem: String) -> Result<Identity> {
    let json: serde_json::Value =
        serde_json::from_str(body).map_err(|e| Error::BadResponse(format!("not JSON: {e}")))?;
    let obj = json
        .as_object()
        .ok_or_else(|| Error::BadResponse("JSON is not an object".into()))?;

    let signed = obj
        .get("signedCert")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::BadResponse("no `signedCert` in response".into()))?;
    let mut leaf = normalize_certificates(signed)?;
    let certificate_pem = leaf.remove(0);
    // Any further certificates in `signedCert` are intermediates: keep them in the chain.
    let certificate_pem = std::iter::once(certificate_pem)
        .chain(leaf)
        .collect::<String>();

    let mut ca_pem = Vec::new();
    for i in 0.. {
        let Some(v) = obj.get(&format!("ca{i}")) else {
            break;
        };
        let text = v
            .as_str()
            .ok_or_else(|| Error::BadResponse(format!("`ca{i}` is not a string")))?;
        ca_pem.extend(normalize_certificates(text)?);
    }
    Ok(Identity {
        certificate_pem,
        private_key_pem,
        ca_pem,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine as _;

    fn der_b64() -> String {
        base64::engine::general_purpose::STANDARD.encode([0x30, 0x03, 0x02, 0x01, 0x05])
    }

    #[test]
    fn sign_response_is_parsed_in_ca_order() {
        let body = serde_json::json!({
            "signedCert": der_b64(),
            "ca1": der_b64(),
            "ca0": der_b64(),
            "extra": "ignored",
        })
        .to_string();
        let id = parse_sign_response(&body, "KEY".into()).unwrap();
        assert_eq!(id.ca_pem.len(), 2);
        assert!(id.certificate_pem.contains("BEGIN CERTIFICATE"));
        assert_eq!(id.private_key_pem, "KEY");
    }

    #[test]
    fn bad_sign_responses_are_errors() {
        for bad in [
            "",
            "[]",
            "{}",
            r#"{"signedCert": 5}"#,
            r#"{"signedCert": "AAAA"}"#,
            "<html/>",
        ] {
            assert!(parse_sign_response(bad, String::new()).is_err(), "{bad}");
        }
    }

    #[test]
    fn credentials_debug_hides_the_secret() {
        let c = Credentials::new("alice", "TOPSECRET");
        assert!(!format!("{c:?}").contains("TOPSECRET"));
    }
}
