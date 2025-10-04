/// Builder patterns for SDK types
use std::path::PathBuf;

use crate::types::{ClaudeAgentOptions, PermissionMode, SystemPromptConfig, SystemPromptPreset};

/// Fluent builder for ClaudeAgentOptions
///
/// Provides a chainable, discoverable API for configuration.
///
/// # Example
/// ```no_run
/// use claude::ClaudeOptionsBuilder;
/// use claude::PermissionMode;
///
/// let options = ClaudeOptionsBuilder::new()
///     .system_prompt("You are a helpful assistant")
///     .max_turns(5)
///     .model("claude-sonnet-4-5")
///     .permission_mode(PermissionMode::AcceptEdits)
///     .allow_tools(["Read", "Write", "Bash"])
///     .build();
/// ```
#[derive(Default, Clone)]
pub struct ClaudeOptionsBuilder {
    inner: ClaudeAgentOptions,
}

impl ClaudeOptionsBuilder {
    /// Create a new builder with default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set system prompt as text
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.inner.system_prompt = Some(SystemPromptConfig::Text(prompt.into()));
        self
    }

    /// Set system prompt from preset
    pub fn system_prompt_preset(mut self, preset: impl Into<String>) -> Self {
        self.inner.system_prompt = Some(SystemPromptConfig::Preset(SystemPromptPreset {
            r#type: "preset".to_string(),
            preset: preset.into(),
            append: None,
        }));
        self
    }

    /// Set system prompt from preset with appended text
    pub fn system_prompt_preset_with_append(
        mut self,
        preset: impl Into<String>,
        append: impl Into<String>,
    ) -> Self {
        self.inner.system_prompt = Some(SystemPromptConfig::Preset(SystemPromptPreset {
            r#type: "preset".to_string(),
            preset: preset.into(),
            append: Some(append.into()),
        }));
        self
    }

    /// Set maximum number of turns
    pub fn max_turns(mut self, turns: i32) -> Self {
        self.inner.max_turns = Some(turns);
        self
    }

    /// Set the model to use
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.inner.model = Some(model.into());
        self
    }

    /// Set permission mode
    pub fn permission_mode(mut self, mode: PermissionMode) -> Self {
        self.inner.permission_mode = Some(mode);
        self
    }

    /// Set allowed tools
    pub fn allow_tools<I, S>(mut self, tools: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.inner.allowed_tools = tools.into_iter().map(Into::into).collect();
        self
    }

    /// Add a single allowed tool
    pub fn allow_tool(mut self, tool: impl Into<String>) -> Self {
        self.inner.allowed_tools.push(tool.into());
        self
    }

    /// Set disallowed tools
    pub fn deny_tools<I, S>(mut self, tools: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.inner.disallowed_tools = tools.into_iter().map(Into::into).collect();
        self
    }

    /// Add a single disallowed tool
    pub fn deny_tool(mut self, tool: impl Into<String>) -> Self {
        self.inner.disallowed_tools.push(tool.into());
        self
    }

    /// Set working directory
    pub fn cwd(mut self, path: impl Into<PathBuf>) -> Self {
        self.inner.cwd = Some(path.into());
        self
    }

    /// Add a directory to context
    pub fn add_directory(mut self, dir: impl Into<PathBuf>) -> Self {
        self.inner.add_dirs.push(dir.into());
        self
    }

    /// Set multiple environment variables
    pub fn envs<I, K, V>(mut self, vars: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (k, v) in vars {
            self.inner.env.insert(k.into(), v.into());
        }
        self
    }

    /// Set a single environment variable
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.env.insert(key.into(), value.into());
        self
    }

    /// Enable conversation continuation
    pub fn continue_conversation(mut self, enable: bool) -> Self {
        self.inner.continue_conversation = enable;
        self
    }

    /// Resume a previous session
    pub fn resume_session(mut self, session_id: impl Into<String>) -> Self {
        self.inner.resume = Some(session_id.into());
        self.inner.fork_session = false;
        self
    }

    /// Fork from a previous session
    pub fn fork_session(mut self, session_id: impl Into<String>) -> Self {
        self.inner.resume = Some(session_id.into());
        self.inner.fork_session = true;
        self
    }

    /// Set permission prompt tool name
    pub fn permission_prompt_tool(mut self, name: impl Into<String>) -> Self {
        self.inner.permission_prompt_tool_name = Some(name.into());
        self
    }

    /// Set settings file path
    pub fn settings(mut self, path: impl Into<String>) -> Self {
        self.inner.settings = Some(path.into());
        self
    }

    /// Set maximum buffer size
    pub fn max_buffer_size(mut self, size: usize) -> Self {
        self.inner.max_buffer_size = Some(size);
        self
    }

    /// Set user identifier
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.inner.user = Some(user.into());
        self
    }

    /// Include partial messages in streaming
    pub fn include_partial_messages(mut self, include: bool) -> Self {
        self.inner.include_partial_messages = include;
        self
    }

    /// Build the final ClaudeAgentOptions
    pub fn build(self) -> ClaudeAgentOptions {
        self.inner
    }
}

impl ClaudeAgentOptions {
    /// Create a new builder
    pub fn builder() -> ClaudeOptionsBuilder {
        ClaudeOptionsBuilder::new()
    }

    /// Quick constructor with just system prompt
    pub fn with_system_prompt(prompt: impl Into<String>) -> Self {
        ClaudeOptionsBuilder::new().system_prompt(prompt).build()
    }

    /// Quick constructor with model
    pub fn with_model(model: impl Into<String>) -> Self {
        ClaudeOptionsBuilder::new().model(model).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let options = ClaudeOptionsBuilder::new()
            .system_prompt("test")
            .max_turns(5)
            .model("claude-sonnet-4-5")
            .build();

        assert!(matches!(options.system_prompt, Some(SystemPromptConfig::Text(ref s)) if s == "test"));
        assert_eq!(options.max_turns, Some(5));
        assert_eq!(options.model, Some("claude-sonnet-4-5".to_string()));
    }

    #[test]
    fn test_builder_tools() {
        let options = ClaudeOptionsBuilder::new()
            .allow_tools(["Read", "Write"])
            .deny_tool("Bash")
            .build();

        assert_eq!(options.allowed_tools, vec!["Read", "Write"]);
        assert_eq!(options.disallowed_tools, vec!["Bash"]);
    }

    #[test]
    fn test_builder_session() {
        let options = ClaudeOptionsBuilder::new().resume_session("session-123").build();

        assert_eq!(options.resume, Some("session-123".to_string()));
        assert!(!options.fork_session);
    }

    #[test]
    fn test_builder_fork() {
        let options = ClaudeOptionsBuilder::new().fork_session("session-456").build();

        assert_eq!(options.resume, Some("session-456".to_string()));
        assert!(options.fork_session);
    }

    #[test]
    fn test_quick_constructors() {
        let opt1 = ClaudeAgentOptions::with_system_prompt("test");
        assert!(matches!(opt1.system_prompt, Some(SystemPromptConfig::Text(ref s)) if s == "test"));

        let opt2 = ClaudeAgentOptions::with_model("claude-sonnet-4-5");
        assert_eq!(opt2.model, Some("claude-sonnet-4-5".to_string()));
    }
}
