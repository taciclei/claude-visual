//! OpenAI API Provider
//!
//! Integration with OpenAI's GPT models.

mod client;
mod models;
mod provider;
mod types;

#[cfg(test)]
mod tests;

pub use provider::OpenAIProvider;

/// OpenAI API base URL
pub(crate) const OPENAI_API_URL: &str = "https://api.openai.com/v1";
