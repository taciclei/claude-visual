//! Windowed message loader for virtualized lists

use std::collections::VecDeque;
use super::{PageInfo, PaginatedResult};

/// Windowed message loader for virtualized list
///
/// Maintains a window of loaded messages for efficient rendering.
pub struct MessageWindow<T> {
    /// All loaded items in order (oldest to newest)
    pub(crate) items: VecDeque<T>,
    /// Maximum items to keep in memory
    pub(crate) max_items: usize,
    /// ID extractor function type marker
    _marker: std::marker::PhantomData<fn(&T) -> String>,
    /// Whether there are older items to load
    pub has_older: bool,
    /// Whether there are newer items to load
    pub has_newer: bool,
    /// First item cursor
    pub(crate) first_cursor: Option<String>,
    /// Last item cursor
    pub(crate) last_cursor: Option<String>,
}

impl<T: Clone> MessageWindow<T> {
    /// Create a new message window
    pub fn new(max_items: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(max_items),
            max_items,
            _marker: std::marker::PhantomData,
            has_older: false,
            has_newer: false,
            first_cursor: None,
            last_cursor: None,
        }
    }

    /// Load initial messages (newest)
    pub fn load_initial<F>(&mut self, result: PaginatedResult<T>, id_fn: F)
    where
        F: Fn(&T) -> String,
    {
        self.items.clear();
        for item in result.items {
            self.items.push_back(item);
        }
        self.has_older = result.page_info.has_previous_page;
        self.has_newer = result.page_info.has_next_page;
        self.update_cursors(&id_fn);
    }

    /// Prepend older messages
    pub fn prepend_older<F>(&mut self, result: PaginatedResult<T>, id_fn: F)
    where
        F: Fn(&T) -> String,
    {
        // Add items to the front
        for item in result.items.into_iter().rev() {
            self.items.push_front(item);
        }

        // Trim excess from the end
        while self.items.len() > self.max_items {
            self.items.pop_back();
            self.has_newer = true;
        }

        self.has_older = result.page_info.has_previous_page;
        self.update_cursors(&id_fn);
    }

    /// Append newer messages
    pub fn append_newer<F>(&mut self, result: PaginatedResult<T>, id_fn: F)
    where
        F: Fn(&T) -> String,
    {
        // Add items to the back
        for item in result.items {
            self.items.push_back(item);
        }

        // Trim excess from the front
        while self.items.len() > self.max_items {
            self.items.pop_front();
            self.has_older = true;
        }

        self.has_newer = result.page_info.has_next_page;
        self.update_cursors(&id_fn);
    }

    /// Add a single new message at the end
    pub fn push_new<F>(&mut self, item: T, id_fn: F)
    where
        F: Fn(&T) -> String,
    {
        self.items.push_back(item);

        // Trim excess from the front
        while self.items.len() > self.max_items {
            self.items.pop_front();
            self.has_older = true;
        }

        self.update_cursors(&id_fn);
    }

    /// Get all items in the window
    pub fn items(&self) -> &VecDeque<T> {
        &self.items
    }

    /// Get items as a slice (for rendering)
    pub fn as_slice(&self) -> Vec<&T> {
        self.items.iter().collect()
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get cursor for loading older messages
    pub fn older_cursor(&self) -> Option<&str> {
        self.first_cursor.as_deref()
    }

    /// Get cursor for loading newer messages
    pub fn newer_cursor(&self) -> Option<&str> {
        self.last_cursor.as_deref()
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.has_older = false;
        self.has_newer = false;
        self.first_cursor = None;
        self.last_cursor = None;
    }

    /// Update cursors based on current items
    fn update_cursors<F>(&mut self, id_fn: &F)
    where
        F: Fn(&T) -> String,
    {
        self.first_cursor = self.items.front().map(id_fn);
        self.last_cursor = self.items.back().map(id_fn);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_window() {
        #[derive(Clone, Debug)]
        struct TestMessage {
            id: String,
            content: String,
        }

        let mut window: MessageWindow<TestMessage> = MessageWindow::new(5);

        // Load initial messages
        let initial = PaginatedResult {
            items: vec![
                TestMessage {
                    id: "1".to_string(),
                    content: "First".to_string(),
                },
                TestMessage {
                    id: "2".to_string(),
                    content: "Second".to_string(),
                },
            ],
            page_info: PageInfo {
                has_previous_page: true,
                has_next_page: false,
                start_cursor: Some("1".to_string()),
                end_cursor: Some("2".to_string()),
                total_count: Some(10),
            },
        };

        window.load_initial(initial, |m| m.id.clone());

        assert_eq!(window.len(), 2);
        assert!(window.has_older);
        assert!(!window.has_newer);
        assert_eq!(window.older_cursor(), Some("1"));
        assert_eq!(window.newer_cursor(), Some("2"));
    }
}
