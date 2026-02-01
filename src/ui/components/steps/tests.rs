use super::*;

#[test]
fn test_steps_creation() {
    let steps = Steps::new("steps-1")
        .add_step(Step::new("Step 1"))
        .add_step(Step::new("Step 2"))
        .add_step(Step::new("Step 3"))
        .current(1);
    assert_eq!(steps.steps.len(), 3);
    assert_eq!(steps.current, 1);
}

#[test]
fn test_step_status() {
    let step = Step::new("Test")
        .description("Description")
        .status(StepStatus::Completed);
    assert_eq!(step.status, StepStatus::Completed);
    assert!(step.description.is_some());
}

#[test]
fn test_progress_stepper() {
    let stepper = ProgressStepper::new("ps-1", 5)
        .current_step(2)
        .labels(vec!["A", "B", "C", "D", "E"]);
    assert_eq!(stepper.total_steps, 5);
    assert_eq!(stepper.current_step, 2);
    assert_eq!(stepper.labels.len(), 5);
}

#[test]
fn test_wizard_nav() {
    let nav = WizardNav::new("wn-1", 2, 5)
        .can_go_back(true)
        .can_go_next(true);
    assert!(nav.can_go_back);
    assert!(nav.can_go_next);
    assert_eq!(nav.current_step, 2);
}

#[test]
fn test_numbered_steps() {
    let ns = NumberedSteps::new("ns-1")
        .steps(vec!["First", "Second", "Third"])
        .mark_completed(0);
    assert_eq!(ns.steps.len(), 3);
    assert!(ns.completed[0]);
    assert!(!ns.completed[1]);
}

#[test]
fn test_steps_size() {
    assert!(StepsSize::Sm.indicator_size() < StepsSize::Md.indicator_size());
    assert!(StepsSize::Md.indicator_size() < StepsSize::Lg.indicator_size());
}
