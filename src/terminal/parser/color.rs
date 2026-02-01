//! Color conversion and handling

use super::types::AnsiColor;

impl AnsiColor {
    /// Convert to RGB tuple
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            AnsiColor::Black => (0, 0, 0),
            AnsiColor::Red => (205, 49, 49),
            AnsiColor::Green => (13, 188, 121),
            AnsiColor::Yellow => (229, 229, 16),
            AnsiColor::Blue => (36, 114, 200),
            AnsiColor::Magenta => (188, 63, 188),
            AnsiColor::Cyan => (17, 168, 205),
            AnsiColor::White => (229, 229, 229),
            AnsiColor::BrightBlack => (102, 102, 102),
            AnsiColor::BrightRed => (241, 76, 76),
            AnsiColor::BrightGreen => (35, 209, 139),
            AnsiColor::BrightYellow => (245, 245, 67),
            AnsiColor::BrightBlue => (59, 142, 234),
            AnsiColor::BrightMagenta => (214, 112, 214),
            AnsiColor::BrightCyan => (41, 184, 219),
            AnsiColor::BrightWhite => (255, 255, 255),
            AnsiColor::Indexed(idx) => Self::indexed_to_rgb(*idx),
            AnsiColor::Rgb(r, g, b) => (*r, *g, *b),
            AnsiColor::Default => (229, 229, 229),
        }
    }

    /// Convert 256-color index to RGB
    fn indexed_to_rgb(idx: u8) -> (u8, u8, u8) {
        if idx < 16 {
            // Standard colors
            let color = match idx {
                0 => AnsiColor::Black,
                1 => AnsiColor::Red,
                2 => AnsiColor::Green,
                3 => AnsiColor::Yellow,
                4 => AnsiColor::Blue,
                5 => AnsiColor::Magenta,
                6 => AnsiColor::Cyan,
                7 => AnsiColor::White,
                8 => AnsiColor::BrightBlack,
                9 => AnsiColor::BrightRed,
                10 => AnsiColor::BrightGreen,
                11 => AnsiColor::BrightYellow,
                12 => AnsiColor::BrightBlue,
                13 => AnsiColor::BrightMagenta,
                14 => AnsiColor::BrightCyan,
                15 => AnsiColor::BrightWhite,
                _ => AnsiColor::Default,
            };
            color.to_rgb()
        } else if idx < 232 {
            // 216-color cube (6x6x6)
            let idx = idx - 16;
            let r = (idx / 36) % 6;
            let g = (idx / 6) % 6;
            let b = idx % 6;
            let to_val = |v: u8| if v == 0 { 0 } else { 55 + 40 * v };
            (to_val(r), to_val(g), to_val(b))
        } else {
            // Grayscale (24 shades)
            let gray = (idx - 232) * 10 + 8;
            (gray, gray, gray)
        }
    }
}
