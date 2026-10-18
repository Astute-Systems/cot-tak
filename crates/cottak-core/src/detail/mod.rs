//! The `<detail>` block: an ordered, lossless list of elements with typed views.
//!
//! `Detail` stores every child as a generic [`XmlElement`], so unknown or
//! vendor-specific elements always survive a parse/serialise round trip.
//! Known elements are read and written through the [`DetailItem`] trait:
//!
//! ```
//! use cottak_core::detail::{Contact, Detail, Remarks};
//!
//! let mut detail = Detail::new()
//!     .with(Contact::new("ALPHA-1"))
//!     .with(Remarks::new("moving to RV"));
//!
//! assert_eq!(detail.get::<Contact>().unwrap().callsign.as_deref(), Some("ALPHA-1"));
//! detail.set(Remarks::new("at RV"));
//! assert_eq!(detail.get::<Remarks>().unwrap().text, "at RV");
//! ```
//!
//! Applications can add their own typed elements by implementing
//! [`DetailItem`].

#[macro_use]
mod macros;

mod basic;
mod chat;
mod sensors;
mod shapes;

pub use basic::{
    Archive, Color, Contact, FillColor, Group, Link, LinkPoint, Marti, MartiDest,
    PrecisionLocation, Remarks, Status, StrokeColor, StrokeWeight, Takv, Track, Uid, UserIcon,
};
pub use chat::{Chat, ChatGrp, ChatReceipt};
pub use sensors::{ConnectionEntry, Emergency, Medevac, Sensor, Video};
pub use shapes::{
    Bearing, BearingUnits, Bullseye, Ellipse, GeoFence, Height, HeightUnit, Inclination, LabelsOn,
    LinkAttr, NorthRef, Range, RangeUnits, Shape,
};

use crate::xml::XmlElement;

/// A typed view of one kind of `<detail>` child element.
pub trait DetailItem: Sized {
    /// XML element name this type reads and writes.
    const NAME: &'static str;

    /// Interprets `el` (whose name is [`Self::NAME`]). Returns `None` if it
    /// cannot be understood, in which case the element stays available raw.
    fn from_xml(el: &XmlElement) -> Option<Self>;

    /// Produces the XML element for this value.
    fn to_xml(&self) -> XmlElement;
}

/// Ordered collection of `<detail>` child elements.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Detail {
    elements: Vec<XmlElement>,
}

impl Detail {
    /// Creates an empty detail.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a detail from raw child elements.
    pub fn from_elements(elements: Vec<XmlElement>) -> Self {
        Self { elements }
    }

    /// All child elements in document order.
    pub fn elements(&self) -> &[XmlElement] {
        &self.elements
    }

    /// Number of child elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// `true` if there are no child elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// First raw element with the given name.
    pub fn raw(&self, name: &str) -> Option<&XmlElement> {
        self.elements.iter().find(|e| e.name == name)
    }

    /// Appends a raw element.
    pub fn push_raw(&mut self, el: XmlElement) {
        self.elements.push(el);
    }

    /// Removes all elements named `name`, returning how many were removed.
    pub fn remove(&mut self, name: &str) -> usize {
        let before = self.elements.len();
        self.elements.retain(|e| e.name != name);
        before - self.elements.len()
    }

    /// Reads the first element of type `T`.
    pub fn get<T: DetailItem>(&self) -> Option<T> {
        self.elements
            .iter()
            .filter(|e| e.name == T::NAME)
            .find_map(T::from_xml)
    }

    /// Reads every element of type `T` (for repeatable elements like `link`).
    pub fn get_all<T: DetailItem>(&self) -> Vec<T> {
        self.elements
            .iter()
            .filter(|e| e.name == T::NAME)
            .filter_map(T::from_xml)
            .collect()
    }

    /// Appends an element, keeping any existing ones of the same name.
    pub fn push<T: DetailItem>(&mut self, item: T) {
        self.elements.push(item.to_xml());
    }

    /// Replaces the first element of this type (or appends if absent),
    /// removing any further duplicates.
    pub fn set<T: DetailItem>(&mut self, item: T) {
        let new = item.to_xml();
        match self.elements.iter().position(|e| e.name == T::NAME) {
            Some(i) => {
                self.elements[i] = new;
                let mut seen = 0;
                self.elements.retain(|e| {
                    if e.name == T::NAME {
                        seen += 1;
                        seen == 1
                    } else {
                        true
                    }
                });
            }
            None => self.elements.push(new),
        }
    }

    /// Builder-style [`Detail::set`].
    #[must_use]
    pub fn with<T: DetailItem>(mut self, item: T) -> Self {
        self.set(item);
        self
    }

    pub(crate) fn from_xml(el: &XmlElement) -> Self {
        Self {
            elements: el.elements().cloned().collect(),
        }
    }

    pub(crate) fn to_xml(&self) -> XmlElement {
        let mut el = XmlElement::new("detail");
        el.children = self
            .elements
            .iter()
            .cloned()
            .map(crate::xml::XmlNode::Element)
            .collect();
        el
    }
}
