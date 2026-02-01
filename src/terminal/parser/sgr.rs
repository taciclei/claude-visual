//! SGR (Select Graphic Rendition) parameter parsing

use super::state::AnsiParser;
use super::types::AnsiColor;

impl AnsiParser {
    /// Apply SGR parameters
    pub(crate) fn apply_sgr_params(&mut self, params: &[u32]) {
        if params.is_empty() {
            self.current_style.reset();
            return;
        }

        let mut i = 0;
        while i < params.len() {
            let param = params[i];

            if param == 38 {
                // Extended foreground color
                if let Some(color) = self.parse_extended_color(params, &mut i) {
                    self.current_style.fg_color = Some(color);
                }
            } else if param == 48 {
                // Extended background color
                if let Some(color) = self.parse_extended_color(params, &mut i) {
                    self.current_style.bg_color = Some(color);
                }
            } else {
                self.current_style.apply_sgr(param as u8);
            }

            i += 1;
        }
    }

    /// Parse extended color (256 or RGB)
    pub(crate) fn parse_extended_color(&self, params: &[u32], i: &mut usize) -> Option<AnsiColor> {
        if *i + 1 >= params.len() {
            return None;
        }

        *i += 1;
        let color_type = params[*i];

        match color_type {
            5 => {
                // 256 color
                if *i + 1 < params.len() {
                    *i += 1;
                    Some(AnsiColor::Indexed(params[*i] as u8))
                } else {
                    None
                }
            }
            2 => {
                // RGB color
                if *i + 3 < params.len() {
                    *i += 1;
                    let r = params[*i] as u8;
                    *i += 1;
                    let g = params[*i] as u8;
                    *i += 1;
                    let b = params[*i] as u8;
                    Some(AnsiColor::Rgb(r, g, b))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
