//! Message database operations

use anyhow::Result;
use rusqlite::params;

use crate::storage::models::{Message, SearchResult};

use super::Database;

impl Database {
    /// Insert a message
    pub fn insert_message(&self, message: &Message) -> Result<()> {
        self.conn.execute(
            "INSERT INTO messages (id, conversation_id, role, content, tool_name, is_error, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                message.id,
                message.conversation_id,
                message.role,
                message.content,
                message.tool_name,
                message.is_error as i32,
                message.timestamp.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// Get messages for a conversation
    pub fn get_messages(&self, conversation_id: &str) -> Result<Vec<Message>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, conversation_id, role, content, tool_name, is_error, timestamp FROM messages WHERE conversation_id = ?1 ORDER BY timestamp",
        )?;

        let messages = stmt
            .query_map(params![conversation_id], |row| {
                let timestamp: String = row.get(6)?;
                Ok(Message {
                    id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    tool_name: row.get(4)?,
                    is_error: row.get::<_, i32>(5)? != 0,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&timestamp)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(messages)
    }

    /// Search messages using full-text search
    pub fn search_messages(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Escape special FTS5 characters and create search query
        let search_query = query
            .replace('"', "\"\"")
            .split_whitespace()
            .map(|word| format!("\"{}\"*", word))
            .collect::<Vec<_>>()
            .join(" ");

        let mut stmt = self.conn.prepare(
            r#"
            SELECT
                m.id,
                m.conversation_id,
                m.role,
                m.content,
                m.tool_name,
                m.is_error,
                m.timestamp,
                c.title,
                highlight(messages_fts, 0, '<mark>', '</mark>') as highlighted,
                rank
            FROM messages_fts
            JOIN messages m ON messages_fts.rowid = m.rowid
            JOIN conversations c ON m.conversation_id = c.id
            WHERE messages_fts MATCH ?1
            ORDER BY rank
            LIMIT ?2
            "#,
        )?;

        let results = stmt
            .query_map(params![search_query, limit], |row| {
                let timestamp: String = row.get(6)?;
                let message = Message {
                    id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    tool_name: row.get(4)?,
                    is_error: row.get::<_, i32>(5)? != 0,
                    timestamp: chrono::DateTime::parse_from_rfc3339(&timestamp)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                };

                Ok(SearchResult {
                    message,
                    conversation_title: row.get(7)?,
                    highlighted: row.get(8)?,
                    rank: row.get(9)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(results)
    }

    /// Search messages with optional filters
    pub fn search_messages_with_filter(
        &self,
        query: &str,
        filter: &crate::storage::models::SearchFilter,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        use crate::storage::models::DateRangeFilter;

        // Escape special FTS5 characters and create search query
        let search_query = query
            .replace('"', "\"\"")
            .split_whitespace()
            .map(|word| format!("\"{}\"*", word))
            .collect::<Vec<_>>()
            .join(" ");

        // Build WHERE clauses based on filters
        let mut where_clauses = vec!["messages_fts MATCH ?1".to_string()];
        let mut param_index = 2;

        // Date filter
        let date_param = filter.date_range.start_date().map(|d| d.to_rfc3339());
        if date_param.is_some() {
            where_clauses.push(format!("m.timestamp >= ?{}", param_index));
            param_index += 1;
        }

        // Project filter
        if filter.project_id.is_some() {
            where_clauses.push(format!("c.project_id = ?{}", param_index));
            param_index += 1;
        }

        let where_clause = where_clauses.join(" AND ");
        let limit_param = format!("?{}", param_index);

        let sql = format!(
            r#"
            SELECT
                m.id,
                m.conversation_id,
                m.role,
                m.content,
                m.tool_name,
                m.is_error,
                m.timestamp,
                c.title,
                highlight(messages_fts, 0, '<mark>', '</mark>') as highlighted,
                rank
            FROM messages_fts
            JOIN messages m ON messages_fts.rowid = m.rowid
            JOIN conversations c ON m.conversation_id = c.id
            WHERE {}
            ORDER BY rank
            LIMIT {}
            "#,
            where_clause, limit_param
        );

        let mut stmt = self.conn.prepare(&sql)?;

        // Build params dynamically
        let results = match (&date_param, &filter.project_id) {
            (None, None) => {
                stmt.query_map(params![search_query, limit], |row| {
                    Self::parse_search_row(row)
                })?
                .collect::<std::result::Result<Vec<_>, _>>()?
            }
            (Some(date), None) => {
                stmt.query_map(params![search_query, date, limit], |row| {
                    Self::parse_search_row(row)
                })?
                .collect::<std::result::Result<Vec<_>, _>>()?
            }
            (None, Some(project)) => {
                stmt.query_map(params![search_query, project, limit], |row| {
                    Self::parse_search_row(row)
                })?
                .collect::<std::result::Result<Vec<_>, _>>()?
            }
            (Some(date), Some(project)) => {
                stmt.query_map(params![search_query, date, project, limit], |row| {
                    Self::parse_search_row(row)
                })?
                .collect::<std::result::Result<Vec<_>, _>>()?
            }
        };

        Ok(results)
    }

    /// Parse a search result row
    fn parse_search_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SearchResult> {
        let timestamp: String = row.get(6)?;
        let message = Message {
            id: row.get(0)?,
            conversation_id: row.get(1)?,
            role: row.get(2)?,
            content: row.get(3)?,
            tool_name: row.get(4)?,
            is_error: row.get::<_, i32>(5)? != 0,
            timestamp: chrono::DateTime::parse_from_rfc3339(&timestamp)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        };

        Ok(SearchResult {
            message,
            conversation_title: row.get(7)?,
            highlighted: row.get(8)?,
            rank: row.get(9)?,
        })
    }

    /// Rebuild FTS index (useful after importing data)
    pub fn rebuild_fts_index(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            INSERT INTO messages_fts(messages_fts) VALUES('rebuild');
            "#,
        )?;
        Ok(())
    }
}
