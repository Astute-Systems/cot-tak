//! A catalogue of CoT type descriptions, loaded from the MITRE `CoTtypes.xml`
//! file format.
//!
//! The catalogue data is **not bundled**: that file carries its own copyright
//! notice (MITRE Corporation). Load a copy you are entitled to use with
//! [`TypeCatalog::from_xml`].
//!
//! The file lists types such as `a-.-G-U-C-I` with a `full` path
//! (`Gnd/Combat/Infantry/...`) and a short `desc`. In atom entries the
//! affiliation segment is `.` and matches every affiliation.

use crate::error::{Error, Result};
use cottak_core::xml::XmlElement;
use cottak_core::CotType;
use std::collections::HashMap;

/// Largest catalogue document accepted (the real file is about 200 KiB).
const MAX_CATALOG_LEN: usize = 4 * 1024 * 1024;

/// Description of a CoT type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeInfo {
    /// The catalogue key that matched, for example `a-.-G-U-C-I`.
    pub matched: String,
    /// `true` if the type itself is listed; `false` if this is the closest listed ancestor.
    pub exact: bool,
    /// Slash-separated hierarchy path, for example `Gnd/Combat/Infantry`.
    pub full: String,
    /// Short human-readable description.
    pub desc: String,
}

/// Lookup table of type descriptions.
#[derive(Debug, Clone, Default)]
pub struct TypeCatalog {
    entries: HashMap<String, (String, String)>,
}

impl TypeCatalog {
    /// Parses a catalogue in the `CoTtypes.xml` format. Entries without a
    /// `cot` attribute are skipped.
    pub fn from_xml(xml: &str) -> Result<Self> {
        if xml.len() > MAX_CATALOG_LEN {
            return Err(Error::Catalog("document too large".into()));
        }
        let root = XmlElement::parse(xml).map_err(|e| Error::Catalog(e.to_string()))?;
        let mut entries = HashMap::new();
        for el in root.elements().filter(|e| e.name == "cot") {
            let Some(key) = el.attr("cot") else { continue };
            // First entry wins if a key is repeated.
            entries.entry(key.trim().to_owned()).or_insert_with(|| {
                (
                    el.attr("full").unwrap_or_default().trim().to_owned(),
                    el.attr("desc").unwrap_or_default().trim().to_owned(),
                )
            });
        }
        if entries.is_empty() {
            return Err(Error::Catalog("no <cot> entries found".into()));
        }
        Ok(Self { entries })
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// `true` if the catalogue has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Describes `t`, falling back to the nearest listed ancestor.
    pub fn describe(&self, t: &CotType) -> Option<TypeInfo> {
        let segments: Vec<&str> = t.segments().collect();
        for n in (1..=segments.len()).rev() {
            let exact = n == segments.len();
            let mut candidates = vec![segments[..n].join("-")];
            // Atom entries use `.` for the affiliation segment.
            if segments[0] == "a" && n >= 2 {
                let mut wild = segments[..n].to_vec();
                wild[1] = ".";
                candidates.push(wild.join("-"));
            }
            for key in candidates {
                if let Some((full, desc)) = self.entries.get(&key) {
                    return Some(TypeInfo {
                        matched: key,
                        exact,
                        full: full.clone(),
                        desc: desc.clone(),
                    });
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tiny catalogue in the real file's format (entries written for this test).
    const SAMPLE: &str = r#"<?xml version="1.0"?>
<!-- test data -->
<types>
  <cot cot="a-.-G"        full="Gnd"                   desc="Ground Track" />
  <cot cot="a-.-G-U-C"    full="Gnd/Combat"            desc="COMBAT" />
  <cot cot="a-.-G-U-C-I"  full="Gnd/Combat/Infantry"   desc="INFANTRY" />
  <cot zot="a-.-G-X"      full="typo entry"            desc="skipped" />
  <cot cot="b-m-r"        full="Bits/Map/Route"        desc="Route" />
</types>"#;

    fn t(s: &str) -> CotType {
        s.parse().unwrap()
    }

    #[test]
    fn exact_and_wildcard_affiliation_lookup() {
        let c = TypeCatalog::from_xml(SAMPLE).unwrap();
        assert_eq!(c.len(), 4, "entry without a cot attribute is skipped");

        let i = c.describe(&t("a-h-G-U-C-I")).unwrap();
        assert_eq!(
            (i.matched.as_str(), i.exact, i.desc.as_str()),
            ("a-.-G-U-C-I", true, "INFANTRY")
        );
        assert_eq!(i.full, "Gnd/Combat/Infantry");
        assert_eq!(c.describe(&t("b-m-r")).unwrap().desc, "Route");
    }

    #[test]
    fn falls_back_to_the_nearest_ancestor() {
        let c = TypeCatalog::from_xml(SAMPLE).unwrap();
        let i = c.describe(&t("a-f-G-U-C-I-Z-Q")).unwrap();
        assert_eq!((i.matched.as_str(), i.exact), ("a-.-G-U-C-I", false));
        let i = c.describe(&t("a-f-G-E-V")).unwrap();
        assert_eq!(i.matched, "a-.-G");
        assert!(c.describe(&t("t-x-c-t")).is_none());
    }

    #[test]
    fn bad_documents_are_errors() {
        for bad in ["", "<types/>", "<types><cot/></types>", "not xml"] {
            assert!(TypeCatalog::from_xml(bad).is_err(), "{bad:?}");
        }
        assert!(TypeCatalog::from_xml(&" ".repeat(MAX_CATALOG_LEN + 1)).is_err());
    }
}
