//! Lazy Initialization
//!
//! Deferred initialization for non-critical components to improve startup time.

use std::sync::OnceLock;
use std::time::Instant;

use tracing::{debug, info};

/// Tracks startup timing for performance monitoring
pub struct StartupMetrics {
    /// Application start time
    start_time: Instant,
    /// Time when window was first shown
    window_shown: Option<Instant>,
    /// Time when UI was fully interactive
    interactive: Option<Instant>,
    /// Component initialization times
    components: Vec<(String, Instant, Instant)>,
}

impl StartupMetrics {
    /// Create new startup metrics
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            window_shown: None,
            interactive: None,
            components: Vec::new(),
        }
    }

    /// Mark window as shown
    pub fn mark_window_shown(&mut self) {
        self.window_shown = Some(Instant::now());
        info!(
            "Window shown in {:?}",
            self.window_shown.unwrap().duration_since(self.start_time)
        );
    }

    /// Mark as interactive
    pub fn mark_interactive(&mut self) {
        self.interactive = Some(Instant::now());
        info!(
            "Interactive in {:?}",
            self.interactive.unwrap().duration_since(self.start_time)
        );
    }

    /// Start tracking a component initialization
    pub fn start_component(&mut self, name: &str) -> ComponentTimer {
        ComponentTimer {
            name: name.to_string(),
            start: Instant::now(),
        }
    }

    /// Record component initialization time
    pub fn record_component(&mut self, timer: ComponentTimer) {
        let end = Instant::now();
        debug!(
            "Component '{}' initialized in {:?}",
            timer.name,
            end.duration_since(timer.start)
        );
        self.components.push((timer.name, timer.start, end));
    }

    /// Get time to window shown
    pub fn time_to_window(&self) -> Option<std::time::Duration> {
        self.window_shown
            .map(|t| t.duration_since(self.start_time))
    }

    /// Get time to interactive
    pub fn time_to_interactive(&self) -> Option<std::time::Duration> {
        self.interactive.map(|t| t.duration_since(self.start_time))
    }

    /// Log all metrics
    pub fn log_summary(&self) {
        info!("=== Startup Metrics ===");
        if let Some(ttw) = self.time_to_window() {
            info!("Time to window: {:?}", ttw);
        }
        if let Some(tti) = self.time_to_interactive() {
            info!("Time to interactive: {:?}", tti);
        }

        if !self.components.is_empty() {
            info!("Component initialization times:");
            for (name, start, end) in &self.components {
                info!("  {}: {:?}", name, end.duration_since(*start));
            }
        }
    }
}

impl Default for StartupMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer for tracking component initialization
pub struct ComponentTimer {
    name: String,
    start: Instant,
}

/// Global startup metrics
static STARTUP_METRICS: OnceLock<parking_lot::Mutex<StartupMetrics>> = OnceLock::new();

/// Get global startup metrics
pub fn startup_metrics() -> &'static parking_lot::Mutex<StartupMetrics> {
    STARTUP_METRICS.get_or_init(|| parking_lot::Mutex::new(StartupMetrics::new()))
}

/// Lazy-initialized value that loads on first access
pub struct LazyInit<T> {
    value: OnceLock<T>,
    init_fn: fn() -> T,
}

impl<T> LazyInit<T> {
    /// Create a new lazy value
    pub const fn new(init_fn: fn() -> T) -> Self {
        Self {
            value: OnceLock::new(),
            init_fn,
        }
    }

    /// Get the value, initializing if necessary
    pub fn get(&self) -> &T {
        self.value.get_or_init(self.init_fn)
    }

    /// Check if already initialized
    pub fn is_initialized(&self) -> bool {
        self.value.get().is_some()
    }
}

/// Deferred task that runs after startup
pub struct DeferredTask {
    name: String,
    priority: u8,
    task: Box<dyn FnOnce() + Send>,
}

impl DeferredTask {
    /// Create a new deferred task
    pub fn new<F>(name: impl Into<String>, priority: u8, task: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            name: name.into(),
            priority,
            task: Box::new(task),
        }
    }

    /// Execute the task
    pub fn execute(self) {
        debug!("Executing deferred task: {}", self.name);
        (self.task)();
    }
}

/// Queue of deferred tasks
pub struct DeferredTaskQueue {
    tasks: Vec<DeferredTask>,
}

impl DeferredTaskQueue {
    /// Create a new task queue
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Add a task to the queue
    pub fn push(&mut self, task: DeferredTask) {
        self.tasks.push(task);
    }

    /// Execute all tasks in priority order
    pub fn execute_all(&mut self) {
        // Sort by priority (lower = higher priority)
        self.tasks.sort_by_key(|t| t.priority);

        let tasks = std::mem::take(&mut self.tasks);
        for task in tasks {
            task.execute();
        }
    }

    /// Get number of pending tasks
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

impl Default for DeferredTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_init() {
        static LAZY: LazyInit<i32> = LazyInit::new(|| 42);

        assert!(!LAZY.is_initialized());
        assert_eq!(*LAZY.get(), 42);
        assert!(LAZY.is_initialized());
    }

    #[test]
    fn test_deferred_queue() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let order = Arc::new(AtomicUsize::new(0));
        let mut queue = DeferredTaskQueue::new();

        let order1 = Arc::clone(&order);
        queue.push(DeferredTask::new("low", 10, move || {
            assert_eq!(order1.fetch_add(1, Ordering::SeqCst), 1);
        }));

        let order2 = Arc::clone(&order);
        queue.push(DeferredTask::new("high", 1, move || {
            assert_eq!(order2.fetch_add(1, Ordering::SeqCst), 0);
        }));

        queue.execute_all();
        assert_eq!(order.load(Ordering::SeqCst), 2);
    }
}
