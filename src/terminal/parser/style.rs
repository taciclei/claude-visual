//! Text style handling and SGR parameter application

use super::types::{AnsiColor, TextStyle};

impl TextStyle {
    /// Reset to default
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Apply SGR parameter
    pub fn apply_sgr(&mut self, param: u8) {
        match param {
            0 => self.reset(),
            1 => self.bold = true,
            2 => self.dim = true,
            3 => self.italic = true,
            4 => self.underline = true,
            5 | 6 => self.blink = true,
            7 => self.reverse = true,
            8 => self.hidden = true,
            9 => self.strikethrough = true,
            21 | 22 => {
                self.bold = false;
                self.dim = false;
            }
            23 => self.italic = false,
            24 => self.underline = false,
            25 => self.blink = false,
            27 => self.reverse = false,
            28 => self.hidden = false,
            29 => self.strikethrough = false,
            // Foreground colors
            30 => self.fg_color = Some(AnsiColor::Black),
            31 => self.fg_color = Some(AnsiColor::Red),
            32 => self.fg_color = Some(AnsiColor::Green),
            33 => self.fg_color = Some(AnsiColor::Yellow),
            34 => self.fg_color = Some(AnsiColor::Blue),
            35 => self.fg_color = Some(AnsiColor::Magenta),
            36 => self.fg_color = Some(AnsiColor::Cyan),
            37 => self.fg_color = Some(AnsiColor::White),
            39 => self.fg_color = None, // Default
            // Background colors
            40 => self.bg_color = Some(AnsiColor::Black),
            41 => self.bg_color = Some(AnsiColor::Red),
            42 => self.bg_color = Some(AnsiColor::Green),
            43 => self.bg_color = Some(AnsiColor::Yellow),
            44 => self.bg_color = Some(AnsiColor::Blue),
            45 => self.bg_color = Some(AnsiColor::Magenta),
            46 => self.bg_color = Some(AnsiColor::Cyan),
            47 => self.bg_color = Some(AnsiColor::White),
            49 => self.bg_color = None, // Default
            // Bright foreground colors
            90 => self.fg_color = Some(AnsiColor::BrightBlack),
            91 => self.fg_color = Some(AnsiColor::BrightRed),
            92 => self.fg_color = Some(AnsiColor::BrightGreen),
            93 => self.fg_color = Some(AnsiColor::BrightYellow),
            94 => self.fg_color = Some(AnsiColor::BrightBlue),
            95 => self.fg_color = Some(AnsiColor::BrightMagenta),
            96 => self.fg_color = Some(AnsiColor::BrightCyan),
            97 => self.fg_color = Some(AnsiColor::BrightWhite),
            // Bright background colors
            100 => self.bg_color = Some(AnsiColor::BrightBlack),
            101 => self.bg_color = Some(AnsiColor::BrightRed),
            102 => self.bg_color = Some(AnsiColor::BrightGreen),
            103 => self.bg_color = Some(AnsiColor::BrightYellow),
            104 => self.bg_color = Some(AnsiColor::BrightBlue),
            105 => self.bg_color = Some(AnsiColor::BrightMagenta),
            106 => self.bg_color = Some(AnsiColor::BrightCyan),
            107 => self.bg_color = Some(AnsiColor::BrightWhite),
            _ => {}
        }
    }
}
