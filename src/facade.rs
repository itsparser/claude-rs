/// High-level facade functions for common operations
use crate::builders::ClaudeOptionsBuilder;
use crate::extensions::MessageVecExt;
use crate::simple_query::simple_query;
use crate::streaming_query::{streaming_query, StreamingQuery};
use crate::types::{ClaudeAgentOptions, Message};
use crate::Result;

/// Ask Claude a simple question and get the text response
///
/// This is the simplest way to interact with Claude. For more control,
/// use `simple_query()` or `streaming_query()`.
///
/// # Example
/// ```no_run
/// use claude::ask;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let answer = ask("What is 2 + 2?").await?;
///     println!("Answer: {}", answer);
///     Ok(())
/// }
/// ```
pub async fn ask(prompt: impl AsRef<str>) -> Result<String> {
    let messages = simple_query(prompt.as_ref(), None).await?;
    Ok(messages.text_content())
}

/// Ask Claude with custom options and get text response
///
/// # Example
/// ```no_run
/// use claude::{ask_with_options, ClaudeAgentOptions};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let options = ClaudeAgentOptions::with_system_prompt("You are a math tutor");
///     let answer = ask_with_options("What is calculus?", options).await?;
///     println!("{}", answer);
///     Ok(())
/// }
/// ```
pub async fn ask_with_options(
    prompt: impl AsRef<str>,
    options: ClaudeAgentOptions,
) -> Result<String> {
    let messages = simple_query(prompt.as_ref(), Some(options)).await?;
    Ok(messages.text_content())
}

/// Fluent query builder for quick interactions
///
/// Provides a chainable API for common query patterns.
///
/// # Example
/// ```no_run
/// use claude::QuickQuery;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let answer = QuickQuery::new("Explain async/await in Rust")
///         .with_system_prompt("You are a Rust expert")
///         .max_turns(1)
///         .ask()
///         .await?;
///     println!("{}", answer);
///     Ok(())
/// }
/// ```
pub struct QuickQuery {
    prompt: String,
    options: ClaudeOptionsBuilder,
}

impl QuickQuery {
    /// Create a new quick query with a prompt
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            options: ClaudeOptionsBuilder::new(),
        }
    }

    /// Set system prompt
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.options = self.options.system_prompt(prompt);
        self
    }

    /// Set the model to use
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.options = self.options.model(model);
        self
    }

    /// Set maximum turns
    pub fn max_turns(mut self, turns: i32) -> Self {
        self.options = self.options.max_turns(turns);
        self
    }

    /// Set allowed tools
    pub fn allow_tools<I, S>(mut self, tools: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.options = self.options.allow_tools(tools);
        self
    }

    /// Execute query and get text response
    pub async fn ask(self) -> Result<String> {
        ask_with_options(self.prompt, self.options.build()).await
    }

    /// Execute query and get full message list
    pub async fn query(self) -> Result<Vec<Message>> {
        simple_query(&self.prompt, Some(self.options.build())).await
    }

    /// Execute query and get streaming response
    pub async fn stream(self) -> Result<StreamingQuery> {
        streaming_query(&self.prompt, Some(self.options.build())).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_query_builder() {
        let query = QuickQuery::new("test prompt")
            .with_system_prompt("system")
            .max_turns(5)
            .with_model("claude-sonnet-4-5")
            .allow_tools(["Read", "Write"]);

        assert_eq!(query.prompt, "test prompt");
    }

    #[test]
    fn test_quick_query_new() {
        let query = QuickQuery::new("test");
        assert_eq!(query.prompt, "test");
    }
}
