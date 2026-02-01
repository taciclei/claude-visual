//! Inline code display components

mod types;
mod inline_code;
mod variable;
mod command;
mod file_path;
mod url_display;
mod key_value;
mod json_value;

pub use types::{CodeSize, JsonType};
pub use inline_code::InlineCode;
pub use variable::Variable;
pub use command::Command;
pub use file_path::FilePath;
pub use url_display::UrlDisplay;
pub use key_value::KeyValue;
pub use json_value::JsonValue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_code() {
        let code = InlineCode::new("let x = 42")
            .size(CodeSize::Large)
            .copyable();

        assert_eq!(code.code, "let x = 42");
        assert!(code.copyable);
    }

    #[test]
    fn test_variable() {
        let var = Variable::new("count")
            .with_value("42");

        assert_eq!(var.name, "count");
        assert_eq!(var.value, Some("42".to_string()));
    }

    #[test]
    fn test_command() {
        let cmd = Command::new("cargo build --release")
            .shell(">");

        assert_eq!(cmd.command, "cargo build --release");
        assert_eq!(cmd.shell, Some(">".to_string()));
    }

    #[test]
    fn test_url_display() {
        let url = UrlDisplay::new("https://example.com/very/long/path/to/some/resource");
        let display = url.display_url();
        assert!(display.len() <= 43); // 20 + ... + 15 = 38 + 3 for ...
    }

    #[test]
    fn test_json_value() {
        let _string = JsonValue::string("hello");
        let _number = JsonValue::number(42.0);
        let _bool = JsonValue::boolean(true);
        let _null = JsonValue::null();
        let _array = JsonValue::array(5);
        let _object = JsonValue::object(3);
    }
}
