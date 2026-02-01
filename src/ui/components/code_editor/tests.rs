//! Tests for code editor components

use super::*;

#[test]
fn test_editor_themes() {
    let dark = EditorTheme::Dark;
    let light = EditorTheme::Light;

    // Dark theme has dark background, light has light background
    assert!(dark.background().l < 0.5);
    assert!(light.background().l > 0.5);
}

#[test]
fn test_editor_font_sizes() {
    let sm = EditorFontSize::Sm;
    let lg = EditorFontSize::Lg;

    assert!(sm.size() < lg.size());
    assert!(sm.line_height() < lg.line_height());
}

#[test]
fn test_editor_line() {
    let line = EditorLine::new(1, "let x = 5;")
        .modified()
        .breakpoint()
        .current();

    assert!(line.is_modified);
    assert!(line.is_breakpoint);
    assert!(line.is_current);
}

#[test]
fn test_selection() {
    let sel = Selection::new(1, 0, 3, 10);
    let cursor = Selection::cursor(5, 12);

    assert_eq!(sel.start_line, 1);
    assert_eq!(sel.end_line, 3);
    assert_eq!(cursor.start_line, cursor.end_line);
}

#[test]
fn test_code_editor() {
    let editor = CodeEditor::new("ce")
        .content("fn main() {\n    println!(\"Hello\");\n}")
        .theme(EditorTheme::Monokai)
        .show_minimap(true);

    assert_eq!(editor.lines.len(), 3);
    assert!(editor.show_minimap);
}

#[test]
fn test_diff_editor() {
    let diff = DiffEditor::new("de")
        .left_content("line 1\nline 2")
        .right_content("line 1\nline 2 modified\nline 3")
        .additions(vec![2, 3])
        .deletions(vec![2]);

    assert_eq!(diff.left_lines.len(), 2);
    assert_eq!(diff.right_lines.len(), 3);
    assert_eq!(diff.additions.len(), 2);
}
