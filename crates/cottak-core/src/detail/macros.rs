//! Declarative helpers for attribute-only detail elements.

/// Defines a struct for an element that carries only attributes.
///
/// Every field is `Option<$ty>` and is parsed with `FromStr`; values that fail
/// to parse (and attributes that are not listed) are kept in `extra`.
macro_rules! attrs_item {
    (
        $(#[$meta:meta])* $name:ident, $elem:literal,
        { $( $(#[$fmeta:meta])* $field:ident : $ty:ty => $attr:literal ),+ $(,)? }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            $( $(#[$fmeta])* pub $field: Option<$ty>, )+
            /// Attributes this crate does not model, preserved verbatim.
            pub extra: Vec<(String, String)>,
        }

        impl $crate::detail::DetailItem for $name {
            const NAME: &'static str = $elem;

            fn from_xml(el: &$crate::xml::XmlElement) -> Option<Self> {
                let mut a = $crate::xml::Attrs::new(el);
                Some(Self {
                    $( $field: a.take_parsed::<$ty>($attr), )+
                    extra: a.into_extra(),
                })
            }

            fn to_xml(&self) -> $crate::xml::XmlElement {
                $crate::xml::XmlElement::new($elem)
                    $( .with_opt_attr($attr, self.$field.as_ref().map(ToString::to_string)) )+
                    .with_extra_attrs(&self.extra)
            }
        }
    };
}

/// Defines a `<name value="..."/>` element holding one value.
macro_rules! value_item {
    ($(#[$meta:meta])* $name:ident, $elem:literal, $ty:ty) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            /// The value.
            pub value: $ty,
        }

        impl $name {
            /// Creates the element.
            pub const fn new(value: $ty) -> Self {
                Self { value }
            }
        }

        impl $crate::detail::DetailItem for $name {
            const NAME: &'static str = $elem;

            fn from_xml(el: &$crate::xml::XmlElement) -> Option<Self> {
                Some(Self { value: el.attr("value")?.trim().parse::<$ty>().ok()? })
            }

            fn to_xml(&self) -> $crate::xml::XmlElement {
                $crate::xml::XmlElement::new($elem).with_attr("value", self.value.to_string())
            }
        }
    };
}
