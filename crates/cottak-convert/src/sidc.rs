//! MIL-STD-2525 symbol identification codes (SIDC) for CoT atom types.
//!
//! CoT atom types follow the structure of the 2525 symbol hierarchy:
//! `a-<affiliation>-<battle dimension>-<function letters...>`. A 15-character
//! 2525B/C SIDC has the layout
//!
//! ```text
//! S F G P U C I - - - - - - - -
//! │ │ │ │ └─────┘ └┘ └┘ │
//! │ │ │ │  function  │  │  └ order of battle
//! │ │ │ │    id (6)  │  └ country (2)
//! │ │ │ └ status     └ modifiers (2)
//! │ │ └ battle dimension
//! │ └ affiliation
//! └ coding scheme (S = warfighting)
//! ```
//!
//! The conversion is purely structural, so no symbol tables are involved. It
//! is exact for types made of single upper-case function letters (the 2525
//! part of CoT types); types with CoT-specific lower-case extensions or more
//! than six function levels cannot be represented and yield `None`.
//!
//! Only the 15-character letter-based codes are supported, not the 20-digit
//! 2525D numeric form.

use crate::error::{Error, Result};
use cottak_core::{Affiliation, CotType, Dimension, Kind};
use std::fmt;
use std::str::FromStr;

const FUNCTION_LEN: usize = 6;

/// A 15-character 2525B/C symbol identification code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sidc(String);

impl Sidc {
    /// Validates the shape of a SIDC: 15 characters from `A-Z`, `0-9`, `-` and `*`.
    pub fn new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        let ok = s.len() == 15
            && s.bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || matches!(b, b'-' | b'*'));
        if ok {
            Ok(Self(s))
        } else {
            Err(Error::InvalidSidc(s))
        }
    }

    /// The code as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Coding scheme (position 1), for example `S`.
    pub fn scheme(&self) -> char {
        self.char_at(0)
    }

    /// Affiliation letter (position 2), for example `F`.
    pub fn affiliation(&self) -> char {
        self.char_at(1)
    }

    /// Battle dimension (position 3), for example `G`.
    pub fn dimension(&self) -> char {
        self.char_at(2)
    }

    /// Status (position 4): `P` present, `A` anticipated, ...
    pub fn status(&self) -> char {
        self.char_at(3)
    }

    /// The six-character function identifier (positions 5-10).
    pub fn function_id(&self) -> &str {
        &self.0[4..4 + FUNCTION_LEN]
    }

    fn char_at(&self, i: usize) -> char {
        char::from(self.0.as_bytes()[i])
    }

    /// The SIDC for a CoT atom type, or `None` if the type cannot be
    /// represented exactly (not an atom, unmapped affiliation or dimension,
    /// lower-case/multi-character function levels, or more than six levels).
    pub fn from_cot(t: &CotType) -> Option<Self> {
        if t.kind() != Kind::Atom {
            return None;
        }
        let mut seg = t.segments().skip(1);
        let aff = to_sidc_affiliation(t.affiliation()?)?;
        let dim = to_sidc_dimension(t.dimension()?)?;
        seg.next();
        seg.next();

        let mut function = String::new();
        for s in seg {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) if c.is_ascii_uppercase() || c.is_ascii_digit() => function.push(c),
                _ => return None,
            }
        }
        if function.len() > FUNCTION_LEN {
            return None;
        }
        while function.len() < FUNCTION_LEN {
            function.push('-');
        }
        Some(Self(format!("S{aff}{dim}P{function}-----")))
    }

    /// The CoT atom type for a warfighting-scheme SIDC, or `None` if the code
    /// uses a scheme, affiliation (exercise variants) or battle dimension that
    /// CoT atoms cannot express. The status position is not represented in CoT.
    pub fn to_cot(&self) -> Option<CotType> {
        if self.scheme() != 'S' {
            return None;
        }
        let aff = from_sidc_affiliation(self.affiliation())?;
        let dim = from_sidc_dimension(self.dimension())?;
        let function = self.function_id().trim_end_matches('-');
        if function.contains(['-', '*']) {
            return None;
        }
        let rest = function
            .chars()
            .map(String::from)
            .collect::<Vec<_>>()
            .join("-");
        CotType::atom(aff, dim, &rest).ok()
    }
}

impl fmt::Display for Sidc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Sidc {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

fn to_sidc_affiliation(a: Affiliation) -> Option<char> {
    Some(match a {
        Affiliation::Pending => 'P',
        Affiliation::Unknown => 'U',
        Affiliation::AssumedFriend => 'A',
        Affiliation::Friend => 'F',
        Affiliation::Neutral => 'N',
        Affiliation::Suspect => 'S',
        Affiliation::Hostile => 'H',
        Affiliation::Joker => 'J',
        Affiliation::Faker => 'K',
        Affiliation::None => 'O',
        _ => return None,
    })
}

fn from_sidc_affiliation(c: char) -> Option<Affiliation> {
    Some(match c {
        'P' => Affiliation::Pending,
        'U' => Affiliation::Unknown,
        'A' => Affiliation::AssumedFriend,
        'F' => Affiliation::Friend,
        'N' => Affiliation::Neutral,
        'S' => Affiliation::Suspect,
        'H' => Affiliation::Hostile,
        'J' => Affiliation::Joker,
        'K' => Affiliation::Faker,
        'O' => Affiliation::None,
        _ => return None,
    })
}

fn to_sidc_dimension(d: Dimension) -> Option<char> {
    Some(match d {
        Dimension::Space => 'P',
        Dimension::Air => 'A',
        Dimension::Ground => 'G',
        Dimension::SeaSurface => 'S',
        Dimension::Subsurface => 'U',
        Dimension::Sof => 'F',
        _ => return None,
    })
}

fn from_sidc_dimension(c: char) -> Option<Dimension> {
    Some(match c {
        'P' => Dimension::Space,
        'A' => Dimension::Air,
        'G' => Dimension::Ground,
        'S' => Dimension::SeaSurface,
        'U' => Dimension::Subsurface,
        'F' => Dimension::Sof,
        _ => return None,
    })
}

/// The conventional display colour of an affiliation as `(r, g, b)`, for
/// shapes and markers that are not drawn with a full symbol. `None` for
/// affiliations without a conventional colour.
pub fn affiliation_color(a: Affiliation) -> Option<(u8, u8, u8)> {
    match a {
        Affiliation::Friend | Affiliation::AssumedFriend => Some((128, 224, 255)),
        Affiliation::Hostile | Affiliation::Suspect | Affiliation::Joker => Some((255, 128, 128)),
        Affiliation::Neutral => Some((170, 255, 170)),
        Affiliation::Unknown | Affiliation::Pending | Affiliation::Faker => Some((255, 255, 128)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(s: &str) -> CotType {
        s.parse().unwrap()
    }

    #[test]
    fn well_known_codes() {
        // Friendly ground combat unit.
        assert_eq!(
            Sidc::from_cot(&t("a-f-G-U-C")).unwrap().as_str(),
            "SFGPUC---------"
        );
        // Hostile infantry.
        assert_eq!(
            Sidc::from_cot(&t("a-h-G-U-C-I")).unwrap().as_str(),
            "SHGPUCI--------"
        );
        // Unknown fixed-wing military aircraft.
        assert_eq!(
            Sidc::from_cot(&t("a-u-A-M-F")).unwrap().as_str(),
            "SUAPMF---------"
        );
        // Bare dimension.
        assert_eq!(
            Sidc::from_cot(&t("a-n-S")).unwrap().as_str(),
            "SNSP-----------"
        );
    }

    #[test]
    fn fields() {
        let s = Sidc::from_cot(&t("a-f-G-U-C-I")).unwrap();
        assert_eq!(
            (s.scheme(), s.affiliation(), s.dimension(), s.status()),
            ('S', 'F', 'G', 'P')
        );
        assert_eq!(s.function_id(), "UCI---");
    }

    #[test]
    fn round_trip_for_representable_types() {
        for ty in [
            "a-f-G-U-C",
            "a-h-A-M-F-F",
            "a-n-S-C-M",
            "a-u-G",
            "a-s-U-S",
            "a-j-G-E-V",
            "a-p-P",
        ] {
            let sidc = Sidc::from_cot(&t(ty)).unwrap();
            assert_eq!(sidc.to_cot().unwrap().as_str(), ty, "{ty} via {sidc}");
        }
    }

    #[test]
    fn unrepresentable_types_are_none() {
        for ty in [
            "b-m-p-s-m",             // not an atom
            "a-f-G-U-C-q",           // lower-case CoT extension
            "a-f-G-U-C-I-A-B-C-D-E", // more than six levels
            "a-x-G-U",               // unmapped affiliation
            "a-f-X-U",               // unmapped dimension
            "a-f-G-ab",              // multi-character level
        ] {
            assert_eq!(Sidc::from_cot(&t(ty)), None, "{ty}");
        }
    }

    #[test]
    fn sidc_to_cot_rejects_what_cot_cannot_express() {
        for code in [
            "GFGPU----------", // tactical graphics scheme
            "SDGPUC---------", // exercise friend
            "SFZPUC---------", // unknown dimension
            "SFGPU-C-------*", // gap inside the function id
        ] {
            assert_eq!(Sidc::new(code).unwrap().to_cot(), None, "{code}");
        }
    }

    #[test]
    fn parse_validates_shape() {
        assert!("SFGPUCI--------".parse::<Sidc>().is_ok());
        for bad in [
            "",
            "SFGPUCI",
            "sfgpuci--------",
            "SFGPUCI-------- ",
            "SFGPUCI-------_-",
        ] {
            assert!(bad.parse::<Sidc>().is_err(), "{bad:?}");
        }
    }

    #[test]
    fn colours() {
        assert_eq!(
            affiliation_color(Affiliation::Hostile),
            Some((255, 128, 128))
        );
        assert_eq!(
            affiliation_color(Affiliation::Friend),
            Some((128, 224, 255))
        );
        assert_eq!(affiliation_color(Affiliation::Other), None);
    }
}
