//! Debug Panel Types
//!
//! Type definitions for debug panel events and states.

use crate::debug::DebugState;

/// Events emitted by debug panel
#[derive(Debug, Clone)]
pub enum DebugPanelEvent {
    /// Start debugging
    Start,
    /// Stop debugging
    Stop,
    /// Restart debugging
    Restart,
    /// Continue execution
    Continue,
    /// Step over
    StepOver,
    /// Step into
    StepInto,
    /// Step out
    StepOut,
    /// Pause execution
    Pause,
    /// Toggle breakpoint at current line
    ToggleBreakpoint,
    /// Open debug configuration
    OpenConfiguration,
    /// Ask AI for debugging help
    AskAI(DebugPromptType),
}

/// Types of AI-assisted debugging prompts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugPromptType {
    /// Analyze current error
    AnalyzeError,
    /// Explain current state
    ExplainState,
    /// Suggest breakpoints
    SuggestBreakpoints,
    /// Analyze stack trace
    AnalyzeStackTrace,
    /// Suggest fix
    SuggestFix,
    /// Explain variables
    ExplainVariables,
    /// Performance analysis
    PerformanceAnalysis,
    /// Memory analysis
    MemoryAnalysis,
}

impl DebugPromptType {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            DebugPromptType::AnalyzeError => "Analyze Error",
            DebugPromptType::ExplainState => "Explain State",
            DebugPromptType::SuggestBreakpoints => "Suggest Breakpoints",
            DebugPromptType::AnalyzeStackTrace => "Analyze Stack Trace",
            DebugPromptType::SuggestFix => "Suggest Fix",
            DebugPromptType::ExplainVariables => "Explain Variables",
            DebugPromptType::PerformanceAnalysis => "Performance Analysis",
            DebugPromptType::MemoryAnalysis => "Memory Analysis",
        }
    }

    /// Get short icon/emoji
    pub fn icon(&self) -> &'static str {
        match self {
            DebugPromptType::AnalyzeError => "!",
            DebugPromptType::ExplainState => "?",
            DebugPromptType::SuggestBreakpoints => "o",
            DebugPromptType::AnalyzeStackTrace => "#",
            DebugPromptType::SuggestFix => "*",
            DebugPromptType::ExplainVariables => "x",
            DebugPromptType::PerformanceAnalysis => "~",
            DebugPromptType::MemoryAnalysis => "m",
        }
    }

    /// Get all prompt types
    pub fn all() -> Vec<DebugPromptType> {
        vec![
            DebugPromptType::AnalyzeError,
            DebugPromptType::ExplainState,
            DebugPromptType::SuggestBreakpoints,
            DebugPromptType::AnalyzeStackTrace,
            DebugPromptType::SuggestFix,
            DebugPromptType::ExplainVariables,
            DebugPromptType::PerformanceAnalysis,
            DebugPromptType::MemoryAnalysis,
        ]
    }

    /// Generate prompt with context
    pub fn generate_prompt(&self, context: &DebugContext) -> String {
        match self {
            DebugPromptType::AnalyzeError => {
                let mut prompt = "Analyze this debugging error and help me understand what went wrong:\n\n".to_string();
                if let Some(ref err) = context.last_error {
                    prompt.push_str(&format!("Error: {}\n\n", err));
                }
                if let Some((ref file, line)) = context.current_location {
                    prompt.push_str(&format!("Location: {}:{}\n\n", file, line));
                }
                if !context.console_output.is_empty() {
                    prompt.push_str("Recent console output:\n```\n");
                    for line in context.console_output.iter().rev().take(20) {
                        prompt.push_str(line);
                        prompt.push('\n');
                    }
                    prompt.push_str("```\n\n");
                }
                prompt.push_str("What is the root cause and how can I fix it?");
                prompt
            }
            DebugPromptType::ExplainState => {
                let mut prompt = "Explain the current debugging state:\n\n".to_string();
                prompt.push_str(&format!("State: {:?}\n", context.state));
                if let Some(ref thread) = context.current_thread {
                    prompt.push_str(&format!("Thread: {}\n", thread));
                }
                if let Some((ref file, line)) = context.current_location {
                    prompt.push_str(&format!("Location: {}:{}\n", file, line));
                }
                prompt.push_str("\nWhat is happening at this point in the execution?");
                prompt
            }
            DebugPromptType::SuggestBreakpoints => {
                let mut prompt = "Based on the current debugging session, suggest strategic breakpoints:\n\n".to_string();
                if let Some((ref file, _)) = context.current_location {
                    prompt.push_str(&format!("Current file: {}\n", file));
                }
                if !context.console_output.is_empty() {
                    prompt.push_str("\nRecent output:\n```\n");
                    for line in context.console_output.iter().rev().take(10) {
                        prompt.push_str(line);
                        prompt.push('\n');
                    }
                    prompt.push_str("```\n");
                }
                prompt.push_str("\nWhere should I set breakpoints to effectively debug this issue?");
                prompt
            }
            DebugPromptType::AnalyzeStackTrace => {
                let mut prompt = "Analyze this stack trace and help me understand the execution flow:\n\n".to_string();
                if !context.stack_frames.is_empty() {
                    prompt.push_str("Stack frames:\n");
                    for (i, frame) in context.stack_frames.iter().enumerate() {
                        prompt.push_str(&format!("  #{}: {}\n", i, frame));
                    }
                    prompt.push('\n');
                }
                if !context.console_output.is_empty() {
                    prompt.push_str("Console output:\n```\n");
                    for line in context.console_output.iter().rev().take(15) {
                        prompt.push_str(line);
                        prompt.push('\n');
                    }
                    prompt.push_str("```\n");
                }
                prompt.push_str("\nExplain the call sequence and identify any issues.");
                prompt
            }
            DebugPromptType::SuggestFix => {
                let mut prompt = "Based on the debugging context, suggest a fix:\n\n".to_string();
                if let Some(ref err) = context.last_error {
                    prompt.push_str(&format!("Error: {}\n\n", err));
                }
                if let Some((ref file, line)) = context.current_location {
                    prompt.push_str(&format!("Location: {}:{}\n\n", file, line));
                }
                if !context.console_output.is_empty() {
                    prompt.push_str("Debug output:\n```\n");
                    for line in context.console_output.iter().rev().take(15) {
                        prompt.push_str(line);
                        prompt.push('\n');
                    }
                    prompt.push_str("```\n\n");
                }
                prompt.push_str("Provide a concrete fix with code changes.");
                prompt
            }
            DebugPromptType::ExplainVariables => {
                let mut prompt = "Explain these variable values in the current debugging context:\n\n".to_string();
                if !context.variables.is_empty() {
                    for (name, value) in &context.variables {
                        prompt.push_str(&format!("{} = {}\n", name, value));
                    }
                    prompt.push('\n');
                }
                if let Some((ref file, line)) = context.current_location {
                    prompt.push_str(&format!("At: {}:{}\n\n", file, line));
                }
                prompt.push_str("Are these values expected? What might be wrong?");
                prompt
            }
            DebugPromptType::PerformanceAnalysis => {
                let mut prompt = "Analyze the performance of this code based on the debug session:\n\n".to_string();
                if let Some((ref file, line)) = context.current_location {
                    prompt.push_str(&format!("Location: {}:{}\n\n", file, line));
                }
                if !context.console_output.is_empty() {
                    prompt.push_str("Output:\n```\n");
                    for line in context.console_output.iter().rev().take(20) {
                        prompt.push_str(line);
                        prompt.push('\n');
                    }
                    prompt.push_str("```\n\n");
                }
                prompt.push_str("Identify performance bottlenecks and suggest optimizations.");
                prompt
            }
            DebugPromptType::MemoryAnalysis => {
                let mut prompt = "Analyze memory usage based on this debug session:\n\n".to_string();
                if !context.variables.is_empty() {
                    prompt.push_str("Variables:\n");
                    for (name, value) in &context.variables {
                        prompt.push_str(&format!("  {} = {}\n", name, value));
                    }
                    prompt.push('\n');
                }
                if !context.console_output.is_empty() {
                    prompt.push_str("Output:\n```\n");
                    for line in context.console_output.iter().rev().take(15) {
                        prompt.push_str(line);
                        prompt.push('\n');
                    }
                    prompt.push_str("```\n\n");
                }
                prompt.push_str("Identify potential memory leaks or excessive memory usage.");
                prompt
            }
        }
    }
}

/// Debug context for AI prompts
#[derive(Debug, Clone, Default)]
pub struct DebugContext {
    /// Current debug state
    pub state: DebugState,
    /// Current file and line
    pub current_location: Option<(String, i64)>,
    /// Current thread
    pub current_thread: Option<String>,
    /// Last error message
    pub last_error: Option<String>,
    /// Recent console output
    pub console_output: Vec<String>,
    /// Stack frames
    pub stack_frames: Vec<String>,
    /// Variable names and values
    pub variables: Vec<(String, String)>,
}

/// Output line in debug console
#[derive(Debug, Clone)]
pub(super) struct OutputLine {
    pub category: String,
    pub text: String,
    pub timestamp: u64,
}

/// Debug panel tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugTab {
    Console,
    Variables,
    CallStack,
    Breakpoints,
    Watch,
}

impl DebugTab {
    pub fn label(&self) -> &'static str {
        match self {
            DebugTab::Console => "Console",
            DebugTab::Variables => "Variables",
            DebugTab::CallStack => "Call Stack",
            DebugTab::Breakpoints => "Breakpoints",
            DebugTab::Watch => "Watch",
        }
    }
}
