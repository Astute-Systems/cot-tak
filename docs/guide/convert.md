# Conversion and symbology

`cottak-convert` (`cottak::convert`, feature `convert`) moves CoT in and out of the formats GIS and
planning tools use, and maps CoT types to MIL-STD-2525 symbols. It does not depend on the protocol
or networking crates.

| Module | What it does |
|---|---|
| `geojson` | Events ⇄ GeoJSON (RFC 7946) with simplestyle properties |
| `kml` | Events ⇄ KML 2.2; KMZ with the `kmz` feature |
| `sidc` | CoT atom types ⇄ MIL-STD-2525B/C 15-character symbol codes |
| `catalog` | Human-readable type descriptions from a `CoTtypes.xml` you supply |
| `geometry` | The rules that turn a CoT event into a point, line or polygon |
| `color` | ARGB, CSS hex and KML colour conversions |

## How CoT maps to geometry

Both exporters share one rule set (`geometry::geometry_of`):

| CoT | Geometry |
|---|---|
| Routes (`b-m-r`) | Line through the waypoints |
| Polygons (`u-d-r`) | Polygon |
| Freehand/closed shapes (`u-d-f`) | Polygon if it has a fill colour or is closed, otherwise a line |
| Circles and ellipses | 64-sided polygon around the centre |
| Range and bearing lines (`u-rb-a`) | Line |
| Everything else | Point |

## GeoJSON

```rust
use cottak::convert::geojson::{self, GeoJsonOptions, ImportLimits};
use cottak::core::detail::Contact;
use cottak::core::{Event, Point};

let events = vec![
    Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.47, 153.02)?)
        .with(Contact::new("Alpha")),
];

// Export. Coordinates are [lon, lat], as RFC 7946 requires.
let collection = geojson::events_to_collection(&events, &GeoJsonOptions::default());
assert_eq!(collection["features"][0]["geometry"]["coordinates"][0], 153.02);
assert_eq!(collection["features"][0]["properties"]["sidc"], "SFGPUC---------");

// Import: Feature, FeatureCollection or a bare geometry.
let back = geojson::events_from_geojson(&collection, &ImportLimits::default())?;
assert_eq!(back[0].uid, "ALPHA-1");
# Ok::<(), Box<dyn std::error::Error>>(())
```

Import understands `Point` (markers), `LineString` and `Polygon` (drawings), reads `uid`,
`callsign`/`name`, `remarks`/`description`, `type` and simplestyle colours (`marker-color`,
`stroke`, `fill`, widths and opacities), and rejects multi-part geometries, polygons with holes
and anything beyond `ImportLimits` (10,000 features and vertices by default). It is
all-or-nothing: the first bad feature fails the import.

## KML

Export writes one `<Placemark>` per event with styling from the CoT colour details and the CoT
identity in `<ExtendedData>`.

```rust
use cottak::convert::kml::{self, KmlOptions};
use cottak::convert::geojson::ImportLimits;
use cottak::core::detail::Contact;
use cottak::core::{Event, Point};

let events = vec![
    Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.47, 153.02)?)
        .with(Contact::new("Alpha")),
];
let text = kml::events_to_kml(&events, &KmlOptions::default());

let import = kml::events_from_kml(&text, &ImportLimits::default())?;
assert_eq!(import.events[0].uid, "ALPHA-1");
assert!(import.skipped.is_empty());
# Ok::<(), Box<dyn std::error::Error>>(())
```

### Importing real-world KML

Real KML files are messy, so the importer is **lenient about content and strict about safety**:

- A placemark that cannot be represented is **skipped and reported** in `KmlImport::skipped`
  (with its name and the reason), instead of failing the file. Skipped: multi-geometries,
  tracks (`gx:Track`), 3-D models, polygons with holes and placemarks with invalid coordinates.
- Malformed XML, DTDs/entities, oversized documents (8 MiB) and anything beyond `ImportLimits`
  fail the whole import.

What is converted: points become markers (`b-m-p-s-m`, or the original type when the file came
from this library's own export), lines and polygons become drawings. Styles come from inline
`<Style>`, shared styles via `<styleUrl>` and `<StyleMap>` normal styles. `<description>` becomes
remarks and `<TimeStamp>` the event time. Altitude is used only with `altitudeMode` `absolute`,
because other modes are heights above ground, not above the ellipsoid. Namespace prefixes
(`<kml:Placemark>`) are accepted, and spaces around commas inside coordinate tuples are tolerated.

```rust
use cottak::convert::geojson::ImportLimits;
use cottak::convert::kml;

let doc = r#"<kml xmlns="http://www.opengis.net/kml/2.2"><Document>
  <Placemark><name>Pin</name><Point><coordinates>153.02,-27.47</coordinates></Point></Placemark>
  <Placemark><name>Many</name><MultiGeometry><Point><coordinates>1,1</coordinates></Point></MultiGeometry></Placemark>
</Document></kml>"#;

let import = kml::events_from_kml(doc, &ImportLimits::default())?;
assert_eq!(import.events.len(), 1);
assert_eq!(import.skipped[0].name.as_deref(), Some("Many"));
println!("skipped: {}", import.skipped[0].reason);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## KMZ

With the `kmz` feature, `events_to_kmz` writes an archive with a deflated `doc.kml` and
`events_from_kmz` reads one. The archive is processed in memory and never extracted, and it is
treated as hostile: the archive size, the entry count and the KML's declared **and** actual
decompressed size are limited (`KmzLimits`), which defuses zip bombs.

```rust
use cottak::convert::geojson::ImportLimits;
use cottak::convert::kml::{self, KmlOptions, KmzLimits};
use cottak::core::{Event, Point};

let events = vec![Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.47, 153.02)?)];
let kmz = kml::events_to_kmz(&events, &KmlOptions::default())?;

let import = kml::events_from_kmz(&kmz, &ImportLimits::default(), &KmzLimits::default())?;
assert_eq!(import.events[0].uid, "ALPHA-1");
# Ok::<(), Box<dyn std::error::Error>>(())
```

`events_from_kmz` uses `doc.kml` at the root, else another `.kml` at the root, else the first
`.kml` anywhere.

## MIL-STD-2525 symbol codes

`Sidc` converts between CoT atom types and 15-character 2525B/C symbol codes:

```rust
use cottak::convert::Sidc;
use cottak::core::CotType;

let cot: CotType = "a-f-G-U-C".parse()?;
let sidc = Sidc::from_cot(&cot).unwrap();
assert_eq!(sidc.as_str(), "SFGPUC---------");
assert_eq!(sidc.to_cot().unwrap(), cot);
assert_eq!(sidc.affiliation(), 'F');
# Ok::<(), Box<dyn std::error::Error>>(())
```

The mapping is structural. It is exact for types whose levels are single uppercase letters (up to
six levels), and returns `None` rather than guessing for anything else.
`sidc::affiliation_color` gives the conventional colour for an affiliation. MIL-STD-2525D numeric
codes are not supported.

## Type descriptions

CoT type strings are cryptic. If you have MITRE's `CoTtypes.xml`, `TypeCatalog` turns a type into
a human-readable description:

```rust,no_run
use cottak::convert::catalog::TypeCatalog;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let catalog = TypeCatalog::from_xml(&std::fs::read_to_string("CoTtypes.xml")?)?;
if let Some(info) = catalog.describe(&"a-f-G-U-C".parse()?) {
    println!("{info:?}");
}
# Ok(()) }
```

The file is **not bundled** because MITRE's copyright notice grants no redistribution licence;
you supply your own copy.
