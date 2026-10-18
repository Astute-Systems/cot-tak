//! Obtain a client certificate from a TAK Server using a `tak://` onboarding link.
//!
//! The server is authenticated either by a CA file or by a pinned certificate fingerprint;
//! there is deliberately no way to skip verification. To learn a self-signed server's
//! fingerprint, run with `--probe` and compare it with the operator's published value.
//!
//! Pass `--key <pkcs8.pem>` to enroll with a key you made yourself (needed for RSA-only servers;
//! `openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:3072 -out client.key`).
//!
//! ```text
//! cargo run -p cottak --example enroll --features enroll -- --probe tak.example.com:8446
//! cargo run -p cottak --example enroll --features enroll -- 'tak://com.atakmap.app/enroll?host=tak.example.com&username=alice&token=…' --pin AB:CD:… ./certs
//! ```

use cottak::enroll::{Credentials, EnrollConfig, Enroller, EnrollmentUrl};
use cottak::net::{probe_server_certificate, TlsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.first().map(String::as_str) == Some("--probe") {
        let addr = args.get(1).ok_or("usage: enroll --probe <host:port>")?;
        let cert = probe_server_certificate(addr).await?;
        println!(
            "SHA-256 fingerprint (NOT yet verified): {}",
            cert.fingerprint()
        );
        println!(
            "Compare it with the value published by the server's operator before using --pin."
        );
        return Ok(());
    }

    let url = EnrollmentUrl::parse(
        args.first()
            .ok_or("usage: enroll <tak://…> (--pin <fp> | --ca <ca.pem>) <out-dir>")?,
    )?;
    let creds = Credentials::from_url(&url).ok_or("the link must carry a username and token")?;
    let mut trust = TlsConfig::new();
    let mut out_dir = String::from(".");
    let mut key_pem: Option<String> = None;
    let mut rest = args[1..].iter();
    while let Some(a) = rest.next() {
        match a.as_str() {
            "--pin" => {
                trust = trust
                    .with_pinned_sha256_hex(rest.next().ok_or("--pin needs a fingerprint")?)?;
            }
            "--ca" => {
                trust = trust.with_ca_pem(std::fs::read(rest.next().ok_or("--ca needs a file")?)?);
            }
            "--key" => {
                key_pem = Some(std::fs::read_to_string(
                    rest.next().ok_or("--key needs a file")?,
                )?);
            }
            dir => out_dir = dir.to_owned(),
        }
    }

    let enroller = Enroller::new(EnrollConfig::from_url(&url, trust)).await?;
    let identity = match &key_pem {
        Some(key) => {
            enroller
                .enroll_with_key(&creds, "cottak-example", key)
                .await?
        }
        None => enroller.enroll(&creds, "cottak-example").await?,
    };
    let files = identity.save(&out_dir, creds.username())?;
    println!("certificate: {}", files.certificate.display());
    println!(
        "private key: {} (keep it secret)",
        files.private_key.display()
    );
    if let Some(ca) = files.ca {
        println!("server CA:   {}", ca.display());
    }
    Ok(())
}
