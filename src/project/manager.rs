//! Project CRUD operations

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::app::theme::ThemeVariant;
use crate::storage::database::Database;

/// A project in Claude Visual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Path to project root
    pub path: PathBuf,
    /// Whether this is a favorite
    pub is_favorite: bool,
    /// Tags for organization
    pub tags: Vec<String>,
    /// Last accessed timestamp
    pub last_accessed: DateTime<Utc>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Optional theme override for this project
    #[serde(default)]
    pub theme_override: Option<ThemeVariant>,
}

impl Project {
    /// Create a new project
    pub fn new(name: impl Into<String>, path: PathBuf) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            path,
            is_favorite: false,
            tags: Vec::new(),
            last_accessed: Utc::now(),
            created_at: Utc::now(),
            theme_override: None,
        }
    }

    /// Create from a path, using the directory name as the project name
    pub fn from_path(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        Self::new(name, path)
    }
}

/// Manager for project CRUD operations
pub struct ProjectManager {
    database: Arc<Database>,
}

impl ProjectManager {
    /// Create a new project manager
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    /// List all projects
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        self.database.list_projects()
    }

    /// List recent projects
    pub fn list_recent(&self, limit: usize) -> Result<Vec<Project>> {
        let mut projects = self.database.list_projects()?;
        projects.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        projects.truncate(limit);
        Ok(projects)
    }

    /// List favorite projects
    pub fn list_favorites(&self) -> Result<Vec<Project>> {
        let projects = self.database.list_projects()?;
        Ok(projects.into_iter().filter(|p| p.is_favorite).collect())
    }

    /// Add a new project
    pub fn add_project(&self, project: Project) -> Result<()> {
        self.database.insert_project(&project)
    }

    /// Get a project by ID
    pub fn get_project(&self, id: &str) -> Result<Option<Project>> {
        self.database.get_project(id)
    }

    /// Update a project
    pub fn update_project(&self, project: &Project) -> Result<()> {
        self.database.update_project(project)
    }

    /// Delete a project
    pub fn delete_project(&self, id: &str) -> Result<()> {
        self.database.delete_project(id)
    }

    /// Update last accessed time
    pub fn touch_project(&self, id: &str) -> Result<()> {
        if let Some(mut project) = self.get_project(id)? {
            project.last_accessed = Utc::now();
            self.update_project(&project)?;
        }
        Ok(())
    }

    /// Toggle favorite status
    pub fn toggle_favorite(&self, id: &str) -> Result<bool> {
        if let Some(mut project) = self.get_project(id)? {
            project.is_favorite = !project.is_favorite;
            let is_favorite = project.is_favorite;
            self.update_project(&project)?;
            Ok(is_favorite)
        } else {
            Ok(false)
        }
    }

    /// Add a tag to a project
    pub fn add_tag(&self, id: &str, tag: &str) -> Result<()> {
        if let Some(mut project) = self.get_project(id)? {
            if !project.tags.contains(&tag.to_string()) {
                project.tags.push(tag.to_string());
                self.update_project(&project)?;
            }
        }
        Ok(())
    }

    /// Remove a tag from a project
    pub fn remove_tag(&self, id: &str, tag: &str) -> Result<()> {
        if let Some(mut project) = self.get_project(id)? {
            project.tags.retain(|t| t != tag);
            self.update_project(&project)?;
        }
        Ok(())
    }

    /// Set theme override for a project
    pub fn set_theme_override(&self, id: &str, theme: Option<ThemeVariant>) -> Result<()> {
        if let Some(mut project) = self.get_project(id)? {
            project.theme_override = theme;
            self.update_project(&project)?;
        }
        Ok(())
    }

    /// Get theme override for a project
    pub fn get_theme_override(&self, id: &str) -> Result<Option<ThemeVariant>> {
        if let Some(project) = self.get_project(id)? {
            Ok(project.theme_override)
        } else {
            Ok(None)
        }
    }
}
