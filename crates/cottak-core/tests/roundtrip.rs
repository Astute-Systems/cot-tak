//! Property tests: serialise → parse must be the identity.

use cottak_core::detail::{Contact, Remarks, Track};
use cottak_core::xml::{XmlElement, XmlNode};
use cottak_core::{CotType, Event, Point};
use proptest::prelude::*;

fn name() -> impl Strategy<Value = String> {
    "[a-zA-Z_][a-zA-Z0-9_]{0,8}"
}

/// Text made only of valid XML characters, including awkward ones.
fn text() -> impl Strategy<Value = String> {
    "[ -~\n\t\r\u{A0}-\u{D7FF}\u{10000}-\u{10FFFF}]{0,24}"
}

fn element() -> impl Strategy<Value = XmlElement> {
    let leaf = (
        name(),
        prop::collection::btree_map(name(), text(), 0..4),
        text(),
    )
        .prop_map(|(n, attrs, t)| {
            let mut e = XmlElement::new(n);
            e.attrs = attrs.into_iter().collect();
            if !t.is_empty() {
                e.children.push(XmlNode::Text(t));
            }
            e
        });
    leaf.prop_recursive(4, 24, 4, |inner| {
        (
            name(),
            prop::collection::btree_map(name(), text(), 0..3),
            prop::collection::vec(inner, 1..4),
        )
            .prop_map(|(n, attrs, kids)| {
                let mut e = XmlElement::new(n);
                e.attrs = attrs.into_iter().collect();
                e.children = kids.into_iter().map(XmlNode::Element).collect();
                e
            })
    })
}

proptest! {
    #[test]
    fn xml_round_trip(el in element()) {
        let xml = el.to_xml();
        prop_assert_eq!(XmlElement::parse(&xml).unwrap(), el);
    }

    #[test]
    fn parser_never_panics(s in "\\PC{0,200}") {
        let _ = XmlElement::parse(&s);
        let _ = s.parse::<Event>();
    }

    #[test]
    fn event_round_trip(
        uid in "[A-Za-z0-9_.-]{1,32}",
        lat in -90.0f64..=90.0,
        lon in -180.0f64..=180.0,
        hae in proptest::option::of(-1000.0f64..20000.0),
        callsign in text(),
        remark in text(),
        course in 0.0f64..360.0,
        speed in 0.0f64..400.0,
    ) {
        let mut point = Point::new(lat, lon).unwrap();
        point.hae = hae;
        let event = Event::new(uid, "a-f-G-U-C".parse::<CotType>().unwrap(), point)
            .with(Contact::new(callsign))
            .with(Remarks::new(remark))
            .with(Track::new(course, speed));
        let parsed: Event = event.to_xml_string().unwrap().parse().unwrap();
        prop_assert_eq!(parsed, event);
    }
}
