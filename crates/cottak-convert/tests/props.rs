//! Property tests for the converters.
#![allow(clippy::float_cmp)]

use cottak_convert::geojson::{
    events_from_geojson, events_to_collection, GeoJsonOptions, ImportLimits,
};
use cottak_convert::kml::{events_from_kml, events_to_kml, KmlOptions};
use cottak_convert::Sidc;
use cottak_core::detail::{Contact, Remarks};
use cottak_core::messages::draw::{self, Style};
use cottak_core::xml::XmlElement;
use cottak_core::{CotType, Event, Point};
use proptest::prelude::*;
use serde_json::{json, Map, Value};

/// Arbitrary JSON, biased toward GeoJSON-looking keys so the importer's deeper paths are reached.
fn json_value() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::Bool),
        any::<f64>().prop_map(|f| json!(f)),
        any::<i64>().prop_map(|i| json!(i)),
        "[ -~]{0,12}".prop_map(Value::String),
        prop_oneof![
            Just("Point"),
            Just("LineString"),
            Just("Polygon"),
            Just("Feature"),
            Just("FeatureCollection")
        ]
        .prop_map(|s| json!(s)),
    ];
    leaf.prop_recursive(4, 48, 6, |inner| {
        let keys = prop_oneof![
            Just("type".to_owned()),
            Just("coordinates".to_owned()),
            Just("geometry".to_owned()),
            Just("properties".to_owned()),
            Just("features".to_owned()),
            Just("uid".to_owned()),
            Just("stroke".to_owned()),
            "[a-z]{1,6}",
        ];
        prop_oneof![
            prop::collection::vec(inner.clone(), 0..5).prop_map(Value::Array),
            prop::collection::vec((keys, inner), 0..6)
                .prop_map(|kv| Value::Object(kv.into_iter().collect::<Map<_, _>>())),
        ]
    })
}

proptest! {
    #[test]
    fn importer_never_panics_on_arbitrary_json(v in json_value()) {
        let _ = events_from_geojson(&v, &ImportLimits::default());
    }

    #[test]
    fn importer_never_panics_on_arbitrary_text(s in "\\PC{0,200}") {
        let _ = cottak_convert::geojson::events_from_str(&s, &ImportLimits::default());
    }

    #[test]
    fn kml_importer_never_panics_on_arbitrary_text(s in "\\PC{0,300}") {
        let _ = events_from_kml(&s, &ImportLimits::default());
    }

    /// Fragments that reach the placemark, style and coordinate code paths.
    #[test]
    fn kml_importer_never_panics_on_kml_shaped_text(
        parts in proptest::collection::vec(
            prop_oneof![
                Just("<kml>"), Just("</kml>"), Just("<Document>"), Just("</Document>"),
                Just("<Placemark id=\"a\">"), Just("</Placemark>"), Just("<name>n</name>"),
                Just("<Point>"), Just("</Point>"), Just("<LineString>"), Just("</LineString>"),
                Just("<Polygon><outerBoundaryIs><LinearRing>"), Just("</LinearRing></outerBoundaryIs></Polygon>"),
                Just("<coordinates>1,2 3,4 5,6</coordinates>"), Just("<coordinates>x,y</coordinates>"),
                Just("<coordinates>181,91,1e999</coordinates>"), Just("<coordinates>1,2,3,4</coordinates>"),
                Just("<altitudeMode>absolute</altitudeMode>"), Just("<styleUrl>#s</styleUrl>"),
                Just("<Style id=\"s\"><LineStyle><color>ff00ff00</color><width>-1</width></LineStyle></Style>"),
                Just("<StyleMap id=\"m\"><Pair><key>normal</key><styleUrl>#s</styleUrl></Pair></StyleMap>"),
                Just("<TimeStamp><when>garbage</when></TimeStamp>"),
                Just("<ExtendedData><Data name=\"type\"><value>a-f-G</value></Data></ExtendedData>"),
                Just("<description><![CDATA[<b>x</b>]]></description>"),
                Just("<MultiGeometry>"), Just("</MultiGeometry>"),
            ],
            0..24,
        )
    ) {
        let _ = events_from_kml(&parts.concat(), &ImportLimits::default());
    }

    #[test]
    fn kml_export_then_import_keeps_markers(
        lat in -90.0f64..=90.0,
        lon in -180.0f64..=180.0,
        callsign in "[A-Za-z0-9 _-]{1,16}",
        uid in "[A-Za-z0-9_.-]{1,24}",
    ) {
        let ev = Event::new(uid.clone(), "a-f-G-U-C".parse::<CotType>().unwrap(), Point::new(lat, lon).unwrap())
            .with(Contact::new(callsign.trim()));
        prop_assume!(!callsign.trim().is_empty());
        let kml = events_to_kml([&ev], &KmlOptions::default());
        let got = events_from_kml(&kml, &ImportLimits::default()).unwrap();
        prop_assert!(got.skipped.is_empty());
        prop_assert_eq!(got.events.len(), 1);
        prop_assert_eq!(&got.events[0].uid, &uid);
        prop_assert!((got.events[0].point.lat - lat).abs() < 1e-9);
        prop_assert!((got.events[0].point.lon - lon).abs() < 1e-9);
        let back = got.events[0].detail.get::<Contact>().unwrap().callsign;
        prop_assert_eq!(back.as_deref(), Some(callsign.trim()));
    }

    #[test]
    fn markers_round_trip_through_geojson(
        lat in -90.0f64..=90.0,
        lon in -180.0f64..=180.0,
        hae in proptest::option::of(-500.0f64..9000.0),
        callsign in "[A-Za-z0-9 _-]{1,16}",
        uid in "[A-Za-z0-9_.-]{1,24}",
    ) {
        let mut point = Point::new(lat, lon).unwrap();
        point.hae = hae;
        let ev = Event::new(uid.clone(), "a-f-G-U-C".parse::<CotType>().unwrap(), point).with(Contact::new(callsign.clone()));
        let doc = events_to_collection([&ev], &GeoJsonOptions::default());
        let back = events_from_geojson(&doc, &ImportLimits::default()).unwrap();
        prop_assert_eq!(back.len(), 1);
        prop_assert_eq!(&back[0].uid, &uid);
        prop_assert_eq!(back[0].point.lat, lat);
        prop_assert_eq!(back[0].point.lon, lon);
        prop_assert_eq!(back[0].point.hae, hae);
        let contact = back[0].detail.get::<Contact>().unwrap();
        prop_assert_eq!(contact.callsign.as_deref(), Some(callsign.as_str()));
        prop_assert_eq!(back[0].cot_type.as_str(), "a-f-G-U-C");
    }

    #[test]
    fn polygons_round_trip_vertex_for_vertex(
        pts in prop::collection::vec((-89.0f64..89.0, -179.0f64..179.0), 3..40),
    ) {
        let vs: Vec<Point> = pts.iter().map(|&(la, lo)| Point::new(la, lo).unwrap()).collect();
        let ev = draw::polygon("P", "poly", &vs, Style::default()).unwrap();
        let doc = events_to_collection([&ev], &GeoJsonOptions::default());
        let back = events_from_geojson(&doc, &ImportLimits::default()).unwrap();
        let links = back[0].detail.get_all::<cottak_core::detail::Link>();
        prop_assert_eq!(links.len(), vs.len());
        for (l, v) in links.iter().zip(&vs) {
            let lp = l.point.unwrap();
            prop_assert_eq!((lp.lat, lp.lon), (v.lat, v.lon));
        }
    }

    /// 2525 conversion is lossless for every type it accepts.
    #[test]
    fn sidc_round_trips(
        aff in prop::sample::select(vec!['p', 'u', 'a', 'f', 'n', 's', 'h', 'j', 'k', 'o']),
        dim in prop::sample::select(vec!['P', 'A', 'G', 'S', 'U', 'F']),
        rest in prop::collection::vec("[A-Z0-9]", 0..=6),
    ) {
        let mut ty = format!("a-{aff}-{dim}");
        for r in &rest {
            ty.push('-');
            ty.push_str(r);
        }
        let cot: CotType = ty.parse().unwrap();
        let sidc = Sidc::from_cot(&cot).expect("representable");
        prop_assert_eq!(sidc.as_str().len(), 15);
        prop_assert_eq!(sidc.to_cot().unwrap(), cot);
    }

    /// Whatever text a callsign or remark contains, KML stays well-formed and keeps the text.
    #[test]
    fn kml_is_always_well_formed(callsign in "[ -~\u{A0}-\u{2FF}\n\t]{1,30}", remark in "[ -~\u{A0}-\u{2FF}\n\t]{1,60}") {
        let ev = Event::new("U", "b-m-p-s-m".parse::<CotType>().unwrap(), Point::new(1.0, 2.0).unwrap())
            .with(Contact::new(callsign.clone()))
            .with(Remarks::new(remark.clone()));
        let kml = events_to_kml([&ev], &KmlOptions::default());
        let root = XmlElement::parse(&kml).unwrap();
        let pm = root.child("Document").unwrap().child("Placemark").unwrap();
        prop_assert_eq!(pm.child("name").unwrap().text(), callsign);
        prop_assert_eq!(pm.child("description").unwrap().text(), remark);
    }
}
