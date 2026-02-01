//! DAP capability types

use serde::{Deserialize, Serialize};

/// Capabilities returned by initialize
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// Supports configuration done request
    #[serde(default)]
    pub supports_configuration_done_request: bool,
    /// Supports function breakpoints
    #[serde(default)]
    pub supports_function_breakpoints: bool,
    /// Supports conditional breakpoints
    #[serde(default)]
    pub supports_conditional_breakpoints: bool,
    /// Supports hit conditional breakpoints
    #[serde(default)]
    pub supports_hit_conditional_breakpoints: bool,
    /// Supports evaluate for hovers
    #[serde(default)]
    pub supports_evaluate_for_hovers: bool,
    /// Exception breakpoint filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_breakpoint_filters: Option<Vec<ExceptionBreakpointsFilter>>,
    /// Supports step back
    #[serde(default)]
    pub supports_step_back: bool,
    /// Supports set variable
    #[serde(default)]
    pub supports_set_variable: bool,
    /// Supports restart frame
    #[serde(default)]
    pub supports_restart_frame: bool,
    /// Supports goto targets
    #[serde(default)]
    pub supports_goto_targets_request: bool,
    /// Supports step in targets
    #[serde(default)]
    pub supports_step_in_targets_request: bool,
    /// Supports completions
    #[serde(default)]
    pub supports_completions_request: bool,
    /// Supports modules
    #[serde(default)]
    pub supports_modules_request: bool,
    /// Supports restart
    #[serde(default)]
    pub supports_restart_request: bool,
    /// Supports exception options
    #[serde(default)]
    pub supports_exception_options: bool,
    /// Supports value formatting
    #[serde(default)]
    pub supports_value_formatting_options: bool,
    /// Supports exception info
    #[serde(default)]
    pub supports_exception_info_request: bool,
    /// Supports terminate debuggee
    #[serde(default)]
    pub support_terminate_debuggee: bool,
    /// Supports suspend debuggee
    #[serde(default)]
    pub support_suspend_debuggee: bool,
    /// Supports delayed stack trace loading
    #[serde(default)]
    pub supports_delayed_stack_trace_loading: bool,
    /// Supports loaded sources
    #[serde(default)]
    pub supports_loaded_sources_request: bool,
    /// Supports log points
    #[serde(default)]
    pub supports_log_points: bool,
    /// Supports terminate threads
    #[serde(default)]
    pub supports_terminate_threads_request: bool,
    /// Supports set expression
    #[serde(default)]
    pub supports_set_expression: bool,
    /// Supports terminate
    #[serde(default)]
    pub supports_terminate_request: bool,
    /// Supports data breakpoints
    #[serde(default)]
    pub supports_data_breakpoints: bool,
    /// Supports read memory
    #[serde(default)]
    pub supports_read_memory_request: bool,
    /// Supports write memory
    #[serde(default)]
    pub supports_write_memory_request: bool,
    /// Supports disassemble
    #[serde(default)]
    pub supports_disassemble_request: bool,
    /// Supports cancel
    #[serde(default)]
    pub supports_cancel_request: bool,
    /// Supports breakpoint locations
    #[serde(default)]
    pub supports_breakpoint_locations_request: bool,
    /// Supports clipboard context
    #[serde(default)]
    pub supports_clipboard_context: bool,
    /// Supports stepping granularity
    #[serde(default)]
    pub supports_stepping_granularity: bool,
    /// Supports instruction breakpoints
    #[serde(default)]
    pub supports_instruction_breakpoints: bool,
    /// Supports exception filter options
    #[serde(default)]
    pub supports_exception_filter_options: bool,
    /// Supports single thread execution
    #[serde(default)]
    pub supports_single_thread_execution_requests: bool,
}

/// Exception breakpoints filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionBreakpointsFilter {
    /// Filter name
    pub filter: String,
    /// Label
    pub label: String,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Default value
    #[serde(default)]
    pub default: bool,
    /// Supports condition
    #[serde(default)]
    pub supports_condition: bool,
    /// Condition description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_description: Option<String>,
}
