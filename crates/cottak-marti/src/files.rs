//! File upload to Enterprise Sync (data packages and mission content).
//!
//! Uploads are `multipart/form-data` POSTs with the file in a part named
//! `assetfile`, as TAK Server's `UploadServlet` and `MissionPackageUploadServlet` expect.

use crate::client::MartiClient;
use crate::error::{Error, Result};
use reqwest::header::CONTENT_TYPE;
use reqwest::Method;
use serde_json::Value;

/// Options for [`MartiClient::upload_file`].
#[derive(Debug, Clone, Default)]
pub struct UploadOptions {
    /// MIME type of the file (for example `application/zip`).
    pub mime: Option<String>,
    /// Keywords to attach.
    pub keywords: Vec<String>,
    /// Owning tool (`public` by default on the server).
    pub tool: Option<String>,
    /// UID of the uploading client.
    pub creator_uid: Option<String>,
    /// Groups that may see the file.
    pub groups: Vec<String>,
    /// Free-text remarks.
    pub remarks: Option<String>,
    /// Expiration, as epoch seconds (server semantics).
    pub expiration: Option<i64>,
}

/// What the server returns for a mission-package upload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadedPackage {
    /// URL from which the package can be downloaded.
    pub url: String,
    /// Content hash, taken from the URL's `hash` parameter.
    pub hash: Option<String>,
}

/// A multipart body under construction.
struct Multipart {
    boundary: String,
    body: Vec<u8>,
}

impl Multipart {
    /// One file part named `assetfile`. The boundary is chosen so it does not occur in the data.
    fn file(filename: &str, mime: &str, data: &[u8]) -> Result<Self> {
        let safe =
            |s: &str| !s.is_empty() && !s.chars().any(|c| c.is_control() || c == '"' || c == '\\');
        if !safe(filename) || !safe(mime) {
            return Err(Error::InvalidArgument(
                "file name and MIME type must be non-empty and free of quotes, backslashes and control characters"
                    .into(),
            ));
        }
        let boundary = loop {
            let candidate = format!(
                "----cottak{:016x}{:016x}",
                fastrand::u64(..),
                fastrand::u64(..)
            );
            if !data
                .windows(candidate.len())
                .any(|w| w == candidate.as_bytes())
            {
                break candidate;
            }
        };
        let mut body = Vec::with_capacity(data.len() + 256);
        body.extend_from_slice(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"assetfile\"; filename=\"{filename}\"\r\nContent-Type: {mime}\r\n\r\n"
            )
            .as_bytes(),
        );
        body.extend_from_slice(data);
        body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
        Ok(Self { boundary, body })
    }

    fn content_type(&self) -> String {
        format!("multipart/form-data; boundary={}", self.boundary)
    }
}

impl MartiClient {
    /// Uploads a file to Enterprise Sync (`POST /Marti/sync/upload`).
    ///
    /// Returns the metadata JSON the server stores for the file, which
    /// includes its `hash` for use with [`add_content`](MartiClient::add_content).
    pub async fn upload_file(
        &self,
        name: &str,
        data: &[u8],
        opts: &UploadOptions,
    ) -> Result<Value> {
        self.check_upload_size(data.len())?;
        let mime = opts.mime.as_deref().unwrap_or("application/octet-stream");
        let part = Multipart::file(name, mime, data)?;

        let expiration = opts.expiration.map(|e| e.to_string());
        let mut query: Vec<(&str, &str)> = vec![("name", name), ("MIME", mime)];
        query.extend(opts.keywords.iter().map(|k| ("Keywords", k.as_str())));
        query.extend(opts.groups.iter().map(|g| ("Groups", g.as_str())));
        for (key, value) in [
            ("Tool", &opts.tool),
            ("CreatorUid", &opts.creator_uid),
            ("Remarks", &opts.remarks),
            ("EXPIRATION", &expiration),
        ] {
            if let Some(v) = value {
                query.push((key, v));
            }
        }

        let url = self.url(&["Marti", "sync", "upload"], &query)?;
        let req = self
            .request(Method::POST, url)
            .header(CONTENT_TYPE, part.content_type())
            .body(part.body);
        let bytes = self.body(req.send().await?, 16 * 1024 * 1024).await?;
        serde_json::from_slice(&bytes)
            .map_err(|e| Error::BadResponse(format!("upload reply is not JSON: {e}")))
    }

    /// Uploads a data package (`POST /Marti/sync/missionupload`).
    ///
    /// `creator_uid` is required by the server.
    pub async fn upload_data_package(
        &self,
        filename: &str,
        creator_uid: &str,
        data: &[u8],
        opts: &UploadOptions,
    ) -> Result<UploadedPackage> {
        self.check_upload_size(data.len())?;
        if creator_uid.is_empty() {
            return Err(Error::InvalidArgument("creator_uid is required".into()));
        }
        let mime = opts
            .mime
            .as_deref()
            .unwrap_or("application/x-zip-compressed");
        let part = Multipart::file(filename, mime, data)?;

        let mut query: Vec<(&str, &str)> = vec![
            ("filename", filename),
            ("creatorUid", creator_uid),
            ("mimetype", mime),
        ];
        query.extend(opts.keywords.iter().map(|k| ("keyword", k.as_str())));
        if let Some(t) = &opts.tool {
            query.push(("tool", t));
        }
        let groups = opts.groups.join(",");
        if !groups.is_empty() {
            query.push(("groups", &groups));
        }

        let url = self.url(&["Marti", "sync", "missionupload"], &query)?;
        let req = self
            .request(Method::POST, url)
            .header(CONTENT_TYPE, part.content_type())
            .body(part.body);
        let bytes = self.body(req.send().await?, 64 * 1024).await?;
        let url = String::from_utf8(bytes)
            .map_err(|_| Error::BadResponse("reply is not UTF-8".into()))?
            .trim()
            .to_owned();
        let hash = url::Url::parse(&url).ok().and_then(|u| {
            u.query_pairs()
                .find(|(k, _)| k == "hash")
                .map(|(_, v)| v.into_owned())
        });
        if url.is_empty() {
            return Err(Error::BadResponse("empty reply".into()));
        }
        Ok(UploadedPackage { url, hash })
    }

    fn check_upload_size(&self, len: usize) -> Result<()> {
        if len > self.max_upload {
            return Err(Error::TooLarge(self.max_upload));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multipart_layout_is_exact() {
        let m = Multipart::file("pkg.zip", "application/zip", b"DATA").unwrap();
        let text = String::from_utf8(m.body.clone()).unwrap();
        let b = &m.boundary;
        assert_eq!(
            text,
            format!(
                "--{b}\r\nContent-Disposition: form-data; name=\"assetfile\"; filename=\"pkg.zip\"\r\nContent-Type: application/zip\r\n\r\nDATA\r\n--{b}--\r\n"
            )
        );
        assert_eq!(
            m.content_type(),
            format!("multipart/form-data; boundary={b}")
        );
    }

    #[test]
    fn header_injection_is_refused() {
        for bad in ["a\"b", "a\r\nX-Evil: 1", "", "a\\b"] {
            assert!(Multipart::file(bad, "text/plain", b"").is_err(), "{bad:?}");
            assert!(Multipart::file("ok", bad, b"").is_err(), "{bad:?}");
        }
    }

    #[test]
    fn binary_data_containing_boundary_like_text_is_safe() {
        let m = Multipart::file("f", "application/octet-stream", b"----cottak--\r\n--").unwrap();
        // The closing delimiter must occur exactly once.
        let close = format!("--{}--", m.boundary);
        let hay = String::from_utf8_lossy(&m.body);
        assert_eq!(hay.matches(&close).count(), 1);
    }
}
