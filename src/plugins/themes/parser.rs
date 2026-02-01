//! Color parsing utilities

/// Parse a hex color string to Hsla
pub(crate) fn parse_color(color: Option<&str>) -> Option<gpui::Hsla> {
    let color = color?;

    // Handle hex colors
    if color.starts_with('#') {
        return parse_hex_color(color);
    }

    // Handle rgba() format
    if color.starts_with("rgba(") || color.starts_with("rgb(") {
        return parse_rgba_color(color);
    }

    None
}

/// Parse a hex color (#RGB, #RRGGBB, #RRGGBBAA)
pub(crate) fn parse_hex_color(hex: &str) -> Option<gpui::Hsla> {
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

    Some(rgb_to_hsla(r, g, b, a))
}

/// Parse rgba(r, g, b, a) format
pub(crate) fn parse_rgba_color(rgba: &str) -> Option<gpui::Hsla> {
    let inner = rgba
        .trim_start_matches("rgba(")
        .trim_start_matches("rgb(")
        .trim_end_matches(')');

    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

    if parts.len() < 3 {
        return None;
    }

    let r: u8 = parts[0].parse().ok()?;
    let g: u8 = parts[1].parse().ok()?;
    let b: u8 = parts[2].parse().ok()?;
    let a: f32 = if parts.len() > 3 {
        parts[3].parse().ok()?
    } else {
        1.0
    };

    Some(rgb_to_hsla(r, g, b, (a * 255.0) as u8))
}

/// Convert RGB to HSLA
fn rgb_to_hsla(r: u8, g: u8, b: u8, a: u8) -> gpui::Hsla {
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

    gpui::hsla(h, s, l, a as f32 / 255.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        let color = parse_hex_color("#ff0000").unwrap();
        assert!((color.h - 0.0).abs() < 0.01);
        assert!((color.s - 1.0).abs() < 0.01);
        assert!((color.l - 0.5).abs() < 0.01);

        let color = parse_hex_color("#00ff00").unwrap();
        assert!((color.h - 0.333).abs() < 0.01);

        let color = parse_hex_color("#0000ff").unwrap();
        assert!((color.h - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_parse_short_hex() {
        let color = parse_hex_color("#f00").unwrap();
        assert!((color.h - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_rgba() {
        let color = parse_rgba_color("rgba(255, 0, 0, 1.0)").unwrap();
        assert!((color.h - 0.0).abs() < 0.01);
        assert!((color.a - 1.0).abs() < 0.01);
    }
}
