use std::fmt;

/// Base error type for all Claude SDK errors
#[derive(Debug)]
pub enum ClaudeSDKError {
    /// Raised when unable to connect to Claude Code
    CLIConnectionError(String),
    /// Raised when Claude Code is not found or not installed
    CLINotFoundError { message: String, cli_path: Option<String> },
    /// Raised when the CLI process fails
    ProcessError {
        message: String,
        exit_code: Option<i32>,
        stderr: Option<String>,
    },
    /// Raised when unable to decode JSON from CLI output
    CLIJSONDecodeError {
        line: String,
        original_error: String,
    },
    /// Raised when unable to parse a message from CLI output
    MessageParseError {
        message: String,
        data: Option<serde_json::Value>,
    },
}

impl fmt::Display for ClaudeSDKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClaudeSDKError::CLIConnectionError(msg) => write!(f, "CLI Connection Error: {}", msg),
            ClaudeSDKError::CLINotFoundError { message, cli_path } => {
                if let Some(path) = cli_path {
                    write!(f, "{}: {}", message, path)
                } else {
                    write!(f, "{}", message)
                }
            }
            ClaudeSDKError::ProcessError {
                message,
                exit_code,
                stderr,
            } => {
                let mut msg = message.clone();
                if let Some(code) = exit_code {
                    msg = format!("{} (exit code: {})", msg, code);
                }
                if let Some(err) = stderr {
                    msg = format!("{}\nError output: {}", msg, err);
                }
                write!(f, "{}", msg)
            }
            ClaudeSDKError::CLIJSONDecodeError { line, original_error } => {
                let truncated = if line.len() > 100 {
                    format!("{}...", &line[..100])
                } else {
                    line.clone()
                };
                write!(f, "Failed to decode JSON: {} (error: {})", truncated, original_error)
            }
            ClaudeSDKError::MessageParseError { message, .. } => {
                write!(f, "Message Parse Error: {}", message)
            }
        }
    }
}

impl std::error::Error for ClaudeSDKError {}

// Convenience constructors
impl ClaudeSDKError {
    pub fn cli_connection_error(message: impl Into<String>) -> Self {
        ClaudeSDKError::CLIConnectionError(message.into())
    }

    pub fn cli_not_found(cli_path: Option<String>) -> Self {
        ClaudeSDKError::CLINotFoundError {
            message: "Claude Code not found".to_string(),
            cli_path,
        }
    }

    pub fn process_error(
        message: impl Into<String>,
        exit_code: Option<i32>,
        stderr: Option<String>,
    ) -> Self {
        ClaudeSDKError::ProcessError {
            message: message.into(),
            exit_code,
            stderr,
        }
    }

    pub fn json_decode_error(line: impl Into<String>, original_error: impl Into<String>) -> Self {
        ClaudeSDKError::CLIJSONDecodeError {
            line: line.into(),
            original_error: original_error.into(),
        }
    }

    pub fn message_parse_error(
        message: impl Into<String>,
        data: Option<serde_json::Value>,
    ) -> Self {
        ClaudeSDKError::MessageParseError {
            message: message.into(),
            data,
        }
    }
}

pub type Result<T> = std::result::Result<T, ClaudeSDKError>;
