//! A small, strict HTTP/1.1 implementation: just enough for the Marti API.
//!
//! * request bodies are framed by `Content-Length` only; `Transfer-Encoding`
//!   is refused, which removes request-smuggling ambiguity;
//! * headers and bodies are size-limited before they are buffered;
//! * connections are kept alive between requests.

use percent_encoding::percent_decode_str;
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const MAX_HEAD: usize = 16 * 1024;
const MAX_HEADERS: usize = 100;

/// A parsed request.
#[derive(Debug)]
pub(crate) struct Request {
    pub(crate) method: String,
    /// Path segments, percent-decoded.
    pub(crate) segments: Vec<String>,
    pub(crate) query: Vec<(String, String)>,
    /// Header names are lower-cased.
    headers: Vec<(String, String)>,
    pub(crate) body: Vec<u8>,
    pub(crate) keep_alive: bool,
}

impl Request {
    pub(crate) fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// The first value of a query parameter (names are case-sensitive).
    pub(crate) fn param(&self, name: &str) -> Option<&str> {
        self.query
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// The first value of a query parameter, ignoring case of the name.
    pub(crate) fn param_ci(&self, name: &str) -> Option<&str> {
        self.query
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    pub(crate) fn params_ci(&self, name: &str) -> Vec<&str> {
        self.query
            .iter()
            .filter(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
            .collect()
    }

    pub(crate) fn flag(&self, name: &str) -> bool {
        self.param(name)
            .is_some_and(|v| v.eq_ignore_ascii_case("true"))
    }
}

/// A response to send.
#[derive(Debug)]
pub(crate) struct Response {
    pub(crate) status: u16,
    pub(crate) content_type: &'static str,
    pub(crate) body: Vec<u8>,
}

impl Response {
    pub(crate) fn new(status: u16, content_type: &'static str, body: impl Into<Vec<u8>>) -> Self {
        Self {
            status,
            content_type,
            body: body.into(),
        }
    }

    pub(crate) fn empty(status: u16) -> Self {
        Self::new(status, "text/plain", Vec::new())
    }

    pub(crate) fn text(status: u16, body: impl Into<String>) -> Self {
        Self::new(status, "text/plain;charset=UTF-8", body.into())
    }

    pub(crate) fn json(status: u16, value: &serde_json::Value) -> Self {
        Self::new(status, "application/json", value.to_string())
    }
}

/// Why a request could not be read.
#[derive(Debug)]
pub(crate) enum ReadError {
    /// The peer closed the connection between requests.
    Closed,
    /// The request was refused; reply with this status, then close.
    Refused(u16, &'static str),
    Io,
}

impl From<io::Error> for ReadError {
    fn from(_: io::Error) -> Self {
        Self::Io
    }
}

/// Reads requests from one connection.
pub(crate) struct Reader<S> {
    io: S,
    buf: Vec<u8>,
    max_body: usize,
}

impl<S: AsyncRead + AsyncWrite + Unpin> Reader<S> {
    pub(crate) fn new(io: S, max_body: usize) -> Self {
        Self {
            io,
            buf: Vec::new(),
            max_body,
        }
    }

    async fn fill(&mut self) -> io::Result<usize> {
        let mut tmp = [0u8; 8192];
        let n = self.io.read(&mut tmp).await?;
        self.buf.extend_from_slice(&tmp[..n]);
        Ok(n)
    }

    /// Reads the next request.
    pub(crate) async fn read_request(&mut self) -> Result<Request, ReadError> {
        let head_end = loop {
            if let Some(p) = find(&self.buf, b"\r\n\r\n") {
                break p;
            }
            if self.buf.len() > MAX_HEAD {
                return Err(ReadError::Refused(431, "request head too large"));
            }
            if self.fill().await? == 0 {
                return Err(if self.buf.is_empty() {
                    ReadError::Closed
                } else {
                    ReadError::Refused(400, "truncated request")
                });
            }
        };
        if head_end > MAX_HEAD {
            return Err(ReadError::Refused(431, "request head too large"));
        }
        let head = std::str::from_utf8(&self.buf[..head_end])
            .map_err(|_| ReadError::Refused(400, "request head is not UTF-8"))?
            .to_owned();
        let parsed = parse_head(&head)?;

        if parsed.expect_continue {
            // The client waits for this before sending the body.
            self.io.write_all(b"HTTP/1.1 100 Continue\r\n\r\n").await?;
        }
        let body_len = parsed.content_length;
        if body_len > self.max_body {
            return Err(ReadError::Refused(413, "request body too large"));
        }
        let body_start = head_end + 4;
        while self.buf.len() < body_start + body_len {
            if self.fill().await? == 0 {
                return Err(ReadError::Refused(400, "truncated body"));
            }
        }
        let body = self.buf[body_start..body_start + body_len].to_vec();
        self.buf.drain(..body_start + body_len);
        Ok(Request {
            method: parsed.method,
            segments: parsed.segments,
            query: parsed.query,
            headers: parsed.headers,
            body,
            keep_alive: parsed.keep_alive,
        })
    }

    pub(crate) async fn write_response(
        &mut self,
        resp: &Response,
        keep_alive: bool,
    ) -> io::Result<()> {
        let head = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: {}\r\n\r\n",
            resp.status,
            reason(resp.status),
            resp.content_type,
            resp.body.len(),
            if keep_alive { "keep-alive" } else { "close" },
        );
        self.io.write_all(head.as_bytes()).await?;
        self.io.write_all(&resp.body).await?;
        self.io.flush().await
    }
}

struct Head {
    method: String,
    segments: Vec<String>,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    content_length: usize,
    expect_continue: bool,
    keep_alive: bool,
}

fn parse_head(head: &str) -> Result<Head, ReadError> {
    let bad = |m| ReadError::Refused(400, m);
    let mut lines = head.split("\r\n");
    let request_line = lines.next().ok_or_else(|| bad("empty request"))?;
    let mut parts = request_line.split(' ');
    let (Some(method), Some(target), Some(version), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(bad("malformed request line"));
    };
    if version != "HTTP/1.1" && version != "HTTP/1.0" {
        return Err(ReadError::Refused(505, "unsupported HTTP version"));
    }
    if method.is_empty() || !method.bytes().all(|b| b.is_ascii_uppercase()) {
        return Err(bad("malformed method"));
    }
    if !target.starts_with('/') {
        return Err(bad("target must be an absolute path"));
    }

    let mut headers = Vec::new();
    for line in lines {
        if headers.len() >= MAX_HEADERS {
            return Err(ReadError::Refused(431, "too many headers"));
        }
        let (name, value) = line
            .split_once(':')
            .ok_or_else(|| bad("malformed header"))?;
        if name.is_empty()
            || name
                .bytes()
                .any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
        {
            return Err(bad("malformed header name"));
        }
        headers.push((name.to_ascii_lowercase(), value.trim().to_owned()));
    }

    if headers.iter().any(|(k, _)| k == "transfer-encoding") {
        return Err(ReadError::Refused(
            501,
            "Transfer-Encoding is not supported; send Content-Length",
        ));
    }
    let mut content_length = None;
    for (_, v) in headers.iter().filter(|(k, _)| k == "content-length") {
        let n: usize = v.parse().map_err(|_| bad("bad Content-Length"))?;
        if content_length.is_some_and(|c| c != n) {
            return Err(bad("conflicting Content-Length headers"));
        }
        content_length = Some(n);
    }
    let connection = headers
        .iter()
        .find(|(k, _)| k == "connection")
        .map(|(_, v)| v.to_ascii_lowercase())
        .unwrap_or_default();
    let keep_alive = if version == "HTTP/1.0" {
        connection.contains("keep-alive")
    } else {
        !connection.contains("close")
    };
    let expect_continue = headers
        .iter()
        .any(|(k, v)| k == "expect" && v.eq_ignore_ascii_case("100-continue"));

    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    // Split before decoding so an encoded `/` stays inside its segment.
    let segments = path
        .split('/')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| {
            percent_decode_str(s)
                .decode_utf8()
                .map(std::borrow::Cow::into_owned)
                .map_err(|_| bad("path is not valid UTF-8"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let query = url::form_urlencoded::parse(query.as_bytes())
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    Ok(Head {
        method: method.to_owned(),
        segments,
        query,
        headers,
        content_length: content_length.unwrap_or(0),
        expect_continue,
        keep_alive,
    })
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn reason(status: u16) -> &'static str {
    match status {
        100 => "Continue",
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        413 => "Payload Too Large",
        431 => "Request Header Fields Too Large",
        501 => "Not Implemented",
        505 => "HTTP Version Not Supported",
        507 => "Insufficient Storage",
        _ => "Internal Server Error",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::duplex;

    async fn parse(raw: &[u8]) -> Result<Request, ReadError> {
        let (a, mut b) = duplex(1 << 20);
        b.write_all(raw).await.unwrap();
        drop(b);
        Reader::new(a, 1024).read_request().await
    }

    #[tokio::test]
    async fn parses_path_query_headers_and_body() {
        let r = parse(
            b"PUT /Marti/api/missions/a%20b%2Fc/contents?creatorUid=x%26y&hash=1&hash=2 HTTP/1.1\r\n\
              Host: h\r\nContent-Length: 4\r\nAuthorization: Basic QQ==\r\n\r\nBODYEXTRA",
        )
        .await
        .unwrap();
        assert_eq!(r.method, "PUT");
        assert_eq!(
            r.segments,
            ["Marti", "api", "missions", "a b/c", "contents"]
        );
        assert_eq!(r.param("creatorUid"), Some("x&y"));
        assert_eq!(r.param_ci("CREATORUID"), Some("x&y"));
        assert_eq!(r.params_ci("hash"), ["1", "2"]);
        assert_eq!(r.header("authorization"), Some("Basic QQ=="));
        assert_eq!(r.body, b"BODY");
        assert!(r.keep_alive);
    }

    #[tokio::test]
    async fn keeps_pipelined_bytes_for_the_next_request() {
        let (a, mut b) = duplex(1 << 20);
        b.write_all(b"GET /a HTTP/1.1\r\n\r\nGET /b HTTP/1.1\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        let mut r = Reader::new(a, 1024);
        assert_eq!(r.read_request().await.unwrap().segments, ["a"]);
        let second = r.read_request().await.unwrap();
        assert_eq!(second.segments, ["b"]);
        assert!(!second.keep_alive);
    }

    #[tokio::test]
    async fn smuggling_vectors_are_refused() {
        for raw in [
            &b"POST /x HTTP/1.1\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n\r\n"[..],
            b"POST /x HTTP/1.1\r\nContent-Length: 1\r\nContent-Length: 2\r\n\r\nab",
            b"POST /x HTTP/1.1\r\nContent-Length: -1\r\n\r\n",
            b"POST /x HTTP/1.1\r\nBad Header: v\r\n\r\n",
            b"GET http://evil/x HTTP/1.1\r\n\r\n",
            b"GET /x HTTP/2\r\n\r\n",
            b"get /x HTTP/1.1\r\n\r\n",
            b"GET /x HTTP/1.1 extra\r\n\r\n",
        ] {
            assert!(
                matches!(parse(raw).await, Err(ReadError::Refused(..))),
                "{}",
                String::from_utf8_lossy(raw)
            );
        }
    }

    #[tokio::test]
    async fn limits_are_enforced_before_buffering() {
        assert!(matches!(
            parse(b"POST /x HTTP/1.1\r\nContent-Length: 2000\r\n\r\n").await,
            Err(ReadError::Refused(413, _))
        ));
        let huge = format!("GET /x HTTP/1.1\r\nX: {}\r\n\r\n", "a".repeat(MAX_HEAD));
        assert!(matches!(
            parse(huge.as_bytes()).await,
            Err(ReadError::Refused(431, _))
        ));
        let many = format!(
            "GET /x HTTP/1.1\r\n{}\r\n",
            "A: b\r\n".repeat(MAX_HEADERS + 1)
        );
        assert!(matches!(
            parse(many.as_bytes()).await,
            Err(ReadError::Refused(431, _))
        ));
    }

    #[tokio::test]
    async fn truncation_and_clean_close() {
        assert!(matches!(parse(b"").await, Err(ReadError::Closed)));
        assert!(matches!(
            parse(b"GET /x HT").await,
            Err(ReadError::Refused(400, _))
        ));
        assert!(matches!(
            parse(b"POST /x HTTP/1.1\r\nContent-Length: 10\r\n\r\nab").await,
            Err(ReadError::Refused(400, _))
        ));
    }

    #[tokio::test]
    async fn invalid_utf8_in_path_is_refused() {
        assert!(matches!(
            parse(b"GET /%ff HTTP/1.1\r\n\r\n").await,
            Err(ReadError::Refused(400, _))
        ));
    }

    #[tokio::test]
    async fn responses_have_exact_framing() {
        let (a, mut b) = duplex(4096);
        let mut r = Reader::new(a, 1024);
        r.write_response(&Response::text(404, "nope"), false)
            .await
            .unwrap();
        drop(r);
        let mut got = String::new();
        b.read_to_string(&mut got).await.unwrap();
        assert_eq!(
            got,
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain;charset=UTF-8\r\nContent-Length: 4\r\nConnection: close\r\n\r\nnope"
        );
    }
}
