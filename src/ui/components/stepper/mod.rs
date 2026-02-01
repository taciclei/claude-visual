//! Stepper component for multi-step processes

mod types;
mod component;
mod render;
mod horizontal;
mod vertical;
mod progress_steps;
mod breadcrumb_steps;

// Re-export public items
pub use types::*;
pub use component::Stepper;
pub use progress_steps::ProgressSteps;
pub use breadcrumb_steps::BreadcrumbSteps;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let step = Step::new("Account Info")
            .description("Enter your details")
            .completed();

        assert_eq!(step.label, "Account Info");
        assert_eq!(step.status, StepStatus::Completed);
    }

    #[test]
    fn test_progress_steps() {
        let steps = ProgressSteps::new(vec!["Step 1", "Step 2", "Step 3"], 1);

        assert_eq!(steps.steps.len(), 3);
        assert_eq!(steps.current, 1);
    }

    #[test]
    fn test_breadcrumb_steps() {
        let steps = BreadcrumbSteps::new(vec!["Cart", "Shipping", "Payment"], 2);

        assert_eq!(steps.steps.len(), 3);
        assert_eq!(steps.current, 2);
    }
}
