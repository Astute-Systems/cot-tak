# Events and details

`cottak-core` (re-exported as `cottak::core`) holds the data model. It has no I/O and no async
runtime, so it is usable anywhere, including in other protocols' gateways and in tests.

## The `Event`

A CoT message is an `<event>` element: who it is about (`uid`), what it is (`type`), where
(`point`), when (`time`, `start`, `stale`), how it was produced (`how`) and a free-form `detail`
block.

```rust
use cottak::core::{Event, Point};

let event = Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.4689, 153.0235)?);

assert_eq!(event.uid, "ALPHA-1");
assert_eq!(event.version, "2.0");
assert!(event.stale > event.start);   // stale two minutes after it starts, by default
assert!(!event.is_stale());
# Ok::<(), cottak::core::Error>(())
```

All fields are public. `Event` is `#[non_exhaustive]`, so create it with `Event::new` and the
builder methods (`with_how`, `with_stale_in`, `with`, `with_detail`), then assign fields
directly where needed.

### Points

`Point::new(lat, lon)` validates ranges and rejects NaN and infinities. Height above ellipsoid and
the circular/linear error use the CoT conventions (`999999` means *unknown*):

```rust
use cottak::core::Point;

let p = Point::new(-27.47, 153.02)?.with_hae(12.0).with_errors(5.0, 10.0);
assert_eq!(p.hae, Some(12.0));
assert!(Point::new(91.0, 0.0).is_err());
assert!(Point::new(f64::NAN, 0.0).is_err());
# Ok::<(), cottak::core::Error>(())
```

### Types

A `CotType` is a validated, case-sensitive type string such as `a-f-G-U-C`. Atoms (`a-...`) have
an affiliation and a battle dimension; `is_a` queries the hierarchy, where `.` matches any single
segment:

```rust
use cottak::core::{Affiliation, CotType, Dimension, Kind};

let t: CotType = "a-f-G-U-C".parse()?;
assert_eq!(t.kind(), Kind::Atom);
assert_eq!(t.affiliation(), Some(Affiliation::Friend));
assert_eq!(t.dimension(), Some(Dimension::Ground));
assert!(t.is_a("a-.-G"));          // any affiliation, ground
assert!(!t.is_a("a-h"));           // not hostile
assert!("not a type".parse::<CotType>().is_err());
# Ok::<(), cottak::core::Error>(())
```

## Details

The `<detail>` block carries everything beyond identity and position. `cottak` models the common
elements as **typed details** and keeps everything else as raw XML.

### Typed details

Each typed detail implements `DetailItem`. Read with `get`, write with `with`/`set`/`push`:

```rust
use cottak::core::detail::{Contact, Remarks, Track};
use cottak::core::{Event, Point};

let mut event = Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(1.0, 2.0)?)
    .with(Contact::new("Rusty"))
    .with(Track::new(90.0, 1.5));

assert_eq!(event.detail.get::<Contact>().unwrap().callsign.as_deref(), Some("Rusty"));

event.detail.set(Remarks::new("moving east"));   // replaces any existing <remarks>
event.detail.set(Remarks::new("halted"));
assert_eq!(event.detail.get::<Remarks>().unwrap().text, "halted");
# Ok::<(), cottak::core::Error>(())
```

`set` replaces, `push` appends (for repeatable elements such as `<link>`), `get_all` reads every
occurrence and `remove` deletes by name. Typed details keep any attributes they do not model in
an `extra` field, so reading and writing back does not lose data.

The modelled elements include `contact`, `__group`, `takv`, `track`, `status`, `remarks`,
`precisionlocation`, `link`, `marti`, `uid`, `archive`, colours and stroke/fill styling, shapes
(`shape`, `ellipse`, `bullseye`), `__chat` and its group/receipt elements, `__medevac`,
`__emergency`, `sensor`, `__video` and `__video` connection entries. See the
[`detail` module](../../api/cottak_core/detail/index.html) for the full list.

### Raw details

Anything else is preserved as an `XmlElement`:

```rust
use cottak::core::xml::XmlElement;
use cottak::core::{Event, Point};

let mut event = Event::new("U", "a-f-G".parse()?, Point::new(1.0, 2.0)?);
event.detail.push_raw(
    XmlElement::new("my_extension")
        .with_attr("key", "value")
        .with_child(XmlElement::new("nested")),
);
assert_eq!(event.detail.raw("my_extension").unwrap().attr("key"), Some("value"));
# Ok::<(), cottak::core::Error>(())
```

## XML in and out

```rust
use cottak::core::Event;

let xml = "<event version='2.0' uid='U' type='a-f-G' how='m-g' \
    time='2025-01-01T00:00:00Z' start='2025-01-01T00:00:00Z' stale='2025-01-01T00:05:00Z'>\
    <point lat='1' lon='2' hae='0' ce='5' le='5'/>\
    <detail><vendor_thing a='1'><x/></vendor_thing></detail></event>";

let event: Event = xml.parse()?;          // or Event::from_xml_str(xml)
let again: Event = event.to_xml_string()?.parse()?;

assert_eq!(again, event);                 // lossless: the unknown element survived
assert!(event.detail.raw("vendor_thing").is_some());
# Ok::<(), cottak::core::Error>(())
```

The writer is hand-written and escapes everything (including line breaks and tabs in
attributes), so any text round-trips exactly.

### What the parser refuses

Event XML usually comes from the network, so the parser is strict:

```rust
use cottak::core::Event;

// DTDs and custom entities (XXE, "billion laughs") are rejected outright.
let evil = "<!DOCTYPE event [<!ENTITY e 'boom'>]><event/>";
assert!(Event::from_xml_str(evil).is_err());

// So is anything that is not an <event>, or is missing required parts.
assert!(Event::from_xml_str("<html/>").is_err());
assert!(Event::from_xml_str("<event/>").is_err());
```

| Limit | Value |
|---|---|
| Document size | 8 MiB (`xml::MAX_XML_LEN`) |
| Element nesting | 64 levels (`xml::MAX_DEPTH`) |
| Attributes per element | 1024 (`xml::MAX_ATTRS`) |
| Control characters | rejected in names and values |

`Event::validate` re-checks a hand-built event (finite numbers, legal characters, sane details)
before you send it; the networking layers call it for you.

## Time

Timestamps are `time::OffsetDateTime` in UTC, written with millisecond precision. The
`cottak::core::timestamp` module parses and formats the CoT form:

```rust
use cottak::core::timestamp;

let t = timestamp::parse("2025-01-31T12:00:00Z")?;
assert_eq!(timestamp::format(t)?, "2025-01-31T12:00:00.000Z");
# Ok::<(), cottak::core::Error>(())
```

## Optional serde support

With the `serde` feature, the model types implement `Serialize` and `Deserialize`, which is handy
for logging events as JSON or storing them in a database.
