//! Project database operations

use std::path::PathBuf;

use anyhow::Result;
use rusqlite::params;

use crate::project::manager::Project;

use super::Database;

impl Database {
    /// Insert a new project
    pub fn insert_project(&self, project: &Project) -> Result<()> {
        let tags_json = serde_json::to_string(&project.tags)?;
        self.conn.execute(
            "INSERT INTO projects (id, name, path, is_favorite, tags, last_accessed, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                project.id,
                project.name,
                project.path.to_string_lossy().to_string(),
                project.is_favorite as i32,
                tags_json,
                project.last_accessed.to_rfc3339(),
                project.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// Get a project by ID
    pub fn get_project(&self, id: &str) -> Result<Option<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, is_favorite, tags, last_accessed, created_at FROM projects WHERE id = ?1",
        )?;

        let project = stmt.query_row(params![id], |row| {
            let tags_json: String = row.get(4)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            let last_accessed: String = row.get(5)?;
            let created_at: String = row.get(6)?;

            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(row.get::<_, String>(2)?),
                is_favorite: row.get::<_, i32>(3)? != 0,
                tags,
                last_accessed: chrono::DateTime::parse_from_rfc3339(&last_accessed)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                theme_override: None,
            })
        });

        match project {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// List all projects
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, is_favorite, tags, last_accessed, created_at FROM projects ORDER BY last_accessed DESC",
        )?;

        let projects = stmt
            .query_map([], |row| {
                let tags_json: String = row.get(4)?;
                let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
                let last_accessed: String = row.get(5)?;
                let created_at: String = row.get(6)?;

                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: PathBuf::from(row.get::<_, String>(2)?),
                    is_favorite: row.get::<_, i32>(3)? != 0,
                    tags,
                    last_accessed: chrono::DateTime::parse_from_rfc3339(&last_accessed)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    theme_override: None,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    /// Update a project
    pub fn update_project(&self, project: &Project) -> Result<()> {
        let tags_json = serde_json::to_string(&project.tags)?;
        self.conn.execute(
            "UPDATE projects SET name = ?1, path = ?2, is_favorite = ?3, tags = ?4, last_accessed = ?5 WHERE id = ?6",
            params![
                project.name,
                project.path.to_string_lossy().to_string(),
                project.is_favorite as i32,
                tags_json,
                project.last_accessed.to_rfc3339(),
                project.id,
            ],
        )?;
        Ok(())
    }

    /// Delete a project
    pub fn delete_project(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM projects WHERE id = ?1", params![id])?;
        Ok(())
    }
}
