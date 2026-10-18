//! `tak://` onboarding URLs.
//!
//! TAK Server administrators hand out QR codes that encode URLs such as
//! `tak://com.atakmap.app/enroll?host=tak.example.com&username=alice&token=s3cret`.

use crate::error::{Error, Result};
use std::fmt;
use url::Url;

const SCHEME: &str = "tak";
const APP: &str = "com.atakmap.app";

/// A parsed enrollment URL.
#[derive(Clone, PartialEq, Eq)]
pub struct EnrollmentUrl {
    /// TAK Server host name (or `host:port`).
    pub host: String,
    /// User name, if the URL carries one.
    pub username: Option<String>,
    /// Password or one-time token, if the URL carries one.
    pub token: Option<String>,
}

impl fmt::Debug for EnrollmentUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnrollmentUrl")
            .field("host", &self.host)
            .field("username", &self.username)
            .field("token", &self.token.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

impl EnrollmentUrl {
    /// Parses `tak://com.atakmap.app/enroll?host=...&username=...&token=...`.
    pub fn parse(input: &str) -> Result<Self> {
        let bad = |why: &str| Error::InvalidUrl(why.to_owned());
        let url = Url::parse(input.trim()).map_err(|e| Error::InvalidUrl(e.to_string()))?;
        if url.scheme() != SCHEME {
            return Err(bad("scheme must be tak://"));
        }
        if url.host_str() != Some(APP) {
            return Err(bad("not a com.atakmap.app URL"));
        }
        if url.path().trim_end_matches('/') != "/enroll" {
            return Err(bad("path must be /enroll"));
        }

        let (mut host, mut username, mut token) = (None, None, None);
        for (k, v) in url.query_pairs() {
            let slot = match k.as_ref() {
                "host" => &mut host,
                "username" => &mut username,
                "token" => &mut token,
                _ => continue,
            };
            if slot.replace(v.into_owned()).is_some() {
                return Err(Error::InvalidUrl(format!("duplicate `{k}` parameter")));
            }
        }

        let host = host
            .filter(|h| !h.is_empty())
            .ok_or_else(|| bad("missing host"))?;
        if !is_plausible_host(&host) {
            return Err(bad("host contains invalid characters"));
        }
        Ok(Self {
            host,
            username: username.filter(|u| !u.is_empty()),
            token: token.filter(|t| !t.is_empty()),
        })
    }

    /// The host name without any `:port` suffix.
    pub fn hostname(&self) -> &str {
        match self.host.rsplit_once(':') {
            Some((h, p)) if p.parse::<u16>().is_ok() && !h.contains(':') => h,
            _ => &self.host,
        }
    }
}

/// Letters, digits, `.`, `-`, `_`, `:` and brackets (for IPv6): enough for
/// hosts, and strict enough that the value cannot smuggle URL syntax.
fn is_plausible_host(h: &str) -> bool {
    h.len() <= 253
        && h.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | ':' | '[' | ']'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_typical_qr_url() {
        let u = EnrollmentUrl::parse(
            "tak://com.atakmap.app/enroll?host=tak.example.com&username=alice&token=s%3Dcr%26t",
        )
        .unwrap();
        assert_eq!(u.host, "tak.example.com");
        assert_eq!(u.username.as_deref(), Some("alice"));
        assert_eq!(u.token.as_deref(), Some("s=cr&t"), "percent-decoded");
        assert_eq!(u.hostname(), "tak.example.com");
    }

    #[test]
    fn host_only_is_allowed_and_port_is_split() {
        let u = EnrollmentUrl::parse("tak://com.atakmap.app/enroll?host=10.0.0.5:8446").unwrap();
        assert_eq!((&u.username, &u.token), (&None, &None));
        assert_eq!(u.hostname(), "10.0.0.5");
    }

    #[test]
    fn rejects_bad_urls() {
        for bad in [
            "https://com.atakmap.app/enroll?host=x",
            "tak://evil.example/enroll?host=x",
            "tak://com.atakmap.app/other?host=x",
            "tak://com.atakmap.app/enroll",
            "tak://com.atakmap.app/enroll?host=",
            "tak://com.atakmap.app/enroll?host=a&host=b",
            "tak://com.atakmap.app/enroll?host=evil.com/path%3Fx",
            "tak://com.atakmap.app/enroll?host=a b",
            "not a url",
        ] {
            assert!(EnrollmentUrl::parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn debug_hides_the_token() {
        let u =
            EnrollmentUrl::parse("tak://com.atakmap.app/enroll?host=h&token=TOPSECRET").unwrap();
        assert!(!format!("{u:?}").contains("TOPSECRET"));
    }
}
