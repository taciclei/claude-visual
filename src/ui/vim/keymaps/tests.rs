//! Tests for Vim key handling

#[cfg(test)]
mod tests {
    use super::super::{VimAction, VimKeyHandler};

    #[test]
    fn test_basic_movement() {
        let mut handler = VimKeyHandler::new();

        assert_eq!(
            handler.handle_normal("h", 1, None),
            Some(VimAction::MoveLeft(1))
        );
        assert_eq!(
            handler.handle_normal("j", 1, None),
            Some(VimAction::MoveDown(1))
        );
        assert_eq!(
            handler.handle_normal("k", 1, None),
            Some(VimAction::MoveUp(1))
        );
        assert_eq!(
            handler.handle_normal("l", 1, None),
            Some(VimAction::MoveRight(1))
        );
    }

    #[test]
    fn test_mode_changes() {
        let mut handler = VimKeyHandler::new();

        assert_eq!(
            handler.handle_normal("i", 1, None),
            Some(VimAction::EnterInsertMode)
        );
        assert_eq!(
            handler.handle_normal("v", 1, None),
            Some(VimAction::EnterVisualMode)
        );
        assert_eq!(
            handler.handle_normal(":", 1, None),
            Some(VimAction::EnterCommandMode)
        );
    }

    #[test]
    fn test_double_key_commands() {
        let mut handler = VimKeyHandler::new();

        // dd should delete line
        assert_eq!(
            handler.handle_normal("d", 1, None),
            Some(VimAction::SetOperator('d'))
        );
        assert_eq!(
            handler.handle_normal("d", 1, Some('d')),
            Some(VimAction::DeleteLine)
        );
    }

    #[test]
    fn test_gg_command() {
        let mut handler = VimKeyHandler::new();

        assert_eq!(handler.handle_normal("g", 1, None), None); // Wait for next key
        assert_eq!(
            handler.handle_normal("g", 1, None),
            Some(VimAction::MoveToTop)
        );
    }

    #[test]
    fn test_count_with_movement() {
        let mut handler = VimKeyHandler::new();

        assert_eq!(
            handler.handle_normal("j", 5, None),
            Some(VimAction::MoveDown(5))
        );
    }
}
