//! Writes one framed TAK Protocol v1 position report to stdout.
//!
//! Useful for cross-checking with other implementations, for example:
//! `cargo run -q -p cottak-proto --example encode_sample > frame.bin`

use bytes::BytesMut;
use cottak_core::detail::{Contact, Remarks, Track};
use cottak_core::{Event, Point};
use cottak_proto::TakFrame;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event = Event::new(
        "ANDROID-1234",
        "a-f-G-U-C".parse()?,
        Point::new(-27.4698, 153.0251)?.with_hae(31.0),
    )
    .with(Contact::new("VIPER 1"))
    .with(Track::new(212.4, 1.3))
    .with(Remarks::new("hello"));

    let mut wire = BytesMut::new();
    TakFrame::from_event(event).encode_stream(&mut wire)?;
    std::io::stdout().write_all(&wire)?;
    Ok(())
}
