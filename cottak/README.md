# CoT TAK Transmission

This project sends Cursor on Target (CoT) messages to a specified UDP port.

## Prerequisites

- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)
- `uuid` crate: Add `uuid = "1.0"` to your `Cargo.toml`
- `ctrlc` crate: Add `ctrlc = "3.2"` to your `Cargo.toml`

## Building

1. Clone the repository:

    ```sh
    git clone https://github.com/Astute-Systems/cot-tak
    cd CotX/cot
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

## Running

1. Set the `COT_CALLSIGN` environment variable (optional):

    ```sh
    export COT_CALLSIGN="YourCallsign"
    ```

2. Run the application:

    ```sh
    cargo run --release
    ```

The application will send CoT messages to `239.2.3.1:6969` every second until interrupted with `Ctrl+C`.

## License

This project is licensed under the MIT License.
