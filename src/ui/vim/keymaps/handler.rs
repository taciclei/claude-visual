//! Vim key handler for processing key sequences

use super::actions::VimAction;

/// Handles key sequences and converts them to Vim actions
pub struct VimKeyHandler {
    /// Buffer for multi-key sequences (e.g., "dd", "gg")
    key_buffer: String,
}

impl VimKeyHandler {
    pub fn new() -> Self {
        Self {
            key_buffer: String::new(),
        }
    }

    /// Handle a key in normal mode
    pub fn handle_normal(
        &mut self,
        key: &str,
        count: usize,
        pending_op: Option<char>,
    ) -> Option<VimAction> {
        // If we have a pending operator, this key is the motion
        if let Some(op) = pending_op {
            return self.handle_operator_motion(op, key, count);
        }

        // Check for multi-key sequences
        self.key_buffer.push_str(key);

        let action = match self.key_buffer.as_str() {
            // Mode changes
            "i" => Some(VimAction::EnterInsertMode),
            "a" => Some(VimAction::EnterInsertModeAppend),
            "I" => Some(VimAction::EnterInsertModeLineStart),
            "A" => Some(VimAction::EnterInsertModeLineEnd),
            "o" => Some(VimAction::EnterInsertModeNewLineBelow),
            "O" => Some(VimAction::EnterInsertModeNewLineAbove),
            "v" => Some(VimAction::EnterVisualMode),
            "V" => Some(VimAction::EnterVisualLineMode),
            ":" => Some(VimAction::EnterCommandMode),
            "/" => Some(VimAction::EnterSearchMode),

            // Movement
            "h" => Some(VimAction::MoveLeft(count)),
            "j" => Some(VimAction::MoveDown(count)),
            "k" => Some(VimAction::MoveUp(count)),
            "l" => Some(VimAction::MoveRight(count)),
            "w" => Some(VimAction::MoveWordForward(count)),
            "b" => Some(VimAction::MoveWordBackward(count)),
            "e" => Some(VimAction::MoveWordEnd(count)),
            "0" => Some(VimAction::MoveLineStart),
            "^" => Some(VimAction::MoveLineFirstNonBlank),
            "$" => Some(VimAction::MoveLineEnd),
            "G" => {
                if count > 1 {
                    Some(VimAction::MoveToLine(count))
                } else {
                    Some(VimAction::MoveToBottom)
                }
            }
            "gg" => Some(VimAction::MoveToTop),

            // Text manipulation
            "x" => Some(VimAction::DeleteChar),
            "X" => Some(VimAction::DeleteCharBefore),
            "dd" => Some(VimAction::DeleteLine),
            "yy" => Some(VimAction::YankLine),
            "cc" => Some(VimAction::ChangeLine),
            "C" => Some(VimAction::ChangeToEnd),
            "p" => Some(VimAction::Put),
            "P" => Some(VimAction::PutBefore),
            "u" => Some(VimAction::Undo),
            "J" => Some(VimAction::Join),
            "." => Some(VimAction::Repeat),
            "~" => Some(VimAction::ToggleCase),
            ">>" => Some(VimAction::Indent),
            "<<" => Some(VimAction::Outdent),

            // Operators (wait for motion)
            "d" => Some(VimAction::SetOperator('d')),
            "y" => Some(VimAction::SetOperator('y')),
            "c" => Some(VimAction::SetOperator('c')),
            ">" => {
                // Could be >> or > + motion
                self.key_buffer.clear();
                self.key_buffer.push('>');
                return None; // Wait for next key
            }
            "<" => {
                self.key_buffer.clear();
                self.key_buffer.push('<');
                return None;
            }
            "g" => {
                // Could be gg, gj, gk, etc.
                return None; // Wait for next key
            }

            // Search navigation
            "n" => Some(VimAction::SearchNext),
            "N" => Some(VimAction::SearchPrev),

            // Ctrl sequences
            "ctrl-d" => Some(VimAction::MoveHalfPageDown),
            "ctrl-u" => Some(VimAction::MoveHalfPageUp),
            "ctrl-f" => Some(VimAction::MovePageDown),
            "ctrl-b" => Some(VimAction::MovePageUp),
            "ctrl-r" => Some(VimAction::Redo),
            "ctrl-v" => Some(VimAction::EnterVisualBlockMode),
            "ctrl-a" => Some(VimAction::IncrementNumber),
            "ctrl-x" => Some(VimAction::DecrementNumber),

            _ => {
                // Check for r{char} (replace)
                if self.key_buffer.starts_with('r') && self.key_buffer.len() == 2 {
                    let ch = self.key_buffer.chars().nth(1).unwrap();
                    self.key_buffer.clear();
                    return Some(VimAction::Replace(ch));
                }
                // Check for f{char}, F{char}, t{char}, T{char}
                if self.key_buffer.len() == 2 {
                    let first = self.key_buffer.chars().next().unwrap();
                    let second = self.key_buffer.chars().nth(1).unwrap();
                    self.key_buffer.clear();
                    return match first {
                        'f' => Some(VimAction::FindChar(second)),
                        'F' => Some(VimAction::FindCharBack(second)),
                        't' => Some(VimAction::TillChar(second)),
                        'T' => Some(VimAction::TillCharBack(second)),
                        _ => None,
                    };
                }
                // Unknown sequence, might be incomplete
                if self.key_buffer.len() < 3 {
                    return None; // Wait for more keys
                }
                // Too long, reset
                self.key_buffer.clear();
                None
            }
        };

        if action.is_some() {
            self.key_buffer.clear();
        }

        action
    }

    /// Handle motion after an operator (d, y, c)
    fn handle_operator_motion(
        &mut self,
        operator: char,
        motion: &str,
        count: usize,
    ) -> Option<VimAction> {
        // Check for special cases like dd, yy, cc
        if motion.len() == 1 && motion.chars().next() == Some(operator) {
            return match operator {
                'd' => Some(VimAction::DeleteLine),
                'y' => Some(VimAction::YankLine),
                'c' => Some(VimAction::ChangeLine),
                _ => None,
            };
        }

        // Parse the motion
        let motion_action = self.handle_normal(motion, count, None)?;

        // Combine operator with motion
        match operator {
            'd' => Some(VimAction::Delete),
            'y' => Some(VimAction::Yank),
            'c' => Some(VimAction::Change),
            _ => Some(motion_action),
        }
    }

    /// Handle a key in visual mode
    pub fn handle_visual(&mut self, key: &str, count: usize) -> Option<VimAction> {
        match key {
            // Movement (same as normal mode)
            "h" => Some(VimAction::MoveLeft(count)),
            "j" => Some(VimAction::MoveDown(count)),
            "k" => Some(VimAction::MoveUp(count)),
            "l" => Some(VimAction::MoveRight(count)),
            "w" => Some(VimAction::MoveWordForward(count)),
            "b" => Some(VimAction::MoveWordBackward(count)),
            "e" => Some(VimAction::MoveWordEnd(count)),
            "0" => Some(VimAction::MoveLineStart),
            "^" => Some(VimAction::MoveLineFirstNonBlank),
            "$" => Some(VimAction::MoveLineEnd),
            "G" => Some(VimAction::MoveToBottom),
            "gg" => Some(VimAction::MoveToTop),

            // Actions on selection
            "d" | "x" => Some(VimAction::Delete),
            "y" => Some(VimAction::Yank),
            "c" => Some(VimAction::Change),
            ">" => Some(VimAction::Indent),
            "<" => Some(VimAction::Outdent),
            "~" => Some(VimAction::ToggleCase),
            "J" => Some(VimAction::Join),

            // Mode switches
            "v" => Some(VimAction::EnterVisualMode),
            "V" => Some(VimAction::EnterVisualLineMode),

            _ => None,
        }
    }
}

impl Default for VimKeyHandler {
    fn default() -> Self {
        Self::new()
    }
}
