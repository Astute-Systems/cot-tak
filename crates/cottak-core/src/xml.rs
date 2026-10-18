//! A small, safe XML tree used as the interchange layer between raw XML
//! text and the typed CoT model.
//!
//! Parsing is done with [`quick_xml`]; serialisation is implemented here so
//! that escaping is fully under this crate's control (attribute whitespace is
//! character-escaped so that values survive a round trip).
//!
//! # Safety limits
//!
//! * `<!DOCTYPE>` is rejected (no entity expansion, no XXE).
//! * Input larger than [`MAX_XML_LEN`] is rejected.
//! * Nesting deeper than [`MAX_DEPTH`] is rejected.

use crate::error::{Error, Result};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use std::fmt::Write as _;
use std::str::FromStr;

/// Largest XML document accepted by [`XmlElement::parse`], in bytes.
pub const MAX_XML_LEN: usize = 8 * 1024 * 1024;

/// Most attributes accepted on one element.
pub const MAX_ATTRS: usize = 1024;

/// Deepest element nesting accepted by [`XmlElement::parse`].
pub const MAX_DEPTH: usize = 64;

/// A child of an [`XmlElement`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum XmlNode {
    /// A nested element.
    Element(XmlElement),
    /// Character data.
    Text(String),
}

/// A generic XML element: name, ordered attributes and ordered children.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XmlElement {
    /// Element name (prefixes are kept verbatim).
    pub name: String,
    /// Attributes in document order.
    pub attrs: Vec<(String, String)>,
    /// Children in document order.
    pub children: Vec<XmlNode>,
}

impl XmlElement {
    /// Creates an empty element.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Adds an attribute (builder style).
    #[must_use]
    pub fn with_attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attrs.push((name.into(), value.into()));
        self
    }

    /// Adds an attribute only if `value` is `Some`.
    #[must_use]
    pub fn with_opt_attr(self, name: &str, value: Option<impl Into<String>>) -> Self {
        match value {
            Some(v) => self.with_attr(name, v),
            None => self,
        }
    }

    /// Adds a child element (builder style).
    #[must_use]
    pub fn with_child(mut self, child: XmlElement) -> Self {
        self.children.push(XmlNode::Element(child));
        self
    }

    /// Adds a text child (builder style).
    #[must_use]
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.children.push(XmlNode::Text(text.into()));
        self
    }

    /// Appends the `extra` attributes left over from typed conversion.
    #[must_use]
    pub fn with_extra_attrs(mut self, extra: &[(String, String)]) -> Self {
        self.attrs.extend(extra.iter().cloned());
        self
    }

    /// Returns the value of the named attribute.
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// Iterates over child elements (skipping text).
    pub fn elements(&self) -> impl Iterator<Item = &XmlElement> {
        self.children.iter().filter_map(|n| match n {
            XmlNode::Element(e) => Some(e),
            XmlNode::Text(_) => None,
        })
    }

    /// Returns the first child element with the given name.
    pub fn child(&self, name: &str) -> Option<&XmlElement> {
        self.elements().find(|e| e.name == name)
    }

    /// Concatenation of all direct text children.
    pub fn text(&self) -> String {
        self.children
            .iter()
            .filter_map(|n| match n {
                XmlNode::Text(t) => Some(t.as_str()),
                XmlNode::Element(_) => None,
            })
            .collect()
    }

    /// `true` if the element has no attributes and no children.
    pub fn is_bare(&self) -> bool {
        self.attrs.is_empty() && self.children.is_empty()
    }

    /// Parses a document and returns its root element.
    pub fn parse(input: &str) -> Result<Self> {
        if input.len() > MAX_XML_LEN {
            return Err(Error::Limit("document too large"));
        }
        let mut reader = Reader::from_str(input);
        let mut stack: Vec<XmlElement> = Vec::new();
        let mut root: Option<XmlElement> = None;

        loop {
            match reader.read_event().map_err(Error::xml)? {
                Event::Start(e) => {
                    if stack.len() >= MAX_DEPTH {
                        return Err(Error::Limit("nesting too deep"));
                    }
                    if root.is_some() && stack.is_empty() {
                        return Err(Error::xml("multiple root elements"));
                    }
                    stack.push(element_from_start(&e)?);
                }
                Event::Empty(e) => {
                    if stack.len() >= MAX_DEPTH {
                        return Err(Error::Limit("nesting too deep"));
                    }
                    attach(&mut stack, &mut root, element_from_start(&e)?)?;
                }
                Event::End(_) => {
                    let mut el = stack
                        .pop()
                        .ok_or_else(|| Error::xml("unbalanced end tag"))?;
                    drop_ignorable_whitespace(&mut el);
                    attach(&mut stack, &mut root, el)?;
                }
                Event::Text(t) => {
                    let s = t.xml10_content();
                    check_chars(&s)?;
                    push_text(&mut stack, &s);
                }
                Event::CData(t) => {
                    let s = t.xml10_content();
                    check_chars(&s)?;
                    push_text(&mut stack, &s);
                }
                Event::GeneralRef(r) => {
                    let s = resolve_ref(&r)?;
                    check_chars(&s)?;
                    push_text(&mut stack, &s);
                }
                Event::DocType(_) => return Err(Error::Forbidden("DOCTYPE declaration")),
                Event::Eof => break,
                Event::Comment(_) | Event::Decl(_) | Event::PI(_) => {}
            }
        }

        if !stack.is_empty() {
            return Err(Error::xml("unexpected end of document"));
        }
        root.ok_or_else(|| Error::xml("empty document"))
    }

    /// Checks that the whole subtree can be written as well-formed XML: valid names, and no
    /// characters XML 1.0 forbids. [`to_xml`](Self::to_xml) silently drops such characters, so
    /// call this first when the tree was not produced by [`parse`](Self::parse).
    pub fn validate(&self) -> Result<()> {
        check_name(&self.name)?;
        for (k, v) in &self.attrs {
            check_name(k)?;
            check_chars(v)?;
        }
        for c in &self.children {
            match c {
                XmlNode::Element(e) => e.validate()?,
                XmlNode::Text(t) => check_chars(t)?,
            }
        }
        Ok(())
    }

    /// Serialises the element (and its subtree) to a compact XML string.
    pub fn to_xml(&self) -> String {
        let mut out = String::new();
        self.write_into(&mut out);
        out
    }

    fn write_into(&self, out: &mut String) {
        out.push('<');
        out.push_str(&self.name);
        for (k, v) in &self.attrs {
            out.push(' ');
            out.push_str(k);
            out.push_str("=\"");
            escape_attr(v, out);
            out.push('"');
        }
        if self.children.is_empty() {
            out.push_str("/>");
            return;
        }
        out.push('>');
        for c in &self.children {
            match c {
                XmlNode::Element(e) => e.write_into(out),
                XmlNode::Text(t) => escape_text(t, out),
            }
        }
        out.push_str("</");
        out.push_str(&self.name);
        out.push('>');
    }
}

fn element_from_start(e: &BytesStart<'_>) -> Result<XmlElement> {
    let name = e.name().as_ref().to_owned();
    check_name(&name)?;
    let mut attrs: Vec<(String, String)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    // quick-xml's own duplicate check is quadratic in the attribute count (RUSTSEC-2026-0194
    // before 0.41), so it is disabled in favour of a linear check and a hard cap.
    for a in e.attributes().with_checks(false) {
        let a = a.map_err(Error::xml)?;
        if attrs.len() >= MAX_ATTRS {
            return Err(Error::Limit("too many attributes on one element"));
        }
        let key = a.key.as_ref().to_owned();
        let value = a
            .normalized_value(XmlVersion::Implicit1_0)
            .map_err(Error::xml)?
            .into_owned();
        check_name(&key)?;
        check_chars(&value)?;
        if !seen.insert(key.clone()) {
            return Err(Error::xml("duplicate attribute"));
        }
        attrs.push((key, value));
    }
    Ok(XmlElement {
        name,
        attrs,
        children: Vec::new(),
    })
}

fn attach(stack: &mut [XmlElement], root: &mut Option<XmlElement>, el: XmlElement) -> Result<()> {
    if let Some(parent) = stack.last_mut() {
        parent.children.push(XmlNode::Element(el));
    } else if root.is_none() {
        *root = Some(el);
    } else {
        return Err(Error::xml("multiple root elements"));
    }
    Ok(())
}

fn push_text(stack: &mut [XmlElement], s: &str) {
    // Text outside the root element (e.g. whitespace) is ignored.
    let Some(parent) = stack.last_mut() else {
        return;
    };
    if let Some(XmlNode::Text(prev)) = parent.children.last_mut() {
        prev.push_str(s);
    } else {
        parent.children.push(XmlNode::Text(s.to_owned()));
    }
}

/// Elements with element children drop whitespace-only text (pretty-printing).
fn drop_ignorable_whitespace(el: &mut XmlElement) {
    if el.elements().next().is_some() {
        el.children.retain(|n| match n {
            XmlNode::Text(t) => !t.trim().is_empty(),
            XmlNode::Element(_) => true,
        });
    }
}

fn resolve_ref(r: &quick_xml::events::BytesRef<'_>) -> Result<String> {
    if let Some(c) = r.resolve_char_ref().map_err(Error::xml)? {
        return Ok(c.to_string());
    }
    let name: &str = r;
    match name {
        "amp" => Ok("&".into()),
        "lt" => Ok("<".into()),
        "gt" => Ok(">".into()),
        "quot" => Ok("\"".into()),
        "apos" => Ok("'".into()),
        _ => Err(Error::Forbidden("custom entity reference")),
    }
}

fn escape_text(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '\r' => out.push_str("&#13;"),
            c if is_xml_char(c) => out.push(c),
            _ => {}
        }
    }
}

fn escape_attr(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\n' | '\r' | '\t' => {
                let _ = write!(out, "&#{};", c as u32);
            }
            c if is_xml_char(c) => out.push(c),
            _ => {}
        }
    }
}

fn is_name_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c == ':' || (!c.is_ascii() && c.is_alphabetic())
}

fn is_name_char(c: char) -> bool {
    is_name_start(c)
        || c.is_ascii_digit()
        || c == '-'
        || c == '.'
        || (!c.is_ascii() && c.is_alphanumeric())
}

/// Element and attribute names must be usable as XML names, or the writer would emit
/// documents that cannot be read back.
fn check_name(s: &str) -> Result<()> {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if is_name_start(c) && chars.all(is_name_char) => Ok(()),
        _ => Err(Error::Forbidden("invalid XML name")),
    }
}

/// Rejects characters that XML 1.0 forbids (control characters other than tab, CR and LF,
/// surrogates and the non-characters). A conforming parser must treat them as fatal, and
/// accepting them would make output differ from input, since the writer drops them.
pub(crate) fn check_text(s: &str) -> Result<()> {
    check_chars(s)
}

fn check_chars(s: &str) -> Result<()> {
    if s.chars().all(is_xml_char) {
        Ok(())
    } else {
        Err(Error::Forbidden("character not allowed in XML 1.0"))
    }
}

/// XML 1.0 `Char` production; other code points are dropped on output.
fn is_xml_char(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | '\u{20}'..='\u{D7FF}' | '\u{E000}'..='\u{FFFD}' | '\u{10000}'..='\u{10FFFF}')
}

/// Mutable view over an element's attributes used by typed conversions.
///
/// Attributes are *taken* as they are understood; whatever remains is kept in
/// the typed value's `extra` field so nothing is lost on a round trip.
#[derive(Debug)]
pub struct Attrs(Vec<(String, String)>);

impl Attrs {
    /// Copies the attributes of `el`.
    pub fn new(el: &XmlElement) -> Self {
        Self(el.attrs.clone())
    }

    /// Removes and returns a string attribute.
    pub fn take(&mut self, name: &str) -> Option<String> {
        let i = self.0.iter().position(|(k, _)| k == name)?;
        Some(self.0.remove(i).1)
    }

    /// Removes and returns an attribute that parses as `T`.
    ///
    /// If the value does not parse it is left in place (and so ends up in
    /// `extra`), preserving it verbatim.
    pub fn take_parsed<T: FromStr>(&mut self, name: &str) -> Option<T> {
        let i = self.0.iter().position(|(k, _)| k == name)?;
        let v = self.0[i].1.parse::<T>().ok()?;
        self.0.remove(i);
        Some(v)
    }

    /// Consumes the view, returning the attributes that were not taken.
    pub fn into_extra(self) -> Vec<(String, String)> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_write_round_trip() {
        let src = r#"<a x="1"><b y="&amp;&lt;">hi &amp; bye</b><c/></a>"#;
        let el = XmlElement::parse(src).unwrap();
        assert_eq!(el.name, "a");
        assert_eq!(el.child("b").unwrap().attr("y"), Some("&<"));
        assert_eq!(el.child("b").unwrap().text(), "hi & bye");
        assert_eq!(XmlElement::parse(&el.to_xml()).unwrap(), el);
    }

    #[test]
    fn attribute_whitespace_survives() {
        let el = XmlElement::new("r").with_attr("v", "line1\nline2\ttab");
        let back = XmlElement::parse(&el.to_xml()).unwrap();
        assert_eq!(back.attr("v"), Some("line1\nline2\ttab"));
    }

    #[test]
    fn rejects_doctype_and_custom_entities() {
        let xxe = r#"<!DOCTYPE e [<!ENTITY x SYSTEM "file:///etc/passwd">]><e>&x;</e>"#;
        assert!(matches!(XmlElement::parse(xxe), Err(Error::Forbidden(_))));
    }

    #[test]
    fn rejects_deep_nesting() {
        let deep = "<a>".repeat(MAX_DEPTH + 1) + &"</a>".repeat(MAX_DEPTH + 1);
        assert!(matches!(XmlElement::parse(&deep), Err(Error::Limit(_))));
    }

    #[test]
    fn rejects_malformed() {
        for bad in ["", "<a>", "<a></b>", "<a/><b/>", "plain text"] {
            assert!(XmlElement::parse(bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn pretty_printed_whitespace_is_ignored() {
        let el = XmlElement::parse("<a>\n  <b/>\n</a>").unwrap();
        assert_eq!(el.children.len(), 1);
    }

    #[test]
    fn rejects_characters_xml_forbids() {
        // Found by fuzzing: these were accepted but could not be written back.
        for bad in [
            "<a x=\"\u{0}\"/>",
            "<a>\u{c}</a>",
            "<a x=\"&#1;\"/>",
            "<a>&#0;</a>",
            "<a\u{1}b/>",
            "<a><![CDATA[\u{7}]]></a>",
        ] {
            assert!(XmlElement::parse(bad).is_err(), "{bad:?}");
        }
        assert!(XmlElement::parse("<a x=\"\t\n&#9;&#10;\">\r\n</a>").is_ok());
    }

    #[test]
    fn rejects_names_that_cannot_be_written_back() {
        // Found by fuzzing: an empty element name was accepted but produced `<` on output.
        for bad in [
            "<\n\t/>",
            "< />",
            "<a h$ae=\"1\"/>",
            "<1a/>",
            "<a -x=\"1\"/>",
        ] {
            assert!(XmlElement::parse(bad).is_err(), "{bad:?}");
        }
        for good in [
            "<ns:tag a.b=\"1\" _c-d=\"2\"/>",
            "<__group name=\"x\"/>",
            "<caf\u{e9} \u{e9}t\u{e9}=\"1\"/>",
        ] {
            let el = XmlElement::parse(good).unwrap();
            assert_eq!(XmlElement::parse(&el.to_xml()).unwrap(), el);
        }
    }

    #[test]
    fn validate_catches_trees_the_writer_would_alter() {
        assert!(XmlElement::new("a").with_attr("x", "ok").validate().is_ok());
        assert!(XmlElement::new("a")
            .with_attr("x", "bad\u{0}")
            .validate()
            .is_err());
        assert!(XmlElement::new("a b").validate().is_err());
        assert!(XmlElement::new("a")
            .with_child(XmlElement::new("b").with_text("\u{c}"))
            .validate()
            .is_err());
    }

    #[test]
    fn attribute_count_is_capped_and_duplicates_rejected() {
        // Guards against quadratic duplicate checking (RUSTSEC-2026-0194 in quick-xml < 0.41).
        let many = |n: usize| {
            let attrs: String = (0..n).fold(String::new(), |mut s, i| {
                write!(s, " a{i}=\"1\"").unwrap();
                s
            });
            format!("<e{attrs}/>")
        };
        assert_eq!(
            XmlElement::parse(&many(MAX_ATTRS)).unwrap().attrs.len(),
            MAX_ATTRS
        );
        let started = std::time::Instant::now();
        assert!(matches!(
            XmlElement::parse(&many(100_000)),
            Err(Error::Limit(_))
        ));
        assert!(
            started.elapsed() < std::time::Duration::from_secs(2),
            "rejection must be fast"
        );
        assert!(XmlElement::parse(r#"<e a="1" a="2"/>"#).is_err());
    }

    #[test]
    fn line_endings_and_whitespace_survive_via_character_references() {
        // quick-xml 0.42 normalises line endings and attribute whitespace as the XML spec requires;
        // the writer escapes them so values still round-trip exactly.
        let el = XmlElement::new("a")
            .with_attr("v", "x\r\ny\tz\n")
            .with_text("l1\r\nl2\rl3");
        assert_eq!(XmlElement::parse(&el.to_xml()).unwrap(), el);
    }
}
