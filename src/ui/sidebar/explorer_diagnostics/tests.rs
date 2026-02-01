//! Tests for explorer diagnostics

use crate::lsp::protocol::{Diagnostic, DiagnosticSeverity, Position, Range};
use crate::ui::sidebar::explorer_diagnostics::{
    BadgeStyle, DiagnosticBadge, DiagnosticCounts, ExplorerDiagnosticsStore, IconDecoration,
};
use std::path::{Path, PathBuf};

fn make_diagnostic(severity: DiagnosticSeverity) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 1,
            },
        },
        severity: Some(severity),
        code: None,
        source: None,
        message: "test".to_string(),
        related_information: None,
    }
}

#[test]
fn test_diagnostic_counts() {
    let diagnostics = vec![
        make_diagnostic(DiagnosticSeverity::Error),
        make_diagnostic(DiagnosticSeverity::Error),
        make_diagnostic(DiagnosticSeverity::Warning),
        make_diagnostic(DiagnosticSeverity::Information),
    ];

    let counts = DiagnosticCounts::from_diagnostics(&diagnostics);
    assert_eq!(counts.errors, 2);
    assert_eq!(counts.warnings, 1);
    assert_eq!(counts.info, 1);
    assert!(counts.has_problems());
    assert_eq!(counts.total(), 4);
}

#[test]
fn test_store_update() {
    let mut store = ExplorerDiagnosticsStore::new();

    let path = PathBuf::from("/test/file.rs");
    let diagnostics = vec![make_diagnostic(DiagnosticSeverity::Error)];

    store.update_file(path.clone(), diagnostics);

    let counts = store.get_counts(&path);
    assert_eq!(counts.errors, 1);
}

#[test]
fn test_parent_aggregation() {
    let mut store = ExplorerDiagnosticsStore::new();

    store.update_file(
        PathBuf::from("/project/src/file1.rs"),
        vec![make_diagnostic(DiagnosticSeverity::Error)],
    );
    store.update_file(
        PathBuf::from("/project/src/file2.rs"),
        vec![make_diagnostic(DiagnosticSeverity::Warning)],
    );

    let src_counts = store.get_counts(Path::new("/project/src"));
    assert_eq!(src_counts.errors, 1);
    assert_eq!(src_counts.warnings, 1);

    let project_counts = store.get_counts(Path::new("/project"));
    assert_eq!(project_counts.errors, 1);
    assert_eq!(project_counts.warnings, 1);
}

#[test]
fn test_badge_text() {
    let badge = DiagnosticBadge {
        errors: 3,
        warnings: 2,
        style: BadgeStyle::Count,
    };
    assert_eq!(badge.text(), "3/2");

    let errors_only = DiagnosticBadge {
        errors: 5,
        warnings: 0,
        style: BadgeStyle::Count,
    };
    assert_eq!(errors_only.text(), "5");
}

#[test]
fn test_icon_decoration() {
    let mut counts = DiagnosticCounts::default();
    assert_eq!(IconDecoration::from_counts(&counts), IconDecoration::None);

    counts.errors = 1;
    assert_eq!(IconDecoration::from_counts(&counts), IconDecoration::Error);

    counts.errors = 0;
    counts.warnings = 1;
    assert_eq!(
        IconDecoration::from_counts(&counts),
        IconDecoration::Warning
    );

    counts.errors = 1;
    assert_eq!(IconDecoration::from_counts(&counts), IconDecoration::Mixed);
}
