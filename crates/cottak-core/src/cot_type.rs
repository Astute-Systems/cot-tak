//! CoT type strings such as `a-f-G-U-C` (friendly ground combat unit).
//!
//! A type is a hyphen-separated path. The first segment selects the *kind*
//! (atom, bits, tasking, ...). For atoms the next two segments are the
//! affiliation and battle dimension; the rest follow MIL-STD-2525 style
//! function codes.

use crate::error::{Error, Result};
use std::fmt;
use std::str::FromStr;

const MAX_TYPE_LEN: usize = 128;

/// Top-level category of a CoT type (first path segment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Kind {
    /// `a` – an atom: a physical entity (unit, vehicle, building ...).
    Atom,
    /// `b` – bits: information (sensor data, chat, alerts, markers ...).
    Bits,
    /// `c` – capability.
    Capability,
    /// `r` – reservation / restriction.
    Reservation,
    /// `t` – tasking (requests, control messages).
    Tasking,
    /// `y` – reply.
    Reply,
    /// Any other letter (for example `u` for user-defined drawings).
    Other(char),
}

/// Affiliation of an atom (second path segment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Affiliation {
    /// `p` – pending.
    Pending,
    /// `u` – unknown.
    Unknown,
    /// `a` – assumed friend.
    AssumedFriend,
    /// `f` – friend.
    Friend,
    /// `n` – neutral.
    Neutral,
    /// `s` – suspect.
    Suspect,
    /// `h` – hostile.
    Hostile,
    /// `j` – joker (friendly acting as hostile in exercises).
    Joker,
    /// `k` – faker (hostile acting as friendly in exercises).
    Faker,
    /// `o` – none specified.
    None,
    /// `x` – other.
    Other,
}

impl Affiliation {
    /// The single-letter CoT code.
    pub const fn code(self) -> char {
        match self {
            Self::Pending => 'p',
            Self::Unknown => 'u',
            Self::AssumedFriend => 'a',
            Self::Friend => 'f',
            Self::Neutral => 'n',
            Self::Suspect => 's',
            Self::Hostile => 'h',
            Self::Joker => 'j',
            Self::Faker => 'k',
            Self::None => 'o',
            Self::Other => 'x',
        }
    }

    /// Parses a single-letter CoT affiliation code.
    pub const fn from_code(c: char) -> Option<Self> {
        Some(match c {
            'p' => Self::Pending,
            'u' => Self::Unknown,
            'a' => Self::AssumedFriend,
            'f' => Self::Friend,
            'n' => Self::Neutral,
            's' => Self::Suspect,
            'h' => Self::Hostile,
            'j' => Self::Joker,
            'k' => Self::Faker,
            'o' => Self::None,
            'x' => Self::Other,
            _ => return None,
        })
    }
}

/// Battle dimension of an atom (third path segment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Dimension {
    /// `P` – space.
    Space,
    /// `A` – air.
    Air,
    /// `G` – ground.
    Ground,
    /// `S` – sea surface.
    SeaSurface,
    /// `U` – subsurface.
    Subsurface,
    /// `F` – special operations forces.
    Sof,
    /// `X` – other.
    Other,
    /// `Z` – unknown.
    Unknown,
}

impl Dimension {
    /// The single-letter CoT code.
    pub const fn code(self) -> char {
        match self {
            Self::Space => 'P',
            Self::Air => 'A',
            Self::Ground => 'G',
            Self::SeaSurface => 'S',
            Self::Subsurface => 'U',
            Self::Sof => 'F',
            Self::Other => 'X',
            Self::Unknown => 'Z',
        }
    }

    /// Parses a single-letter CoT dimension code.
    pub const fn from_code(c: char) -> Option<Self> {
        Some(match c {
            'P' => Self::Space,
            'A' => Self::Air,
            'G' => Self::Ground,
            'S' => Self::SeaSurface,
            'U' => Self::Subsurface,
            'F' => Self::Sof,
            'X' => Self::Other,
            'Z' => Self::Unknown,
            _ => return None,
        })
    }
}

/// A validated CoT type string.
///
/// Matching of the type strings is case sensitive, as in the CoT specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct CotType(String);

impl CotType {
    /// Creates a type from a string, validating its syntax.
    pub fn new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        let ok = !s.is_empty()
            && s.len() <= MAX_TYPE_LEN
            && !s.starts_with('-')
            && !s.ends_with('-')
            && !s.contains("--")
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_'));
        if ok {
            Ok(Self(s))
        } else {
            Err(Error::InvalidType(s))
        }
    }

    /// Builds an atom type: `a-<affiliation>-<dimension>[-<rest>]`.
    ///
    /// `rest` is the function path (for example `U-C` for a combat unit) and
    /// may be empty.
    pub fn atom(aff: Affiliation, dim: Dimension, rest: &str) -> Result<Self> {
        let mut s = format!("a-{}-{}", aff.code(), dim.code());
        if !rest.is_empty() {
            s.push('-');
            s.push_str(rest);
        }
        Self::new(s)
    }

    /// The type as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Iterates over the hyphen-separated segments.
    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.0.split('-')
    }

    /// The top-level kind of this type.
    pub fn kind(&self) -> Kind {
        let first = self.segments().next().unwrap_or_default();
        match first {
            "a" => Kind::Atom,
            "b" => Kind::Bits,
            "c" => Kind::Capability,
            "r" => Kind::Reservation,
            "t" => Kind::Tasking,
            "y" => Kind::Reply,
            other => Kind::Other(other.chars().next().unwrap_or('?')),
        }
    }

    /// Affiliation, for atoms with a recognised affiliation segment.
    pub fn affiliation(&self) -> Option<Affiliation> {
        if self.kind() != Kind::Atom {
            return None;
        }
        single_char(self.segments().nth(1)?).and_then(Affiliation::from_code)
    }

    /// Battle dimension, for atoms with a recognised dimension segment.
    pub fn dimension(&self) -> Option<Dimension> {
        if self.kind() != Kind::Atom {
            return None;
        }
        single_char(self.segments().nth(2)?).and_then(Dimension::from_code)
    }

    /// Returns a copy with the affiliation segment replaced.
    ///
    /// Returns `None` if this is not an atom.
    pub fn with_affiliation(&self, aff: Affiliation) -> Option<Self> {
        if self.kind() != Kind::Atom || self.segments().nth(1).is_none() {
            return None;
        }
        let mut parts: Vec<String> = self.segments().map(str::to_owned).collect();
        parts[1] = aff.code().to_string();
        Self::new(parts.join("-")).ok()
    }

    /// `true` if this type lies at or below `pattern` in the type hierarchy.
    ///
    /// A `.` segment in the pattern matches any single segment, which allows
    /// affiliation-agnostic queries such as `a-.-G`.
    pub fn is_a(&self, pattern: &str) -> bool {
        let mut mine = self.segments();
        for p in pattern.split('-') {
            match mine.next() {
                Some(m) if p == "." || p == m => {}
                _ => return false,
            }
        }
        true
    }
}

fn single_char(s: &str) -> Option<char> {
    let mut it = s.chars();
    match (it.next(), it.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

impl fmt::Display for CotType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CotType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl TryFrom<String> for CotType {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::new(s)
    }
}

impl From<CotType> for String {
    fn from(t: CotType) -> Self {
        t.0
    }
}

impl AsRef<str> for CotType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_atoms() {
        let t: CotType = "a-f-G-U-C".parse().unwrap();
        assert_eq!(t.kind(), Kind::Atom);
        assert_eq!(t.affiliation(), Some(Affiliation::Friend));
        assert_eq!(t.dimension(), Some(Dimension::Ground));
    }

    #[test]
    fn classifies_non_atoms() {
        assert_eq!("b-t-f".parse::<CotType>().unwrap().kind(), Kind::Bits);
        assert_eq!("t-x-c-t".parse::<CotType>().unwrap().kind(), Kind::Tasking);
        assert_eq!(
            "u-d-c-c".parse::<CotType>().unwrap().kind(),
            Kind::Other('u')
        );
        assert_eq!("b-t-f".parse::<CotType>().unwrap().affiliation(), None);
    }

    #[test]
    fn hierarchy_matching() {
        let t: CotType = "a-h-A-M-F".parse().unwrap();
        assert!(t.is_a("a"));
        assert!(t.is_a("a-h-A"));
        assert!(t.is_a("a-.-A"));
        assert!(!t.is_a("a-f-A"));
        assert!(!t.is_a("a-h-A-M-F-Q"));
    }

    #[test]
    fn builds_and_rewrites_atoms() {
        let t = CotType::atom(Affiliation::Friend, Dimension::Ground, "U-C").unwrap();
        assert_eq!(t.as_str(), "a-f-G-U-C");
        assert_eq!(
            t.with_affiliation(Affiliation::Hostile).unwrap().as_str(),
            "a-h-G-U-C"
        );
        assert!("b-t-f"
            .parse::<CotType>()
            .unwrap()
            .with_affiliation(Affiliation::Hostile)
            .is_none());
    }

    #[test]
    fn rejects_invalid() {
        for bad in ["", "-a", "a-", "a--b", "a b", "a-f<G", &"a".repeat(200)] {
            assert!(CotType::new(bad).is_err(), "{bad:?}");
        }
    }
}
