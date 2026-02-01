//! Translation bundle for a locale

use super::super::locale::Locale;
use std::collections::HashMap;

/// Translation bundle for a locale
#[derive(Debug, Clone, Default)]
pub struct TranslationBundle {
    /// Locale this bundle is for
    locale: Locale,
    /// Translation strings keyed by message ID
    messages: HashMap<String, String>,
}

impl TranslationBundle {
    /// Create a new empty bundle
    pub fn new(locale: Locale) -> Self {
        Self {
            locale,
            messages: HashMap::new(),
        }
    }

    /// Add a message to the bundle
    pub fn add(&mut self, id: impl Into<String>, message: impl Into<String>) {
        self.messages.insert(id.into(), message.into());
    }

    /// Get a message by ID
    pub fn get(&self, id: &str) -> Option<&str> {
        self.messages.get(id).map(|s| s.as_str())
    }

    /// Get a message with arguments substitution
    pub fn format(&self, id: &str, args: &HashMap<String, String>) -> Option<String> {
        self.get(id).map(|msg| {
            let mut result = msg.to_string();
            for (key, value) in args {
                result = result.replace(&format!("{{{}}}", key), value);
            }
            result
        })
    }

    /// Get the locale
    pub fn locale(&self) -> Locale {
        self.locale
    }

    /// Get number of messages
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if bundle is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}
