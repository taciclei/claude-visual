//! Type definitions for ANSI parsing

use serde::{Deserialize, Serialize};

/// ANSI color (standard 16 colors)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    /// 256-color palette index
    Indexed(u8),
    /// RGB color
    Rgb(u8, u8, u8),
    /// Default color
    Default,
}

/// Text style attributes
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextStyle {
    /// Bold/bright
    pub bold: bool,
    /// Dim/faint
    pub dim: bool,
    /// Italic
    pub italic: bool,
    /// Underline
    pub underline: bool,
    /// Blink (slow)
    pub blink: bool,
    /// Reverse video
    pub reverse: bool,
    /// Hidden/invisible
    pub hidden: bool,
    /// Strikethrough
    pub strikethrough: bool,
    /// Foreground color
    pub fg_color: Option<AnsiColor>,
    /// Background color
    pub bg_color: Option<AnsiColor>,
}

/// ANSI event types
#[derive(Debug, Clone, PartialEq)]
pub enum AnsiEvent {
    /// Plain text
    Text(String),
    /// Style change
    Style(TextStyle),
    /// Cursor movement
    CursorMove { row: i32, col: i32 },
    /// Cursor position set
    CursorPosition { row: u32, col: u32 },
    /// Clear screen
    ClearScreen,
    /// Clear line
    ClearLine,
    /// Bell
    Bell,
    /// Set title
    SetTitle(String),
    /// Newline
    Newline,
    /// Carriage return
    CarriageReturn,
    /// Tab
    Tab,
    /// Backspace
    Backspace,
}
