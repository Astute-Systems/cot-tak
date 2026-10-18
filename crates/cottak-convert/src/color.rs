//! Colour helpers. CoT stores colours as signed 32-bit `ARGB` integers.

use cottak_core::detail::Color;

/// `(r, g, b, a)` of a packed signed ARGB value.
pub(crate) fn rgba(argb: i32) -> (u8, u8, u8, u8) {
    Color { argb }.rgba()
}

/// `#rrggbb`.
pub fn hex(argb: i32) -> String {
    let (r, g, b, _) = rgba(argb);
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// Opacity in `0.0..=1.0`, rounded to two decimals.
pub fn opacity(argb: i32) -> f64 {
    let (_, _, _, a) = rgba(argb);
    (f64::from(a) / 255.0 * 100.0).round() / 100.0
}

/// KML colour: `aabbggrr`.
pub fn kml(argb: i32) -> String {
    let (r, g, b, a) = rgba(argb);
    format!("{a:02x}{b:02x}{g:02x}{r:02x}")
}

/// Parses `#rgb` or `#rrggbb` (the leading `#` is optional).
pub fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    let h = s.trim().trim_start_matches('#');
    let digit = |c: char| c.to_digit(16).and_then(|d| u8::try_from(d).ok());
    let chars: Vec<char> = h.chars().collect();
    match chars.len() {
        3 => {
            let v: Option<Vec<u8>> = chars.iter().map(|&c| digit(c).map(|d| d * 17)).collect();
            let v = v?;
            Some((v[0], v[1], v[2]))
        }
        6 => {
            let v: Option<Vec<u8>> = chars
                .chunks(2)
                .map(|p| Some(digit(p[0])? * 16 + digit(p[1])?))
                .collect();
            let v = v?;
            Some((v[0], v[1], v[2]))
        }
        _ => None,
    }
}

/// Parses a KML colour (`aabbggrr`, optionally with a leading `#`) to CoT's signed ARGB.
pub fn parse_kml(s: &str) -> Option<i32> {
    let h = s.trim().trim_start_matches('#');
    if h.len() != 8 || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let byte = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok();
    Some(Color::from_rgba(byte(6)?, byte(4)?, byte(2)?, byte(0)?).argb)
}

/// Packs a colour and opacity (`0.0..=1.0`, clamped) into CoT's signed ARGB.
pub fn pack(rgb: (u8, u8, u8), opacity: f64) -> i32 {
    // Clamped to 0..=255 first, so the cast cannot truncate.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let a = (opacity.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color::from_rgba(rgb.0, rgb.1, rgb.2, a).argb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let red = Color::from_rgba(255, 0, 0, 255).argb;
        assert_eq!(red, -65536);
        assert_eq!(hex(red), "#ff0000");
        assert_eq!(kml(red), "ff0000ff");
        assert_eq!(opacity(red), 1.0);
        let half = Color::from_rgba(0, 128, 255, 128).argb;
        assert_eq!(opacity(half), 0.5);
        assert_eq!(kml(half), "80ff8000");
    }

    #[test]
    fn kml_colours_round_trip() {
        for argb in [
            -65536,
            0,
            i32::MIN,
            i32::MAX,
            Color::from_rgba(0, 128, 255, 128).argb,
        ] {
            assert_eq!(parse_kml(&kml(argb)), Some(argb), "{argb}");
        }
        assert_eq!(
            parse_kml("#80FF8000"),
            Some(Color::from_rgba(0, 128, 255, 128).argb)
        );
        for bad in ["", "ff00", "gg0000ff", "ff0000ff00", "ff00000é"] {
            assert_eq!(parse_kml(bad), None, "{bad}");
        }
    }

    #[test]
    fn hex_parsing_and_packing_round_trip() {
        assert_eq!(parse_hex("#0af"), Some((0, 170, 255)));
        assert_eq!(parse_hex("FF8000"), Some((255, 128, 0)));
        for bad in ["", "#12", "#gggggg", "#1234567"] {
            assert_eq!(parse_hex(bad), None, "{bad}");
        }
        let packed = pack((10, 20, 30), 0.5);
        assert_eq!(hex(packed), "#0a141e");
        assert_eq!(opacity(packed), 0.5);
        assert_eq!(opacity(pack((0, 0, 0), 7.0)), 1.0, "opacity is clamped");
    }
}
