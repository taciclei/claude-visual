//! MCP Tool utilities and helpers
//!
//! Provides tool discovery, categorization, and invocation helpers.

use super::protocol::{McpTool, McpError};
use serde_json::Value;
use std::collections::HashMap;

/// Tool category for organization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolCategory {
    /// File system operations
    FileSystem,
    /// Code analysis and manipulation
    Code,
    /// Web and network operations
    Web,
    /// Database operations
    Database,
    /// Search and query
    Search,
    /// Git and version control
    Git,
    /// Shell and command execution
    Shell,
    /// AI and LLM operations
    AI,
    /// Custom or uncategorized
    Custom(String),
}

impl ToolCategory {
    /// Categorize a tool based on its name
    pub fn from_tool_name(name: &str) -> Self {
        let name_lower = name.to_lowercase();

        if name_lower.contains("file") || name_lower.contains("read") || name_lower.contains("write")
            || name_lower.contains("directory") || name_lower.contains("path") {
            ToolCategory::FileSystem
        } else if name_lower.contains("code") || name_lower.contains("syntax") || name_lower.contains("parse")
            || name_lower.contains("ast") || name_lower.contains("lint") {
            ToolCategory::Code
        } else if name_lower.contains("http") || name_lower.contains("fetch") || name_lower.contains("url")
            || name_lower.contains("web") || name_lower.contains("api") {
            ToolCategory::Web
        } else if name_lower.contains("sql") || name_lower.contains("database") || name_lower.contains("query")
            || name_lower.contains("db") {
            ToolCategory::Database
        } else if name_lower.contains("search") || name_lower.contains("find") || name_lower.contains("grep")
            || name_lower.contains("glob") {
            ToolCategory::Search
        } else if name_lower.contains("git") || name_lower.contains("commit") || name_lower.contains("branch")
            || name_lower.contains("repo") {
            ToolCategory::Git
        } else if name_lower.contains("shell") || name_lower.contains("bash") || name_lower.contains("exec")
            || name_lower.contains("command") || name_lower.contains("terminal") {
            ToolCategory::Shell
        } else if name_lower.contains("ai") || name_lower.contains("llm") || name_lower.contains("model")
            || name_lower.contains("prompt") || name_lower.contains("chat") {
            ToolCategory::AI
        } else {
            ToolCategory::Custom("Other".to_string())
        }
    }

    /// Get display name for category
    pub fn display_name(&self) -> &str {
        match self {
            ToolCategory::FileSystem => "File System",
            ToolCategory::Code => "Code",
            ToolCategory::Web => "Web & API",
            ToolCategory::Database => "Database",
            ToolCategory::Search => "Search",
            ToolCategory::Git => "Git",
            ToolCategory::Shell => "Shell",
            ToolCategory::AI => "AI",
            ToolCategory::Custom(name) => name,
        }
    }

    /// Get icon for category (emoji)
    pub fn icon(&self) -> &str {
        match self {
            ToolCategory::FileSystem => "📁",
            ToolCategory::Code => "💻",
            ToolCategory::Web => "🌐",
            ToolCategory::Database => "🗄️",
            ToolCategory::Search => "🔍",
            ToolCategory::Git => "📦",
            ToolCategory::Shell => "⌨️",
            ToolCategory::AI => "🤖",
            ToolCategory::Custom(_) => "🔧",
        }
    }
}

/// Tool with additional metadata
#[derive(Debug, Clone)]
pub struct EnrichedTool {
    /// The base MCP tool
    pub tool: McpTool,
    /// Server that provides this tool
    pub server_name: String,
    /// Inferred category
    pub category: ToolCategory,
    /// Usage count (for sorting by popularity)
    pub usage_count: u32,
    /// Whether this tool is favorited
    pub is_favorite: bool,
}

impl EnrichedTool {
    /// Create an enriched tool from an MCP tool
    pub fn new(tool: McpTool, server_name: impl Into<String>) -> Self {
        let category = ToolCategory::from_tool_name(&tool.name);
        Self {
            tool,
            server_name: server_name.into(),
            category,
            usage_count: 0,
            is_favorite: false,
        }
    }

    /// Get the full tool name (server__tool)
    pub fn full_name(&self) -> String {
        format!("{}:{}", self.server_name, self.tool.name)
    }
}

/// Tool registry for managing and querying tools
pub struct ToolRegistry {
    /// All registered tools
    tools: HashMap<String, EnrichedTool>,
    /// Tools grouped by category
    by_category: HashMap<ToolCategory, Vec<String>>,
    /// Tools grouped by server
    by_server: HashMap<String, Vec<String>>,
    /// Favorite tools
    favorites: Vec<String>,
    /// Recently used tools (most recent first)
    recent: Vec<String>,
    /// Maximum recent tools to track
    max_recent: usize,
}

impl ToolRegistry {
    /// Create a new tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            by_category: HashMap::new(),
            by_server: HashMap::new(),
            favorites: Vec::new(),
            recent: Vec::new(),
            max_recent: 10,
        }
    }

    /// Register a tool from an MCP server
    pub fn register(&mut self, tool: McpTool, server_name: &str) {
        let enriched = EnrichedTool::new(tool, server_name);
        let full_name = enriched.full_name();
        let category = enriched.category.clone();

        // Add to main registry
        self.tools.insert(full_name.clone(), enriched);

        // Add to category index
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(full_name.clone());

        // Add to server index
        self.by_server
            .entry(server_name.to_string())
            .or_insert_with(Vec::new)
            .push(full_name);
    }

    /// Unregister all tools from a server
    pub fn unregister_server(&mut self, server_name: &str) {
        if let Some(tool_names) = self.by_server.remove(server_name) {
            for name in tool_names {
                if let Some(tool) = self.tools.remove(&name) {
                    // Remove from category index
                    if let Some(category_tools) = self.by_category.get_mut(&tool.category) {
                        category_tools.retain(|n| n != &name);
                    }
                }
                // Remove from favorites
                self.favorites.retain(|n| n != &name);
                // Remove from recent
                self.recent.retain(|n| n != &name);
            }
        }
    }

    /// Get a tool by full name
    pub fn get(&self, full_name: &str) -> Option<&EnrichedTool> {
        self.tools.get(full_name)
    }

    /// Get all tools
    pub fn all(&self) -> impl Iterator<Item = &EnrichedTool> {
        self.tools.values()
    }

    /// Get tools by category
    pub fn by_category(&self, category: &ToolCategory) -> Vec<&EnrichedTool> {
        self.by_category
            .get(category)
            .map(|names| {
                names.iter()
                    .filter_map(|name| self.tools.get(name))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get tools by server
    pub fn by_server(&self, server_name: &str) -> Vec<&EnrichedTool> {
        self.by_server
            .get(server_name)
            .map(|names| {
                names.iter()
                    .filter_map(|name| self.tools.get(name))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all categories with tool counts
    pub fn categories(&self) -> Vec<(&ToolCategory, usize)> {
        self.by_category
            .iter()
            .map(|(cat, tools)| (cat, tools.len()))
            .collect()
    }

    /// Search tools by name or description
    pub fn search(&self, query: &str) -> Vec<&EnrichedTool> {
        let query_lower = query.to_lowercase();
        self.tools.values()
            .filter(|tool| {
                tool.tool.name.to_lowercase().contains(&query_lower) ||
                tool.tool.description.as_ref()
                    .map(|d| d.to_lowercase().contains(&query_lower))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Toggle tool favorite status
    pub fn toggle_favorite(&mut self, full_name: &str) {
        if let Some(tool) = self.tools.get_mut(full_name) {
            tool.is_favorite = !tool.is_favorite;
            if tool.is_favorite {
                if !self.favorites.contains(&full_name.to_string()) {
                    self.favorites.push(full_name.to_string());
                }
            } else {
                self.favorites.retain(|n| n != full_name);
            }
        }
    }

    /// Get favorite tools
    pub fn favorites(&self) -> Vec<&EnrichedTool> {
        self.favorites.iter()
            .filter_map(|name| self.tools.get(name))
            .collect()
    }

    /// Record tool usage
    pub fn record_usage(&mut self, full_name: &str) {
        if let Some(tool) = self.tools.get_mut(full_name) {
            tool.usage_count += 1;
        }

        // Update recent list
        self.recent.retain(|n| n != full_name);
        self.recent.insert(0, full_name.to_string());
        if self.recent.len() > self.max_recent {
            self.recent.truncate(self.max_recent);
        }
    }

    /// Get recent tools
    pub fn recent(&self) -> Vec<&EnrichedTool> {
        self.recent.iter()
            .filter_map(|name| self.tools.get(name))
            .collect()
    }

    /// Get tool count
    pub fn count(&self) -> usize {
        self.tools.len()
    }

    /// Clear all tools
    pub fn clear(&mut self) {
        self.tools.clear();
        self.by_category.clear();
        self.by_server.clear();
        // Keep favorites and recent for persistence
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Build tool call arguments from a template
pub fn build_arguments(
    tool: &McpTool,
    values: HashMap<String, Value>,
) -> Result<HashMap<String, Value>, McpError> {
    // Validate required parameters
    if let Some(ref required) = tool.input_schema.required {
        for param_name in required {
            if !values.contains_key(param_name) {
                return Err(McpError::Protocol(format!(
                    "Missing required parameter: {}", param_name
                )));
            }
        }
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_categorization() {
        assert_eq!(ToolCategory::from_tool_name("read_file"), ToolCategory::FileSystem);
        assert_eq!(ToolCategory::from_tool_name("git_commit"), ToolCategory::Git);
        assert_eq!(ToolCategory::from_tool_name("execute_bash"), ToolCategory::Shell);
        assert_eq!(ToolCategory::from_tool_name("http_fetch"), ToolCategory::Web);
        assert_eq!(ToolCategory::from_tool_name("search_code"), ToolCategory::Search);
    }

    #[test]
    fn test_tool_registry() {
        let mut registry = ToolRegistry::new();

        let tool = McpTool {
            name: "read_file".to_string(),
            description: Some("Read a file".to_string()),
            input_schema: None,
        };

        registry.register(tool, "filesystem");

        assert_eq!(registry.count(), 1);
        assert!(registry.get("filesystem:read_file").is_some());
    }
}
