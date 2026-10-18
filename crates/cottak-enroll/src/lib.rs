//! Certificate enrollment with a TAK Server.
//!
//! TAK Server issues client certificates through its enrollment service
//! (port 8446 by default). The flow implemented here:
//!
//! 1. [`EnrollmentUrl`] – parse the `tak://com.atakmap.app/enroll?...` link or
//!    QR code an administrator provides.
//! 2. Decide how to authenticate the server. Ideally you have its CA. For a
//!    server with a self-signed certificate, obtain its fingerprint with
//!    [`cottak_net::probe_server_certificate`], confirm it with the server's
//!    operator, and pin it with [`cottak_net::TlsConfig::with_pinned_sha256`].
//!    There is deliberately no "trust anything" option.
//! 3. [`Enroller`] – fetch the subject template, generate a key pair locally
//!    (the private key never leaves this process), send a certificate signing
//!    request, and receive the signed certificate and CA chain as an
//!    [`Identity`].
//! 4. [`Identity::tls_config`] / [`Identity::with_trust`] – use the identity
//!    for mutual TLS with `cottak-net`, or [`Identity::save`] it.
//!
//! Keys are ECDSA P-256. TAK Server versions that only accept RSA requests are
//! not supported yet.
//!
//! ```
//! use cottak_enroll::EnrollmentUrl;
//!
//! let url = EnrollmentUrl::parse(
//!     "tak://com.atakmap.app/enroll?host=tak.example.com&username=alice&token=secret",
//! )?;
//! assert_eq!(url.hostname(), "tak.example.com");
//! # Ok::<(), cottak_enroll::Error>(())
//! ```

mod client;
mod csr;
pub mod error;
mod identity;
mod onboarding;
mod pem;
#[cfg(feature = "pkcs12")]
mod pkcs12;

pub use client::{Credentials, EnrollConfig, Enroller, DEFAULT_PORT};
pub use error::{Error, Result};
pub use identity::{Identity, SavedFiles};
pub use onboarding::EnrollmentUrl;
#[cfg(feature = "pkcs12")]
pub use pkcs12::DEFAULT_PASSWORD;
