# cottak-enroll

TAK Server certificate enrollment: `tak://` onboarding, CSR and `signClient/v2`.

Part of [cottak](https://github.com/Astute-Systems/cot-tak), the Rust toolkit for Cursor on Target and TAK.

## Features

- Parses `tak://com.atakmap.app/enroll` onboarding links
- Builds a CSR (ECDSA P-256 generated, or **bring your own** RSA/ECDSA/Ed25519 key) and has the server sign it
- `Identity` save/load (`0600` key files, no silent overwrite), key and certificate match check
- PKCS#12 import and ATAK-compatible export (feature `pkcs12`)
- Credentials are sent only after the server is verified; redirects are never followed

## Example

```rust
use cottak_enroll::{Credentials, EnrollmentUrl};

let url = EnrollmentUrl::parse("tak://com.atakmap.app/enroll?host=tak.example.com&username=alice&token=s3cret")?;
assert_eq!(url.hostname(), "tak.example.com");
let creds = Credentials::from_url(&url).unwrap();
assert_eq!(creds.username(), "alice");
# Ok::<(), cottak_enroll::Error>(())
```

## Documentation

- [Guide](https://astute-systems.github.io/cot-tak/guide/enrollment/) (every Rust snippet in it is compiled in CI)
- [API reference](https://astute-systems.github.io/cot-tak/api/cottak_enroll/)
- [Repository](https://github.com/Astute-Systems/cot-tak) and the [`cottak`](https://crates.io/crates/cottak) facade crate

## Licence

Licensed under either of Apache-2.0 or MIT at your option. **Note:** this crate depends on `cottak-proto`, whose bundled TAK Protocol schema is GPL-3.0, so programs that link it are a combined work; see the [licensing page](https://astute-systems.github.io/cot-tak/reference/licensing/).
