//! KMZ: KML in a zip archive (enabled by the `kmz` feature).
//!
//! The archive is read in memory and never extracted to disk, so entry names
//! cannot escape anywhere. Archives are untrusted input: the compressed size,
//! the number of entries and the *declared and actual* size of the KML entry
//! are all limited, which defuses zip bombs.

use crate::error::{Error, Result};
use crate::geojson::ImportLimits;
use crate::kml::{events_from_kml, events_to_kml, KmlImport, KmlOptions};
use cottak_core::Event;
use std::io::{Cursor, Read, Write};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

/// Limits applied when reading a KMZ archive.
#[derive(Debug, Clone, Copy)]
pub struct KmzLimits {
    /// Largest archive accepted, in bytes.
    pub max_archive_bytes: usize,
    /// Largest KML document accepted once decompressed, in bytes.
    pub max_kml_bytes: usize,
    /// Most entries an archive may contain.
    pub max_entries: usize,
}

impl Default for KmzLimits {
    fn default() -> Self {
        Self {
            max_archive_bytes: 64 * 1024 * 1024,
            max_kml_bytes: cottak_core::xml::MAX_XML_LEN,
            max_entries: 10_000,
        }
    }
}

fn bad(why: impl std::fmt::Display) -> Error {
    Error::InvalidKmz(why.to_string())
}

/// Chooses the KML entry: `doc.kml` at the root, else another `.kml` at the root, else the
/// first `.kml` anywhere.
fn pick_entry(archive: &ZipArchive<Cursor<&[u8]>>) -> Option<usize> {
    let mut root_kml = None;
    let mut any_kml = None;
    for i in 0..archive.len() {
        let Some(Ok(name)) = archive.name_for_index(i) else {
            continue;
        };
        let lower = name.to_ascii_lowercase();
        let is_kml = std::path::Path::new(name.as_ref())
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("kml"));
        if name.ends_with('/') || !is_kml {
            continue;
        }
        if lower == "doc.kml" {
            return Some(i);
        }
        if !name.contains('/') && root_kml.is_none() {
            root_kml = Some(i);
        }
        any_kml.get_or_insert(i);
    }
    root_kml.or(any_kml)
}

/// Converts the KML inside a KMZ archive to events. See [`events_from_kml`] for what is
/// converted and what is skipped.
pub fn events_from_kmz(
    archive: &[u8],
    limits: &ImportLimits,
    kmz: &KmzLimits,
) -> Result<KmlImport> {
    if archive.len() > kmz.max_archive_bytes {
        return Err(Error::Limit("KMZ archive too large"));
    }
    let mut zip = ZipArchive::new(Cursor::new(archive)).map_err(bad)?;
    if zip.len() > kmz.max_entries {
        return Err(Error::Limit("too many entries in KMZ archive"));
    }
    let index = pick_entry(&zip).ok_or_else(|| bad("the archive contains no .kml file"))?;
    let entry = zip.by_index(index).map_err(bad)?;
    let max = u64::try_from(kmz.max_kml_bytes).unwrap_or(u64::MAX);
    // The declared size comes from the archive, so it is checked up front *and* the actual
    // read is capped: a lying header cannot make us inflate more than `max`.
    if entry.size() > max {
        return Err(Error::Limit("KML in KMZ too large"));
    }
    let mut bytes = Vec::new();
    entry
        .take(max.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(bad)?;
    if bytes.len() as u64 > max {
        return Err(Error::Limit("KML in KMZ too large"));
    }
    let text = String::from_utf8(bytes).map_err(|_| bad("the KML is not valid UTF-8"))?;
    events_from_kml(&text, limits)
}

/// Writes events as a KMZ archive holding one deflated `doc.kml`.
pub fn events_to_kmz<'a>(
    events: impl IntoIterator<Item = &'a Event>,
    opts: &KmlOptions,
) -> Result<Vec<u8>> {
    let kml = events_to_kml(events, opts);
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    zip.start_file(
        "doc.kml",
        SimpleFileOptions::default().compression_method(CompressionMethod::Deflated),
    )
    .map_err(bad)?;
    zip.write_all(kml.as_bytes()).map_err(bad)?;
    Ok(zip.finish().map_err(bad)?.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::detail::Contact;
    use cottak_core::Point;

    fn archive(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        for (name, data) in entries {
            zip.start_file(
                *name,
                SimpleFileOptions::default().compression_method(CompressionMethod::Deflated),
            )
            .unwrap();
            zip.write_all(data).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }

    const PLACEMARK: &str = "<kml><Placemark><name>{}</name><Point><coordinates>1,2</coordinates></Point></Placemark></kml>";

    fn kml_with(name: &str) -> Vec<u8> {
        PLACEMARK.replace("{}", name).into_bytes()
    }

    fn names(import: &KmlImport) -> Vec<String> {
        import
            .events
            .iter()
            .map(|e| e.detail.get::<Contact>().unwrap().callsign.unwrap())
            .collect()
    }

    fn read(bytes: &[u8]) -> Result<KmlImport> {
        events_from_kmz(bytes, &ImportLimits::default(), &KmzLimits::default())
    }

    #[test]
    fn export_then_import_round_trips() {
        let ev = Event::new(
            "U1",
            "a-f-G-U-C".parse().unwrap(),
            Point::new(-27.5, 153.0).unwrap(),
        )
        .with(Contact::new("ALPHA"));
        let kmz = events_to_kmz([&ev], &KmlOptions::default()).unwrap();
        assert_eq!(&kmz[..2], b"PK");
        let got = read(&kmz).unwrap();
        assert_eq!((got.events.len(), got.events[0].uid.as_str()), (1, "U1"));
    }

    #[test]
    fn doc_kml_wins_then_root_kml_then_any() {
        let a = archive(&[
            ("other.kml", &kml_with("other")),
            ("files/inner.kml", &kml_with("inner")),
            ("doc.kml", &kml_with("doc")),
        ]);
        assert_eq!(names(&read(&a).unwrap()), ["doc"]);

        let b = archive(&[
            ("files/inner.kml", &kml_with("inner")),
            ("Root.KML", &kml_with("root")),
        ]);
        assert_eq!(names(&read(&b).unwrap()), ["root"]);

        let c = archive(&[
            ("images/a.png", b"png"),
            ("files/inner.kml", &kml_with("inner")),
        ]);
        assert_eq!(names(&read(&c).unwrap()), ["inner"]);
    }

    #[test]
    fn unusable_archives_are_errors_not_panics() {
        let no_kml = archive(&[("a.txt", b"x")]);
        let empty = archive(&[]);
        let not_utf8 = archive(&[("doc.kml", &[0xff, 0xfe, 0x00])]);
        let wrong_root = archive(&[("doc.kml", b"<html/>")]);
        let truncated = archive(&[("doc.kml", &kml_with("x"))]);
        let truncated = &truncated[..truncated.len() / 2];
        for (what, bytes) in [
            ("no kml", &no_kml[..]),
            ("empty archive", &empty[..]),
            ("not utf-8", &not_utf8[..]),
            ("wrong root", &wrong_root[..]),
            ("truncated", truncated),
            ("not a zip", b"<kml/>"),
            ("empty input", b""),
            ("zip magic only", b"PK\x03\x04"),
        ] {
            assert!(read(bytes).is_err(), "{what}");
        }
    }

    #[test]
    fn size_limits_defuse_zip_bombs() {
        // 20 MiB of zeros compresses to a few KiB but must not be inflated.
        let bomb = archive(&[("doc.kml", &vec![b' '; 20 * 1024 * 1024])]);
        assert!(bomb.len() < 100 * 1024, "the test archive must be small");
        assert!(matches!(read(&bomb), Err(Error::Limit(_))));

        let limits = KmzLimits {
            max_archive_bytes: 100,
            ..KmzLimits::default()
        };
        let big = archive(&[("doc.kml", &kml_with("x"))]);
        let r = events_from_kmz(&big, &ImportLimits::default(), &limits);
        assert!(matches!(r, Err(Error::Limit("KMZ archive too large"))));

        let limits = KmzLimits {
            max_entries: 2,
            ..KmzLimits::default()
        };
        let many = archive(&[
            ("a.txt", b"1"),
            ("b.txt", b"2"),
            ("c.txt", b"3"),
            ("doc.kml", &kml_with("x")),
        ]);
        let r = events_from_kmz(&many, &ImportLimits::default(), &limits);
        assert!(matches!(
            r,
            Err(Error::Limit("too many entries in KMZ archive"))
        ));
    }

    #[test]
    fn hostile_entry_names_are_only_ever_matched_not_used() {
        let a = archive(&[
            ("../../etc/passwd.kml", &kml_with("evil")),
            ("/abs/doc.kml", b"<kml/>"),
        ]);
        // Nothing is written anywhere; the first .kml is simply read.
        assert_eq!(names(&read(&a).unwrap()), ["evil"]);
    }
}
