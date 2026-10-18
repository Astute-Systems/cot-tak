//! Certificate signing requests built from the server's subject template.

use crate::error::{Error, Result};
use cottak_core::xml::{XmlElement, XmlNode};
use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair};

/// Subject attributes the server asks clients to use (`O`, `OU`, `C`, ...).
pub(crate) fn parse_name_entries(xml: &str) -> Result<Vec<(String, String)>> {
    let root = XmlElement::parse(xml)
        .map_err(|e| Error::BadResponse(format!("config is not XML: {e}")))?;
    let mut out = Vec::new();
    collect(&root, &mut out);
    Ok(out)
}

fn collect(el: &XmlElement, out: &mut Vec<(String, String)>) {
    // Namespaced (`ns2:nameEntry`) and plain forms both occur.
    let local = el.name.rsplit(':').next().unwrap_or(&el.name);
    if local == "nameEntry" || local == "entry" {
        if let (Some(n), Some(v)) = (el.attr("name"), el.attr("value")) {
            if !n.is_empty() && !v.is_empty() {
                out.push((n.to_owned(), v.to_owned()));
            }
        }
    }
    for child in &el.children {
        if let XmlNode::Element(c) = child {
            collect(c, out);
        }
    }
}

/// Builds a PEM CSR whose common name is `username`, followed by the
/// server-requested attributes (the first occurrence of each wins).
pub(crate) fn build_csr(
    entries: &[(String, String)],
    username: &str,
    key: &KeyPair,
) -> Result<String> {
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, username);
    for (name, ty) in [
        ("O", DnType::OrganizationName),
        ("OU", DnType::OrganizationalUnitName),
        ("C", DnType::CountryName),
        ("ST", DnType::StateOrProvinceName),
        ("L", DnType::LocalityName),
    ] {
        if let Some((_, value)) = entries.iter().find(|(n, _)| n == name) {
            dn.push(ty, value.as_str());
        }
    }
    let mut params = CertificateParams::new(Vec::<String>::new())?;
    params.distinguished_name = dn;
    Ok(params.serialize_request(key)?.pem()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_both_xml_shapes() {
        let modern = r#"<ns2:certificateConfig xmlns:ns2="http://bbn.com/marti/xml/config">
            <ns2:nameEntries>
              <ns2:nameEntry name="O" value="Acme"/>
              <ns2:nameEntry name="OU" value="Ops"/>
            </ns2:nameEntries></ns2:certificateConfig>"#;
        assert_eq!(
            parse_name_entries(modern).unwrap(),
            [("O".into(), "Acme".into()), ("OU".into(), "Ops".into())]
        );
        let legacy = r#"<config><entry name="O" value="Acme"/></config>"#;
        assert_eq!(
            parse_name_entries(legacy).unwrap(),
            [("O".into(), "Acme".into())]
        );
        assert!(parse_name_entries("<html>").is_err());
    }

    #[test]
    fn csr_is_pem_and_carries_the_subject() {
        let key = KeyPair::generate().unwrap();
        let entries = vec![
            ("O".to_owned(), "Acme".to_owned()),
            ("CN".to_owned(), "ignored".to_owned()),
        ];
        let pem = build_csr(&entries, "alice", &key).unwrap();
        assert!(pem.starts_with("-----BEGIN CERTIFICATE REQUEST-----"));

        let parsed = rcgen::CertificateSigningRequestParams::from_pem(&pem).unwrap();
        let dn = &parsed.params.distinguished_name;
        assert_eq!(
            dn.get(&DnType::CommonName),
            Some(&rcgen::DnValue::Utf8String("alice".into()))
        );
        assert_eq!(
            dn.get(&DnType::OrganizationName),
            Some(&rcgen::DnValue::Utf8String("Acme".into()))
        );
    }
}
