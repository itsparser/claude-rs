use crate::errors::{ClaudeSDKError, Result};
use crate::types::ClaudeAgentOptions;
use async_trait::async_trait;
use futures::stream::Stream;
use futures::FutureExt;
use serde_json::Value;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command, ChildStdin};

#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&mut self) -> Result<()>;
    async fn write(&mut self, data: &str) -> Result<()>;
    async fn end_input(&mut self) -> Result<()>;
    fn read_messages(&mut self) -> impl Stream<Item = Result<Value>> + Send;
    async fn close(&mut self) -> Result<()>;
    fn is_ready(&self) -> bool;
}

pub struct SubprocessTransport {
    prompt: String,
    options: ClaudeAgentOptions,
    cli_path: String,
    process: Option<Child>,
    stdin: Option<ChildStdin>,
    ready: bool,
}

impl SubprocessTransport {
    pub fn new(prompt: String, options: ClaudeAgentOptions) -> Self {
        let cli_path = Self::find_claude_cli().unwrap_or_else(|_| "claude".to_string());

        Self {
            prompt,
            options,
            cli_path,
            process: None,
            stdin: None,
            ready: false,
        }
    }

    fn find_claude_cli() -> Result<String> {
        // Try to find claude in PATH
        if let Ok(path) = which::which("claude") {
            return Ok(path.to_string_lossy().to_string());
        }

        // Common installation locations
        let locations = vec![
            PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".npm-global/bin/claude"),
            PathBuf::from("/usr/local/bin/claude"),
            PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".local/bin/claude"),
        ];

        for path in locations {
            if path.exists() {
                return Ok(path.to_string_lossy().to_string());
            }
        }

        Err(ClaudeSDKError::cli_not_found(None))
    }

    fn build_command(&self) -> Vec<String> {
        let mut cmd = vec![
            "--output-format".to_string(),
            "stream-json".to_string(),
            "--verbose".to_string(),
        ];

        // System prompt
        if let Some(ref prompt) = self.options.system_prompt {
            match prompt {
                crate::types::SystemPromptConfig::Text(text) => {
                    cmd.push("--system-prompt".to_string());
                    cmd.push(text.clone());
                }
                crate::types::SystemPromptConfig::Preset(preset) => {
                    if let Some(ref append) = preset.append {
                        cmd.push("--append-system-prompt".to_string());
                        cmd.push(append.clone());
                    }
                }
            }
        }

        // Allowed tools
        if !self.options.allowed_tools.is_empty() {
            cmd.push("--allowedTools".to_string());
            cmd.push(self.options.allowed_tools.join(","));
        }

        // Max turns
        if let Some(max_turns) = self.options.max_turns {
            cmd.push("--max-turns".to_string());
            cmd.push(max_turns.to_string());
        }

        // Permission mode
        if let Some(ref mode) = self.options.permission_mode {
            cmd.push("--permission-mode".to_string());
            cmd.push(match mode {
                crate::types::PermissionMode::Default => "default",
                crate::types::PermissionMode::AcceptEdits => "acceptEdits",
                crate::types::PermissionMode::Plan => "plan",
                crate::types::PermissionMode::BypassPermissions => "bypassPermissions",
            }.to_string());
        }

        // Model
        if let Some(ref model) = self.options.model {
            cmd.push("--model".to_string());
            cmd.push(model.clone());
        }

        // Add the prompt for one-shot mode
        // For interactive mode (empty prompt), don't add --print flag
        if !self.prompt.is_empty() {
            cmd.push("--print".to_string());
            cmd.push("--".to_string());
            cmd.push(self.prompt.clone());
        }

        cmd
    }
}

#[async_trait]
impl Transport for SubprocessTransport {
    async fn connect(&mut self) -> Result<()> {
        if self.process.is_some() {
            return Ok(());
        }

        let args = self.build_command();

        let mut command = Command::new(&self.cli_path);
        command
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("CLAUDE_CODE_ENTRYPOINT", "sdk-rust")
            .env("CLAUDE_AGENT_SDK_VERSION", env!("CARGO_PKG_VERSION"));

        if let Some(ref cwd) = self.options.cwd {
            command.current_dir(cwd);
        }

        let mut child = command
            .spawn()
            .map_err(|e| ClaudeSDKError::cli_connection_error(format!("Failed to spawn Claude Code: {}", e)))?;

        // Take ownership of stdin for writing
        self.stdin = child.stdin.take();
        self.process = Some(child);
        self.ready = true;

        Ok(())
    }

    async fn write(&mut self, data: &str) -> Result<()> {
        // Check if ready
        if !self.ready {
            return Err(ClaudeSDKError::cli_connection_error(
                "Transport is not ready for writing".to_string(),
            ));
        }

        // Check if stdin is available
        let stdin = self.stdin.as_mut().ok_or_else(|| {
            ClaudeSDKError::cli_connection_error("Stdin not available for writing".to_string())
        })?;

        // Check if process is still alive
        if let Some(ref mut process) = self.process {
            if let Ok(Some(exit_status)) = process.try_wait() {
                return Err(ClaudeSDKError::process_error(
                    format!("Cannot write to terminated process (exit code: {:?})", exit_status.code()),
                    exit_status.code(),
                    None,
                ));
            }
        }

        // Write data to stdin
        stdin
            .write_all(data.as_bytes())
            .await
            .map_err(|e| {
                self.ready = false;
                ClaudeSDKError::cli_connection_error(format!(
                    "Failed to write to process stdin: {}",
                    e
                ))
            })?;

        // Flush to ensure data is sent
        stdin.flush().await.map_err(|e| {
            self.ready = false;
            ClaudeSDKError::cli_connection_error(format!("Failed to flush stdin: {}", e))
        })?;

        Ok(())
    }

    async fn end_input(&mut self) -> Result<()> {
        if let Some(mut stdin) = self.stdin.take() {
            let _ = stdin.shutdown().await;
        }
        Ok(())
    }

    fn read_messages(&mut self) -> impl Stream<Item = Result<Value>> + Send {
        let process = self.process.take();

        async move {
            let mut results = Vec::new();

            if let Some(mut process) = process {
                if let Some(stdout) = process.stdout.take() {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }

                        match serde_json::from_str::<Value>(line) {
                            Ok(value) => results.push(Ok(value)),
                            Err(e) => {
                                results.push(Err(ClaudeSDKError::json_decode_error(
                                    line.to_string(),
                                    e.to_string(),
                                )));
                            }
                        }
                    }
                }
            }

            futures::stream::iter(results)
        }
        .flatten_stream()
    }

    async fn close(&mut self) -> Result<()> {
        self.ready = false;

        if let Some(mut process) = self.process.take() {
            // Try to kill the process
            let _ = process.kill().await;
            let _ = process.wait().await;
        }

        Ok(())
    }

    fn is_ready(&self) -> bool {
        self.ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transport_write_not_ready() {
        let opts = ClaudeAgentOptions::default();
        let mut transport = SubprocessTransport::new("test".to_string(), opts);

        // Should fail when not connected
        let result = transport.write("test data\n").await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not ready for writing"));
    }

    #[tokio::test]
    async fn test_transport_end_input() {
        let opts = ClaudeAgentOptions::default();
        let mut transport = SubprocessTransport::new("test".to_string(), opts);

        // Should succeed even without connection
        let result = transport.end_input().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_transport_creation() {
        let opts = ClaudeAgentOptions::default();
        let transport = SubprocessTransport::new("test prompt".to_string(), opts);

        assert!(!transport.is_ready());
        assert_eq!(transport.prompt, "test prompt");
    }
}
