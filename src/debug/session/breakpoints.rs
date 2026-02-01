//! Breakpoint management operations

use std::collections::HashMap;
use std::path::PathBuf;

use super::super::client::DapClientError;
use super::super::protocol::*;
use super::core::DebugSession;
use super::types::UserBreakpoint;

impl DebugSession {
    /// Add a breakpoint
    pub fn add_breakpoint(&mut self, file: PathBuf, line: i64) -> usize {
        let id = self.next_breakpoint_id;
        self.next_breakpoint_id += 1;

        let mut bp = UserBreakpoint::new(file, line);
        bp.id = id;

        self.breakpoints.insert(id, bp);

        id
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, id: usize) -> Option<UserBreakpoint> {
        self.breakpoints.remove(&id)
    }

    /// Toggle breakpoint enabled state
    pub fn toggle_breakpoint(&mut self, id: usize) {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.enabled = !bp.enabled;
        }
    }
}

/// Sync breakpoints with adapter (internal helper)
pub(super) async fn sync_breakpoints(session: &mut DebugSession) -> Result<(), DapClientError> {
    // Group breakpoints by file - clone to avoid borrowing issues
    let breakpoints_clone: Vec<_> = session.breakpoints.values().cloned().collect();
    let mut by_file: HashMap<PathBuf, Vec<UserBreakpoint>> = HashMap::new();

    for bp in breakpoints_clone {
        if bp.enabled {
            by_file.entry(bp.file.clone()).or_default().push(bp);
        }
    }

    // Set breakpoints for each file
    for (file, bps) in by_file {
        let source = Source {
            name: file.file_name().map(|n| n.to_string_lossy().to_string()),
            path: Some(file.to_string_lossy().to_string()),
            source_reference: None,
            presentation_hint: None,
            origin: None,
        };

        let locations: Vec<BreakpointLocation> = bps
            .iter()
            .map(|bp| BreakpointLocation {
                line: bp.line,
                column: None,
                end_line: None,
                end_column: None,
            })
            .collect();

        let verified = session.client.set_breakpoints(source, locations).await?;

        // Update verified status - collect IDs first to avoid borrowing conflicts
        let updates: Vec<(usize, bool)> = bps.iter().enumerate()
            .map(|(i, bp)| (bp.id, verified.get(i).map(|v| v.verified).unwrap_or(false)))
            .collect();

        for (id, is_verified) in updates {
            if let Some(user_bp) = session.breakpoints.get_mut(&id) {
                user_bp.verified = is_verified;
            }
        }
    }

    Ok(())
}
