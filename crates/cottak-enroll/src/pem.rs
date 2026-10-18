//! Certificate and key handling shared by enrollment steps.

use crate::error::{Error, Result};
use base64::alphabet;
use base64::engine::{general_purpose::GeneralPurpose, DecodePaddingMode, GeneralPurposeConfig};
use base64::Engine as _;

/// Largest certificate (DER) accepted from a server.
const MAX_CERT_DER: usize = 64 * 1024;

/// Accepts base64 with or without padding, as servers differ.
const LENIENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
);

/// Normalises certificate text from a server into well-formed PEM blocks.
///
/// The TAK Server `signClient/v2` endpoint returns certificates as bare base64
/// without `BEGIN`/`END` lines, sometimes with escaped newlines; other
/// versions return proper PEM. Both are accepted. Each certificate is
/// base64-decoded and checked to start like a DER `SEQUENCE` before being
/// re-wrapped.
pub(crate) fn normalize_certificates(text: &str) -> Result<Vec<String>> {
    let text = text.replace("\\r", "").replace("\\n", "\n");
    let bodies: Vec<String> = if text.contains("-----BEGIN") {
        text.split("-----END CERTIFICATE-----")
            .filter_map(|chunk| {
                chunk
                    .split_once("-----BEGIN CERTIFICATE-----")
                    .map(|(_, b)| b.to_owned())
            })
            .collect()
    } else {
        vec![text]
    };
    if bodies.is_empty() {
        return Err(Error::BadResponse("no certificate found".into()));
    }
    bodies.iter().map(|b| wrap_pem(b)).collect()
}

fn wrap_pem(base64_body: &str) -> Result<String> {
    let compact: String = base64_body.chars().filter(|c| !c.is_whitespace()).collect();
    let der = LENIENT
        .decode(compact.as_bytes())
        .map_err(|e| Error::BadResponse(format!("certificate is not valid base64: {e}")))?;
    if der.is_empty() || der.len() > MAX_CERT_DER || der[0] != 0x30 {
        return Err(Error::BadResponse("certificate is not DER encoded".into()));
    }
    Ok(der_to_pem(&der))
}

/// Encodes DER as a PEM `CERTIFICATE` block.
pub(crate) fn der_to_pem(der: &[u8]) -> String {
    pem_block("CERTIFICATE", der)
}

/// Encodes DER as a PEM block with the given label.
#[cfg_attr(not(feature = "pkcs12"), allow(dead_code))]
pub(crate) fn pem_block(label: &str, der: &[u8]) -> String {
    let b64 = base64::engine::general_purpose::STANDARD.encode(der);
    let mut out = format!("-----BEGIN {label}-----\n");
    for line in b64.as_bytes().chunks(64) {
        out.push_str(std::str::from_utf8(line).unwrap_or_default());
        out.push('\n');
    }
    out.push_str("-----END ");
    out.push_str(label);
    out.push_str("-----\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // A minimal DER SEQUENCE is enough for the structural checks.
    const DER: [u8; 5] = [0x30, 0x03, 0x02, 0x01, 0x05];

    fn b64() -> String {
        base64::engine::general_purpose::STANDARD.encode(DER)
    }

    #[test]
    fn bare_base64_gets_headers() {
        let pem = normalize_certificates(&b64()).unwrap();
        assert_eq!(pem.len(), 1);
        assert!(pem[0].starts_with("-----BEGIN CERTIFICATE-----\n"));
        assert!(pem[0].ends_with("-----END CERTIFICATE-----\n"));
    }

    #[test]
    fn escaped_newlines_unpadded_and_proper_pem_are_equivalent() {
        let want = normalize_certificates(&b64()).unwrap();
        let escaped = format!("{}\\n", b64());
        let unpadded = b64().trim_end_matches('=').to_owned();
        let proper = format!(
            "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----\n",
            b64()
        );
        for variant in [escaped, unpadded, proper] {
            assert_eq!(
                normalize_certificates(&variant).unwrap(),
                want,
                "{variant:?}"
            );
        }
    }

    #[test]
    fn multiple_pem_blocks_are_split() {
        let one = format!(
            "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----\n",
            b64()
        );
        assert_eq!(
            normalize_certificates(&format!("{one}{one}"))
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn garbage_is_rejected() {
        for bad in ["", "!!!not base64!!!", "AAAA", "<html>login</html>"] {
            assert!(normalize_certificates(bad).is_err(), "{bad:?}");
        }
    }
}
