//! Vim actions that can be triggered by key sequences

/// Vim actions that can be triggered by key sequences
#[derive(Debug, Clone, PartialEq)]
pub enum VimAction {
    // Mode transitions
    EnterInsertMode,           // i
    EnterInsertModeAppend,     // a
    EnterInsertModeLineStart,  // I
    EnterInsertModeLineEnd,    // A
    EnterInsertModeNewLineBelow, // o
    EnterInsertModeNewLineAbove, // O
    ExitInsertMode,            // Esc
    EnterVisualMode,           // v
    EnterVisualLineMode,       // V
    EnterVisualBlockMode,      // Ctrl+V
    ExitVisualMode,            // Esc
    EnterCommandMode,          // :
    EnterSearchMode,           // /

    // Cursor movement
    MoveLeft(usize),           // h
    MoveDown(usize),           // j
    MoveUp(usize),             // k
    MoveRight(usize),          // l
    MoveWordForward(usize),    // w
    MoveWordBackward(usize),   // b
    MoveWordEnd(usize),        // e
    MoveLineStart,             // 0
    MoveLineFirstNonBlank,     // ^
    MoveLineEnd,               // $
    MoveToTop,                 // gg
    MoveToBottom,              // G
    MoveToLine(usize),         // {n}G
    MoveHalfPageDown,          // Ctrl+D
    MoveHalfPageUp,            // Ctrl+U
    MovePageDown,              // Ctrl+F
    MovePageUp,                // Ctrl+B

    // Text manipulation
    Delete,                    // d (with motion)
    DeleteLine,                // dd
    DeleteChar,                // x
    DeleteCharBefore,          // X
    Change,                    // c (with motion)
    ChangeLine,                // cc
    ChangeToEnd,               // C
    Yank,                      // y (with motion)
    YankLine,                  // yy
    Put,                       // p (paste after)
    PutBefore,                 // P (paste before)
    Undo,                      // u
    Redo,                      // Ctrl+R
    Replace(char),             // r{char}
    Join,                      // J (join lines)

    // Search & navigation
    SearchForward(String),     // /{pattern}
    SearchBackward(String),    // ?{pattern}
    SearchNext,                // n
    SearchPrev,                // N
    FindChar(char),            // f{char}
    FindCharBack(char),        // F{char}
    TillChar(char),            // t{char}
    TillCharBack(char),        // T{char}

    // Commands
    ExecuteCommand(String),    // :{command}
    ExecuteSearch(String),     // /{pattern} Enter
    CancelCommand,             // Esc in command mode
    CancelSearch,              // Esc in search mode

    // Operators (wait for motion)
    SetOperator(char),         // d, y, c, etc.

    // Misc
    Repeat,                    // . (repeat last change)
    Indent,                    // >>
    Outdent,                   // <<
    ToggleCase,                // ~
    IncrementNumber,           // Ctrl+A
    DecrementNumber,           // Ctrl+X
}
