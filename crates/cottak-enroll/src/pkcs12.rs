//! PKCS#12 (`.p12`) import and export, for TAK data packages and ATAK.
//!
//! Export uses the legacy `pbeWithSHA1And3-KeyTripleDES-CBC` encryption with
//! an HMAC-SHA1 MAC, which is what ATAK's Android key store reads. Modern
//! PBES2/AES containers are not accepted by those clients.

use crate::error::{Error, Result};
use crate::identity::Identity;
use crate::pem::{der_to_pem, pem_block};
use p12_keystore::{
    Certificate, EncryptionAlgorithm, KeyStore, KeyStoreEntry, MacAlgorithm, Pkcs12ImportPolicy,
    PrivateKey, PrivateKeyChain,
};
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

/// The conventional password of TAK-issued `.p12` files.
pub const DEFAULT_PASSWORD: &str = "atakatak";

const ITERATIONS: u32 = 2048;

fn err(e: impl std::fmt::Display) -> Error {
    Error::Pkcs12(e.to_string())
}

impl Identity {
    /// Reads the client identity from a PKCS#12 file.
    ///
    /// The first private-key entry becomes the identity; its leaf certificate
    /// is the certificate, and every other certificate (in its chain or
    /// stored as a trusted entry) is returned in `ca_pem`.
    pub fn from_pkcs12(data: &[u8], password: &str) -> Result<Self> {
        let store =
            KeyStore::from_pkcs12(data, password, Pkcs12ImportPolicy::Strict).map_err(err)?;
        let (_, chain) = store
            .private_key_chain()
            .ok_or_else(|| Error::Pkcs12("no private key found".into()))?;
        let mut certs = chain.certs().iter();
        let leaf = certs
            .next()
            .ok_or_else(|| Error::Pkcs12("key has no certificate".into()))?;

        let mut ca_pem: Vec<String> = certs.map(|c| der_to_pem(c.as_der())).collect();
        for (_, entry) in store.entries() {
            if let KeyStoreEntry::Certificate(c) = entry {
                let pem = der_to_pem(c.as_der());
                if !ca_pem.contains(&pem) {
                    ca_pem.push(pem);
                }
            }
        }

        let id = Self {
            certificate_pem: der_to_pem(leaf.as_der()),
            private_key_pem: pem_block("PRIVATE KEY", chain.key().as_der()),
            ca_pem,
        };
        id.verify_key_matches()?;
        Ok(id)
    }

    /// Writes the identity as an ATAK-compatible PKCS#12 file.
    pub fn to_pkcs12(&self, password: &str, alias: &str) -> Result<Vec<u8>> {
        self.verify_key_matches()?;
        let key_der =
            PrivateKeyDer::from_pem_slice(self.private_key_pem.as_bytes()).map_err(err)?;
        let key = PrivateKey::from_der(key_der.secret_der()).map_err(err)?;

        let mut certs = Vec::new();
        for pem in std::iter::once(&self.certificate_pem).chain(self.ca_pem.iter()) {
            for der in CertificateDer::pem_slice_iter(pem.as_bytes()) {
                certs.push(Certificate::from_der(&der.map_err(err)?).map_err(err)?);
            }
        }

        let mut store = KeyStore::new();
        store.add_entry(
            alias,
            KeyStoreEntry::PrivateKeyChain(PrivateKeyChain::new(alias.as_bytes(), key, certs)),
        );
        store
            .writer(password)
            .encryption_algorithm(EncryptionAlgorithm::PbeWithShaAnd3KeyTripleDesCbc)
            .encryption_iterations(ITERATIONS)
            .mac_algorithm(MacAlgorithm::HmacSha1)
            .mac_iterations(ITERATIONS)
            .write()
            .map_err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rcgen::{BasicConstraints, CertificateParams, DnType, IsCa, Issuer, KeyPair};

    fn sample() -> Identity {
        let ca_key = KeyPair::generate().unwrap();
        let mut ca_params = CertificateParams::new(Vec::<String>::new()).unwrap();
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        ca_params
            .distinguished_name
            .push(DnType::CommonName, "Test CA");
        let ca = ca_params.self_signed(&ca_key).unwrap();
        let issuer = Issuer::new(ca_params, ca_key);

        let key = KeyPair::generate().unwrap();
        let mut leaf_params = CertificateParams::new(vec!["client".to_owned()]).unwrap();
        leaf_params
            .distinguished_name
            .push(DnType::CommonName, "client");
        let leaf = leaf_params.signed_by(&key, &issuer).unwrap();
        Identity {
            certificate_pem: leaf.pem(),
            private_key_pem: key.serialize_pem(),
            ca_pem: vec![ca.pem()],
        }
    }

    #[test]
    fn round_trips_and_uses_the_legacy_encryption_atak_needs() {
        let id = sample();
        let p12 = id.to_pkcs12(DEFAULT_PASSWORD, "TAK Client").unwrap();

        // OID 1.2.840.113549.1.12.1.3 = pbeWithSHAAnd3-KeyTripleDES-CBC
        let triple_des = [
            0x06, 0x0A, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x0C, 0x01, 0x03,
        ];
        assert!(p12.windows(triple_des.len()).any(|w| w == triple_des));

        let back = Identity::from_pkcs12(&p12, DEFAULT_PASSWORD).unwrap();
        let der = |pem: &str| {
            CertificateDer::pem_slice_iter(pem.as_bytes())
                .next()
                .unwrap()
                .unwrap()
                .to_vec()
        };
        assert_eq!(der(&back.certificate_pem), der(&id.certificate_pem));
        assert_eq!(back.ca_pem.len(), 1);
        assert_eq!(der(&back.ca_pem[0]), der(&id.ca_pem[0]));
        back.verify_key_matches().unwrap();
    }

    #[test]
    fn wrong_password_and_garbage_are_errors() {
        let p12 = sample().to_pkcs12("right", "x").unwrap();
        assert!(Identity::from_pkcs12(&p12, "wrong").is_err());
        assert!(Identity::from_pkcs12(b"not a p12", "x").is_err());
    }

    #[test]
    fn export_refuses_a_mismatched_key() {
        let mut id = sample();
        id.private_key_pem = KeyPair::generate().unwrap().serialize_pem();
        assert!(matches!(id.to_pkcs12("x", "y"), Err(Error::KeyMismatch)));
    }
}
