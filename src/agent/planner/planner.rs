//! Agent planner implementation

use super::errors::{PlanError, PlanValidationError};
use super::types::{Plan, PlanStep};

/// Agent planner for generating execution plans
pub struct AgentPlanner {
    /// Planning prompts/templates
    planning_prompt: String,
    /// Tools available for planning
    available_tools: Vec<String>,
    /// Max steps in a plan
    max_steps: usize,
    /// Auto-approve low-risk steps
    auto_approve_low_risk: bool,
}

impl Default for AgentPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentPlanner {
    /// Create a new planner
    pub fn new() -> Self {
        Self {
            planning_prompt: Self::default_planning_prompt(),
            available_tools: vec![
                "read_file".to_string(),
                "write_file".to_string(),
                "edit_file".to_string(),
                "run_command".to_string(),
                "search_files".to_string(),
                "list_directory".to_string(),
            ],
            max_steps: 20,
            auto_approve_low_risk: true,
        }
    }

    /// Default planning prompt template
    fn default_planning_prompt() -> String {
        r#"You are an AI agent planner. Given a user's goal, create a detailed step-by-step plan.

For each step, specify:
1. A clear title
2. Detailed description of what to do
3. Which tools will be used
4. Any dependencies on previous steps
5. Risk level (0-10, where 10 is highest risk)
6. Whether human approval is needed

Format your response as JSON:
{
  "title": "Plan title",
  "description": "Overall plan description",
  "steps": [
    {
      "step_number": 1,
      "title": "Step title",
      "description": "What to do",
      "tools": ["tool1", "tool2"],
      "depends_on": [],
      "risk_level": 3,
      "requires_approval": false
    }
  ]
}

Available tools: {tools}
User's goal: {goal}"#.to_string()
    }

    /// Set available tools
    pub fn with_tools(mut self, tools: Vec<String>) -> Self {
        self.available_tools = tools;
        self
    }

    /// Set max steps
    pub fn with_max_steps(mut self, max: usize) -> Self {
        self.max_steps = max;
        self
    }

    /// Generate planning prompt for a goal
    pub fn generate_prompt(&self, goal: &str) -> String {
        self.planning_prompt
            .replace("{tools}", &self.available_tools.join(", "))
            .replace("{goal}", goal)
    }

    /// Parse a plan from AI response
    pub fn parse_plan(&self, response: &str) -> Result<Plan, PlanError> {
        // Try to extract JSON from response
        let json_str = Self::extract_json(response)?;

        // Parse JSON
        let parsed: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| PlanError::ParseError(e.to_string()))?;

        // Extract plan fields
        let title = parsed["title"]
            .as_str()
            .ok_or(PlanError::MissingField("title".to_string()))?;
        let description = parsed["description"]
            .as_str()
            .unwrap_or("");

        let mut plan = Plan::new(title, description);

        // Extract steps
        let steps = parsed["steps"]
            .as_array()
            .ok_or(PlanError::MissingField("steps".to_string()))?;

        for (i, step_json) in steps.iter().enumerate() {
            if i >= self.max_steps {
                break;
            }

            let step = PlanStep {
                step_number: step_json["step_number"]
                    .as_u64()
                    .unwrap_or((i + 1) as u64) as usize,
                title: step_json["title"]
                    .as_str()
                    .unwrap_or(&format!("Step {}", i + 1))
                    .to_string(),
                description: step_json["description"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                tools: step_json["tools"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default(),
                estimated_tokens: step_json["estimated_tokens"]
                    .as_u64()
                    .map(|n| n as usize),
                depends_on: step_json["depends_on"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_u64().map(|n| n as usize))
                            .collect()
                    })
                    .unwrap_or_default(),
                risk_level: step_json["risk_level"]
                    .as_u64()
                    .unwrap_or(5) as u8,
                requires_approval: step_json["requires_approval"]
                    .as_bool()
                    .unwrap_or_else(|| {
                        // Auto-determine based on risk
                        !self.auto_approve_low_risk || step_json["risk_level"].as_u64().unwrap_or(5) > 3
                    }),
            };

            plan.add_step(step);
        }

        if plan.steps.is_empty() {
            return Err(PlanError::EmptyPlan);
        }

        Ok(plan)
    }

    /// Extract JSON from a response that might have extra text
    fn extract_json(response: &str) -> Result<String, PlanError> {
        // Try to find JSON object
        let start = response.find('{');
        let end = response.rfind('}');

        match (start, end) {
            (Some(s), Some(e)) if s < e => Ok(response[s..=e].to_string()),
            _ => Err(PlanError::NoJsonFound),
        }
    }

    /// Validate a plan
    pub fn validate_plan(&self, plan: &Plan) -> Result<(), Vec<PlanValidationError>> {
        let mut errors = Vec::new();

        // Check for empty steps
        if plan.steps.is_empty() {
            errors.push(PlanValidationError::EmptyPlan);
        }

        // Check for circular dependencies
        for step in &plan.steps {
            if step.depends_on.contains(&step.step_number) {
                errors.push(PlanValidationError::CircularDependency(step.step_number));
            }
        }

        // Check for invalid dependencies
        let step_numbers: Vec<usize> = plan.steps.iter().map(|s| s.step_number).collect();
        for step in &plan.steps {
            for dep in &step.depends_on {
                if !step_numbers.contains(dep) {
                    errors.push(PlanValidationError::InvalidDependency(step.step_number, *dep));
                }
            }
        }

        // Check for unknown tools
        for step in &plan.steps {
            for tool in &step.tools {
                if !self.available_tools.contains(tool) {
                    errors.push(PlanValidationError::UnknownTool(tool.clone()));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
