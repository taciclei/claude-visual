//! Tests for planner module

use super::*;

#[test]
fn test_plan_creation() {
    let mut plan = Plan::new("Test Plan", "Description");
    plan.add_step(PlanStep {
        step_number: 1,
        title: "First step".to_string(),
        description: "Do something".to_string(),
        tools: vec!["read_file".to_string()],
        estimated_tokens: None,
        depends_on: vec![],
        risk_level: 2,
        requires_approval: false,
    });

    assert_eq!(plan.steps.len(), 1);
}

#[test]
fn test_plan_to_task_tree() {
    let mut plan = Plan::new("Test", "Test plan");
    plan.add_step(PlanStep {
        step_number: 1,
        title: "Step 1".to_string(),
        description: "First step".to_string(),
        tools: vec![],
        estimated_tokens: None,
        depends_on: vec![],
        risk_level: 2,
        requires_approval: false,
    });

    let tree = plan.to_task_tree();
    assert_eq!(tree.roots().len(), 1);
}

#[test]
fn test_parse_plan() {
    let planner = AgentPlanner::new();
    let json = r#"
    {
        "title": "Test Plan",
        "description": "A test",
        "steps": [
            {
                "step_number": 1,
                "title": "Read file",
                "description": "Read the file",
                "tools": ["read_file"],
                "depends_on": [],
                "risk_level": 1,
                "requires_approval": false
            }
        ]
    }
    "#;

    let plan = planner.parse_plan(json).unwrap();
    assert_eq!(plan.title, "Test Plan");
    assert_eq!(plan.steps.len(), 1);
}
