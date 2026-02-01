//! Helper functions for color conversion

use gpui::*;

/// Convert HSLA to hex string
pub(crate) fn hsla_to_hex(color: Hsla) -> String {
    // Convert HSL to RGB
    let c = (1.0 - (2.0 * color.l - 1.0).abs()) * color.s;
    let x = c * (1.0 - ((color.h * 6.0) % 2.0 - 1.0).abs());
    let m = color.l - c / 2.0;

    let (r, g, b) = match (color.h * 6.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    if color.a < 1.0 {
        let a = (color.a * 255.0) as u8;
        format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
    } else {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
}

/// Parse hex color to HSLA
pub(crate) fn parse_hex_color(hex: &str) -> Option<Hsla> {
    let hex = hex.trim_start_matches('#');

    let (r, g, b, a) = match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            (r, g, b, 255u8)
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 255u8)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        _ => return None,
    };

    // Convert RGB to HSL
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    let (h, s) = if delta == 0.0 {
        (0.0, 0.0)
    } else {
        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        let h = if max == r {
            ((g - b) / delta) % 6.0
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };

        let h = h / 6.0;
        let h = if h < 0.0 { h + 1.0 } else { h };

        (h, s)
    };

    Some(hsla(h, s, l, a as f32 / 255.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsla_to_hex() {
        let red = hsla(0.0, 1.0, 0.5, 1.0);
        assert_eq!(hsla_to_hex(red), "#ff0000");

        let green = hsla(1.0 / 3.0, 1.0, 0.5, 1.0);
        assert_eq!(hsla_to_hex(green), "#00ff00");

        let blue = hsla(2.0 / 3.0, 1.0, 0.5, 1.0);
        assert_eq!(hsla_to_hex(blue), "#0000ff");
    }

    #[test]
    fn test_parse_hex_color() {
        let red = parse_hex_color("#ff0000").unwrap();
        assert!((red.h - 0.0).abs() < 0.01);
        assert!((red.s - 1.0).abs() < 0.01);
        assert!((red.l - 0.5).abs() < 0.01);

        let short_red = parse_hex_color("#f00").unwrap();
        assert!((short_red.h - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_hex_roundtrip() {
        let original = hsla(0.5, 0.8, 0.6, 1.0);
        let hex = hsla_to_hex(original);
        let parsed = parse_hex_color(&hex).unwrap();

        assert!((original.h - parsed.h).abs() < 0.02);
        assert!((original.s - parsed.s).abs() < 0.02);
        assert!((original.l - parsed.l).abs() < 0.02);
    }
}
