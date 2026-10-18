//! The result of enrollment: a client certificate, its key and the server's CA.

use crate::error::{Error, Result};
use cottak_net::TlsConfig;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

/// A client identity obtained from a TAK Server.
#[derive(Clone)]
pub struct Identity {
    /// The client certificate (PEM).
    pub certificate_pem: String,
    /// The matching private key (PEM, PKCS#8). Keep this secret.
    pub private_key_pem: String,
    /// CA certificates the server returned (PEM), used to trust the server.
    pub ca_pem: Vec<String>,
}

impl fmt::Debug for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Identity")
            .field("certificate_pem_bytes", &self.certificate_pem.len())
            .field("private_key_pem", &"<redacted>")
            .field("ca_certificates", &self.ca_pem.len())
            .finish()
    }
}

/// Where [`Identity::save`] wrote its files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedFiles {
    /// Client certificate.
    pub certificate: PathBuf,
    /// Private key (mode 0600 on Unix).
    pub private_key: PathBuf,
    /// CA bundle, if the server returned any.
    pub ca: Option<PathBuf>,
}

impl Identity {
    /// Checks that the private key belongs to the certificate.
    pub fn verify_key_matches(&self) -> Result<()> {
        let chain = CertificateDer::pem_slice_iter(self.certificate_pem.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Error::BadResponse(format!("bad certificate PEM: {e}")))?;
        let key = PrivateKeyDer::from_pem_slice(self.private_key_pem.as_bytes())
            .map_err(|e| Error::BadResponse(format!("bad key PEM: {e}")))?;
        let signing = rustls::crypto::ring::sign::any_supported_type(&key)
            .map_err(|e| Error::BadResponse(format!("unsupported key type: {e}")))?;
        rustls::sign::CertifiedKey::new(chain, signing)
            .keys_match()
            .map_err(|_| Error::KeyMismatch)
    }

    /// Adds this identity to `trust`, which says how to authenticate the
    /// server (a CA, a pinned fingerprint, or both).
    pub fn with_trust(&self, trust: TlsConfig) -> TlsConfig {
        trust.with_client_identity_pem(self.certificate_pem.clone(), self.private_key_pem.clone())
    }

    /// A ready-to-use [`TlsConfig`] that trusts the CA(s) the server returned.
    ///
    /// Fails if the server returned no CA; use [`with_trust`](Self::with_trust)
    /// with your own trust settings in that case.
    pub fn tls_config(&self) -> Result<TlsConfig> {
        if self.ca_pem.is_empty() {
            return Err(Error::BadResponse(
                "the server returned no CA certificate; use `with_trust` to supply trust".into(),
            ));
        }
        let trust = self
            .ca_pem
            .iter()
            .fold(TlsConfig::new(), TlsConfig::with_ca_pem);
        let cfg = self.with_trust(trust);
        cfg.validate()?;
        Ok(cfg)
    }

    /// Writes `<stem>.pem`, `<stem>.key` and (if any) `<stem>-ca.pem` into `dir`.
    ///
    /// The key file is created with owner-only permissions on Unix and an
    /// existing file is never overwritten silently: the call fails instead.
    pub fn save(&self, dir: impl AsRef<Path>, stem: &str) -> Result<SavedFiles> {
        let valid = !stem.is_empty()
            && stem
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
            && !stem.starts_with('.');
        if !valid {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "file stem must be alphanumeric, '-', '_' or '.' and not start with '.'",
            )));
        }
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir)?;

        let certificate = dir.join(format!("{stem}.pem"));
        let private_key = dir.join(format!("{stem}.key"));
        write_new(&private_key, self.private_key_pem.as_bytes(), true)?;
        write_new(&certificate, self.certificate_pem.as_bytes(), false)?;
        let ca = if self.ca_pem.is_empty() {
            None
        } else {
            let path = dir.join(format!("{stem}-ca.pem"));
            write_new(&path, self.ca_pem.concat().as_bytes(), false)?;
            Some(path)
        };
        Ok(SavedFiles {
            certificate,
            private_key,
            ca,
        })
    }

    /// Loads an identity from PEM files written by [`save`](Self::save) or equivalent.
    pub fn load(
        certificate: impl AsRef<Path>,
        private_key: impl AsRef<Path>,
        ca: Option<&Path>,
    ) -> Result<Self> {
        let ca_pem = match ca {
            Some(p) => vec![std::fs::read_to_string(p)?],
            None => Vec::new(),
        };
        let id = Self {
            certificate_pem: std::fs::read_to_string(certificate)?,
            private_key_pem: std::fs::read_to_string(private_key)?,
            ca_pem,
        };
        id.verify_key_matches()?;
        Ok(id)
    }
}

fn write_new(path: &Path, bytes: &[u8], secret: bool) -> Result<()> {
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    if secret {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    #[cfg(not(unix))]
    let _ = secret;
    let mut f = opts.open(path)?;
    f.write_all(bytes)?;
    f.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rcgen::{CertificateParams, KeyPair};

    fn identity() -> Identity {
        let key = KeyPair::generate().unwrap();
        let cert = CertificateParams::new(vec!["c".to_owned()])
            .unwrap()
            .self_signed(&key)
            .unwrap();
        Identity {
            certificate_pem: cert.pem(),
            private_key_pem: key.serialize_pem(),
            ca_pem: vec![cert.pem()],
        }
    }

    fn temp_dir(tag: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "cottak-enroll-{tag}-{}-{nanos}",
            std::process::id()
        ))
    }

    #[test]
    fn save_and_load_round_trip_with_private_key_permissions() {
        let dir = temp_dir("save");
        let id = identity();
        let files = id.save(&dir, "alice").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&files.private_key)
                .unwrap()
                .permissions()
                .mode()
                & 0o777;
            assert_eq!(mode, 0o600, "private key must not be readable by others");
        }
        let back =
            Identity::load(&files.certificate, &files.private_key, files.ca.as_deref()).unwrap();
        assert_eq!(back.certificate_pem, id.certificate_pem);
        assert_eq!(back.ca_pem.len(), 1);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn save_never_overwrites_and_rejects_path_tricks() {
        let dir = temp_dir("overwrite");
        let id = identity();
        id.save(&dir, "alice").unwrap();
        assert!(
            id.save(&dir, "alice").is_err(),
            "existing files must not be replaced"
        );
        for bad in ["", "../escape", "a/b", ".hidden", "a b"] {
            assert!(id.save(&dir, bad).is_err(), "{bad:?}");
        }
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn mismatched_key_is_detected_and_debug_hides_the_key() {
        let mut id = identity();
        assert!(id.verify_key_matches().is_ok());
        assert!(!format!("{id:?}").contains("PRIVATE KEY"));
        id.private_key_pem = KeyPair::generate().unwrap().serialize_pem();
        assert!(matches!(id.verify_key_matches(), Err(Error::KeyMismatch)));
    }

    #[test]
    fn tls_config_needs_a_ca_unless_trust_is_supplied() {
        let mut id = identity();
        id.ca_pem.clear();
        assert!(id.tls_config().is_err());
        let cfg = id.with_trust(TlsConfig::new().with_pinned_sha256([1; 32]));
        assert!(cfg.validate().is_ok());
    }
}
