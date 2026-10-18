//! Minimal `multipart/form-data` reader for the `assetfile` upload part.

/// The uploaded file.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FilePart {
    pub(crate) filename: Option<String>,
    pub(crate) content_type: Option<String>,
    pub(crate) data: Vec<u8>,
}

/// Extracts the boundary from a `Content-Type` header value.
pub(crate) fn boundary(content_type: &str) -> Option<&str> {
    let (mime, params) = content_type.split_once(';')?;
    if !mime.trim().eq_ignore_ascii_case("multipart/form-data") {
        return None;
    }
    params.split(';').find_map(|p| {
        let (k, v) = p.split_once('=')?;
        k.trim()
            .eq_ignore_ascii_case("boundary")
            .then(|| v.trim().trim_matches('"'))
            .filter(|b| !b.is_empty() && b.len() <= 200)
    })
}

/// Finds the part named `assetfile` (or, failing that, the first part with a
/// file name) in `body`.
pub(crate) fn asset_file(body: &[u8], boundary: &str) -> Option<FilePart> {
    let first = format!("--{boundary}");
    // A delimiter inside the body must start a line, so `--boundary` occurring
    // in the middle of binary data is not a delimiter.
    let inner = format!("\r\n--{boundary}");
    let mut parts = Vec::new();
    let mut pos = find(body, first.as_bytes(), 0)? + first.len();
    loop {
        // After a delimiter: `--` ends the body, CRLF starts a part.
        if body.get(pos..pos + 2) == Some(b"--") {
            break;
        }
        pos = skip_crlf(body, pos)?;
        let end = find(body, inner.as_bytes(), pos.checked_sub(2)?)?;
        // An empty part has its delimiter immediately after the CRLF we skipped.
        parts.push(body.get(pos..end.max(pos))?);
        pos = end + inner.len();
    }

    let mut first_file = None;
    for part in parts {
        let Some(split) = find(part, b"\r\n\r\n", 0) else {
            continue;
        };
        let head = std::str::from_utf8(&part[..split]).ok()?;
        let data = &part[split + 4..];
        let (mut name, mut filename, mut ctype) = (None, None, None);
        for line in head.split("\r\n") {
            let Some((k, v)) = line.split_once(':') else {
                continue;
            };
            if k.trim().eq_ignore_ascii_case("content-disposition") {
                name = disposition_param(v, "name");
                filename = disposition_param(v, "filename");
            } else if k.trim().eq_ignore_ascii_case("content-type") {
                ctype = Some(v.trim().to_owned());
            }
        }
        let candidate = FilePart {
            filename,
            content_type: ctype,
            data: data.to_vec(),
        };
        if name.as_deref() == Some("assetfile") {
            return Some(candidate);
        }
        if first_file.is_none() && candidate.filename.is_some() {
            first_file = Some(candidate);
        }
    }
    first_file
}

fn disposition_param(value: &str, key: &str) -> Option<String> {
    value.split(';').skip(1).find_map(|p| {
        let (k, v) = p.split_once('=')?;
        (k.trim() == key).then(|| v.trim().trim_matches('"').to_owned())
    })
}

fn skip_crlf(body: &[u8], pos: usize) -> Option<usize> {
    (body.get(pos..pos + 2) == Some(b"\r\n")).then_some(pos + 2)
}

fn find(hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    hay.get(from..)?
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|p| p + from)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body(b: &str, parts: &[(&str, &[u8])]) -> Vec<u8> {
        let mut out = Vec::new();
        for (head, data) in parts {
            out.extend_from_slice(format!("--{b}\r\n{head}\r\n\r\n").as_bytes());
            out.extend_from_slice(data);
            out.extend_from_slice(b"\r\n");
        }
        out.extend_from_slice(format!("--{b}--\r\n").as_bytes());
        out
    }

    #[test]
    fn boundary_parsing() {
        assert_eq!(boundary("multipart/form-data; boundary=abc"), Some("abc"));
        assert_eq!(
            boundary("multipart/form-data; charset=x; boundary=\"a b\""),
            Some("a b")
        );
        assert_eq!(boundary("text/plain; boundary=abc"), None);
        assert_eq!(boundary("multipart/form-data"), None);
        assert_eq!(boundary("multipart/form-data; boundary="), None);
    }

    #[test]
    fn extracts_the_asset_file_with_binary_data() {
        let data = b"\x00\xff--B x\r\n-B\r\nrest";
        let raw = body(
            "B",
            &[
                ("Content-Disposition: form-data; name=\"other\"", b"x"),
                (
                    "Content-Disposition: form-data; name=\"assetfile\"; filename=\"p.zip\"\r\nContent-Type: application/zip",
                    data,
                ),
            ],
        );
        // `--B` in the middle of a line is data, not a delimiter.
        let part = asset_file(&raw, "B").unwrap();
        assert_eq!(part.filename.as_deref(), Some("p.zip"));
        assert_eq!(part.content_type.as_deref(), Some("application/zip"));
        assert_eq!(part.data, data);
    }

    #[test]
    fn round_trips_the_client_layout_exactly() {
        let raw = b"--b0\r\nContent-Disposition: form-data; name=\"assetfile\"; filename=\"f.txt\"\r\nContent-Type: text/plain\r\n\r\nDATA\r\n--b0--\r\n";
        assert_eq!(
            asset_file(raw, "b0").unwrap(),
            FilePart {
                filename: Some("f.txt".into()),
                content_type: Some("text/plain".into()),
                data: b"DATA".to_vec()
            }
        );
    }

    #[test]
    fn falls_back_to_the_first_file_part() {
        let raw = body(
            "B",
            &[(
                "Content-Disposition: form-data; name=\"file\"; filename=\"a\"",
                b"1",
            )],
        );
        assert_eq!(asset_file(&raw, "B").unwrap().data, b"1");
    }

    #[test]
    fn malformed_bodies_are_rejected_without_panicking() {
        for raw in [
            &b""[..],
            b"--B",
            b"--B\r\n",
            b"--B\r\nno headers end",
            b"--B\r\nContent-Disposition: x\r\n\r\ndata", // no closing delimiter
            b"--B--",
            b"garbage",
            b"--B\r\n\r\n\r\n--B--",
        ] {
            assert!(
                asset_file(raw, "B").is_none(),
                "{}",
                String::from_utf8_lossy(raw)
            );
        }
    }
}
