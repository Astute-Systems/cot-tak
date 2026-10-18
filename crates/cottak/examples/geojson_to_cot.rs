//! Convert a GeoJSON file to CoT events (and a KML copy), printing both.
//!
//! ```text
//! cargo run -p cottak --example geojson_to_cot --features convert -- area.geojson
//! ```

use cottak::convert::{geojson, kml};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: geojson_to_cot <file.geojson>")?;
    let text = std::fs::read_to_string(path)?;
    let events = geojson::events_from_str(&text, &geojson::ImportLimits::default())?;

    for ev in &events {
        println!("{}", ev.to_xml_string()?);
    }
    eprintln!("{} event(s); KML follows", events.len());
    println!(
        "{}",
        kml::events_to_kml(&events, &kml::KmlOptions::default())
    );
    Ok(())
}
