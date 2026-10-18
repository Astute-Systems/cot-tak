# Documentation

This page explains how the documentation is built, tested and published, and how to change it.

## Two complementary parts

| Part | Source | Built with | Purpose |
|---|---|---|---|
| **Guide** (this site) | `docs/**/*.md`, `mkdocs.yml` | MkDocs with the Material theme | Concepts, how-tos, security and compatibility notes |
| **API reference** | `///` comments in the source | `rustdoc` (`cargo doc`) | Every public item, with search and source links |

CI builds both and publishes them as **one GitHub Pages site**: the guide at the root and rustdoc
under `/api/`. The guide's [API reference](../reference/api.md) page and many in-text links point
into it.

## Why rustdoc instead of Doxygen?

Doxygen is built around C-family languages. It has no native understanding of Rust: third-party
filters exist, but they cannot represent traits, generics, lifetimes, macros, feature gates or
intra-doc links, so the result is a lossy second reference that drifts from the real API. Rustdoc:

- understands the type system and renders trait implementations, blanket impls and re-exports;
- resolves intra-doc links, and CI fails on broken ones (`-D warnings`);
- **compiles and runs the examples** in the documentation;
- is what docs.rs publishes, so the local, CI and docs.rs views are identical.

Embedding rustdoc inside MkDocs pages is also not possible (it is a complete static site with its
own search), so the best mechanism is to publish it next to the guide and link between them,
which is what the pipeline does. If a Markdown rendering of the API is ever needed, rustdoc's JSON
output is the supported input, but it is still unstable and not used here.

## Code in the guide is tested

Every `rust` code block in the guide is compiled as a **doctest**. `crates/cottak/src/guide_tests.rs`
includes the pages with `include_str!`, gated on the features each page needs, so
`cargo test --doc --all-features` fails if a snippet stops compiling or an assertion fails.
Examples that need a network are marked `no_run` (compiled, not executed); the embedded-server page
runs for real.

Write snippets using rustdoc conventions:

- A line starting with `# ` is **hidden** on the website but compiled. Use it for the `async fn`
  or `fn main` wrapper and other setup (a MkDocs hook, `docs/hooks/rustdoc_hidden.py`, strips these
  lines and tidies the info string).
- ` ```rust,no_run ` compiles without running; ` ```rust,ignore ` is skipped (use sparingly, for
  code that needs a feature the test run does not enable).
- Prefer `assert!` over `println!`, so the example proves what the text claims.
- Do not indent code blocks (for example inside tabs or admonitions): rustdoc would treat the
  indented text as a separate, broken code block. Use plain sections instead of tabs.

A typical network snippet:

````text
```rust,no_run
use cottak::net::{ClientConfig, TakClient};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let (handle, _events) = TakClient::start(ClientConfig::new("127.0.0.1:8087", "ME"))?;
handle.shutdown().await;
# Ok(()) }
```
````

## Adding a page

1. Create `docs/<section>/<name>.md`.
2. Add it to `nav` in `mkdocs.yml`.
3. If it contains `rust` blocks, add it to `crates/cottak/src/guide_tests.rs` with the Cargo
   features it needs.
4. Build locally (below).

## Building locally

```sh
python3 -m venv .venv && . .venv/bin/activate
pip install -r docs/requirements.txt

cargo doc --workspace --no-deps --all-features   # the API reference
mkdocs serve                                      # live preview at http://127.0.0.1:8000
mkdocs build --strict                             # what CI runs: any warning fails the build
```

`mkdocs serve` does not include the rustdoc output; to preview the complete site, build the guide
and copy `target/doc` to `site/api`:

```sh
mkdocs build --strict && cp -r target/doc site/api
python3 -m http.server --directory site 8000
```

## Publishing

`.github/workflows/document-ubuntu-22.04.yaml` ("Docs") runs on every push to `main` and on pull
requests (pull requests only build, to catch problems). On `main` it:

1. builds rustdoc with `-D warnings`;
2. runs the guide's doctests;
3. builds the MkDocs site with `--strict`;
4. merges both into one artifact (`/` for the guide, `/api/` for rustdoc);
5. deploys it to GitHub Pages.

!!! note "One-time repository setting"
    In *Settings, Pages*, set the source to **GitHub Actions**.

## Style

- Lead with what the reader wants to do; put reference tables after the example.
- State limits, defaults and failure behaviour explicitly.
- Be honest about what is verified: link to
  [Compatibility and limits](../concepts/compatibility.md) rather than overclaiming.
- Keep security-relevant behaviour (what is refused, what is bounded) in the guide page of the
  feature, and summarise it in the [security model](../concepts/security.md).
