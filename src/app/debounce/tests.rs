use std::time::Duration;

use super::*;

#[test]
fn test_debouncer_basic() {
    let debouncer = Debouncer::new(Duration::from_millis(100));

    // First call should execute
    assert!(debouncer.should_execute());

    // Immediate second call should not
    assert!(!debouncer.should_execute());
    assert!(debouncer.has_pending());

    // After interval, should execute again
    std::thread::sleep(Duration::from_millis(110));
    assert!(debouncer.should_execute());
    assert!(!debouncer.has_pending());
}

#[test]
fn test_batch_accumulator() {
    let accumulator = BatchAccumulator::<i32>::new(Duration::from_secs(10), 3);

    // Add items until batch is full
    assert!(accumulator.add(1).is_none());
    assert!(accumulator.add(2).is_none());
    let batch = accumulator.add(3);
    assert!(batch.is_some());
    assert_eq!(batch.unwrap(), vec![1, 2, 3]);
    assert!(accumulator.is_empty());
}

#[test]
fn test_update_coalescer() {
    let coalescer = UpdateCoalescer::new(Duration::from_millis(50));

    // Initially clean
    assert!(!coalescer.is_dirty());
    assert!(!coalescer.should_render());

    // Mark dirty
    coalescer.mark_dirty();
    assert!(coalescer.is_dirty());

    // First render should work
    assert!(coalescer.should_render());
    assert!(!coalescer.is_dirty());

    // Immediate render should not work
    coalescer.mark_dirty();
    assert!(!coalescer.should_render());

    // But force render should
    assert!(coalescer.force_render());
}
