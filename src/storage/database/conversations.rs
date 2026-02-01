//! Conversation database operations

use anyhow::Result;
use rusqlite::params;

use crate::storage::models::Conversation;

use super::Database;
use super::helpers::row_to_conversation;

impl Database {
    /// Insert a new conversation
    pub fn insert_conversation(&self, conversation: &Conversation) -> Result<()> {
        self.conn.execute(
            "INSERT INTO conversations (id, project_id, title, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                conversation.id,
                conversation.project_id,
                conversation.title,
                conversation.created_at.to_rfc3339(),
                conversation.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// Get conversations for a project
    pub fn get_conversations(&self, project_id: Option<&str>) -> Result<Vec<Conversation>> {
        let mut stmt = if project_id.is_some() {
            self.conn.prepare(
                "SELECT id, project_id, title, created_at, updated_at FROM conversations WHERE project_id = ?1 ORDER BY updated_at DESC",
            )?
        } else {
            self.conn.prepare(
                "SELECT id, project_id, title, created_at, updated_at FROM conversations ORDER BY updated_at DESC",
            )?
        };

        let conversations = if let Some(pid) = project_id {
            stmt.query_map(params![pid], row_to_conversation)?
        } else {
            stmt.query_map([], row_to_conversation)?
        };

        conversations.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Delete a conversation
    pub fn delete_conversation(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM conversations WHERE id = ?1", params![id])?;
        Ok(())
    }
}
