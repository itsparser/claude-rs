use crate::errors::{ClaudeSDKError, Result};
use crate::permissions::CanUseToolCallback;
use crate::query::Query;
use crate::transport::{SubprocessTransport, Transport};
use crate::types::{ClaudeAgentOptions, Message};
use futures::stream::Stream;
use serde_json::json;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

/// Client for bidirectional, interactive conversations with Claude Code.
///
/// This client provides full control over the conversation flow with support
/// for streaming, interrupts, and dynamic message sending. For simple one-shot
/// queries, consider using the query() function instead.
///
/// Key features:
/// - **Bidirectional**: Send and receive messages at any time
/// - **Stateful**: Maintains conversation context across messages
/// - **Interactive**: Send follow-ups based on responses
/// - **Control flow**: Support for interrupts and session management
///
/// When to use ClaudeSDKClient:
/// - Building chat interfaces or conversational UIs
/// - Interactive debugging or exploration sessions
/// - Multi-turn conversations with context
/// - When you need to react to Claude's responses
/// - Real-time applications with user input
/// - When you need interrupt capabilities
///
/// When to use query() instead:
/// - Simple one-off questions
/// - Batch processing of prompts
/// - Fire-and-forget automation scripts
/// - When all inputs are known upfront
/// - Stateless operations
pub struct ClaudeSDKClient {
    options: ClaudeAgentOptions,
    query: Option<Query>,
    can_use_tool: Option<CanUseToolCallback>,
}

impl ClaudeSDKClient {
    /// Create a new ClaudeSDKClient instance
    pub fn new(options: Option<ClaudeAgentOptions>) -> Self {
        Self {
            options: options.unwrap_or_default(),
            query: None,
            can_use_tool: None,
        }
    }

    /// Create a new ClaudeSDKClient with can_use_tool callback
    pub fn with_can_use_tool(
        options: Option<ClaudeAgentOptions>,
        can_use_tool: CanUseToolCallback,
    ) -> Self {
        Self {
            options: options.unwrap_or_default(),
            query: None,
            can_use_tool: Some(can_use_tool),
        }
    }

    /// Connect to Claude Code and start the session
    ///
    /// # Example
    /// ```no_run
    /// use claude::{ClaudeSDKClient, ClaudeAgentOptions};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = ClaudeSDKClient::new(None);
    ///     client.connect().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect(&mut self) -> Result<()> {
        // Create transport with empty prompt for interactive mode
        let mut transport = SubprocessTransport::new(String::new(), self.options.clone());

        // Connect the transport (start the subprocess)
        transport.connect().await?;

        // Create Query instance for control protocol
        let mut query = if let Some(ref callback) = self.can_use_tool {
            Query::with_can_use_tool(transport, true, callback.clone())
        } else {
            Query::new(transport, true)
        };

        // Start reading messages
        query.start().await?;

        // Initialize control protocol (with timeout to handle CLI versions that don't support it)
        // If initialization fails, we continue anyway - it's only needed for hooks
        let _ = tokio::time::timeout(
            tokio::time::Duration::from_secs(2),
            query.initialize()
        ).await;

        self.query = Some(query);
        Ok(())
    }

    /// Receive all messages from Claude
    ///
    /// Returns a stream of messages that you can iterate over.
    ///
    /// # Example
    /// ```no_run
    /// use claude::{ClaudeSDKClient, Message};
    /// use futures::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = ClaudeSDKClient::new(None);
    ///     client.connect().await?;
    ///
    ///     let mut messages = client.receive_messages();
    ///     while let Some(result) = messages.next().await {
    ///         match result {
    ///             Ok(msg) => println!("{:?}", msg),
    ///             Err(e) => eprintln!("Error: {}", e),
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn receive_messages(&mut self) -> MessageStream {
        if let Some(ref mut query) = self.query {
            let rx = query.receive_messages();
            MessageStream { receiver: rx }
        } else {
            // Return empty stream if not connected
            let (_tx, rx) = mpsc::unbounded_channel();
            MessageStream { receiver: rx }
        }
    }

    /// Send a new query to Claude
    ///
    /// # Arguments
    /// * `prompt` - The message to send to Claude
    /// * `session_id` - Optional session identifier (defaults to "default")
    ///
    /// # Example
    /// ```no_run
    /// use claude::ClaudeSDKClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = ClaudeSDKClient::new(None);
    ///     client.connect().await?;
    ///     client.query("What is 2 + 2?", None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn query(&mut self, prompt: &str, session_id: Option<&str>) -> Result<()> {
        let query = self
            .query
            .as_mut()
            .ok_or_else(|| ClaudeSDKError::cli_connection_error("Not connected. Call connect() first.".to_string()))?;

        let session = session_id.unwrap_or("default");

        // Build user message
        let message = json!({
            "type": "user",
            "message": {
                "role": "user",
                "content": prompt
            },
            "parent_tool_use_id": null,
            "session_id": session
        });

        // Send via query's transport
        query.send_message(message).await?;

        Ok(())
    }

    /// Receive messages until and including a ResultMessage
    ///
    /// This is a convenience method that yields messages and automatically
    /// terminates after receiving a ResultMessage.
    ///
    /// # Example
    /// ```no_run
    /// use claude::{ClaudeSDKClient, Message};
    /// use futures::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = ClaudeSDKClient::new(None);
    ///     client.connect().await?;
    ///     client.query("What is the capital of France?", None).await?;
    ///
    ///     let mut response = client.receive_response();
    ///     while let Some(result) = response.next().await {
    ///         match result {
    ///             Ok(msg) => println!("{:?}", msg),
    ///             Err(e) => eprintln!("Error: {}", e),
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn receive_response(&mut self) -> ResponseStream {
        let messages = self.receive_messages();
        ResponseStream {
            inner: messages,
            terminated: false,
        }
    }

    /// Send an interrupt signal to stop the current operation
    ///
    /// Only works in streaming mode.
    pub async fn interrupt(&mut self) -> Result<()> {
        let query = self
            .query
            .as_mut()
            .ok_or_else(|| ClaudeSDKError::cli_connection_error("Not connected. Call connect() first.".to_string()))?;

        query.interrupt().await
    }

    /// Change permission mode during conversation
    ///
    /// # Arguments
    /// * `mode` - The permission mode to set:
    ///   - "default": CLI prompts for dangerous tools
    ///   - "acceptEdits": Auto-accept file edits
    ///   - "bypassPermissions": Allow all tools (use with caution)
    ///
    /// # Example
    /// ```no_run
    /// use claude::ClaudeSDKClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = ClaudeSDKClient::new(None);
    ///     client.connect().await?;
    ///
    ///     // Start with default permissions
    ///     client.query("Help me analyze this codebase", None).await?;
    ///
    ///     // Switch to auto-accept edits
    ///     client.set_permission_mode("acceptEdits").await?;
    ///     client.query("Now implement the fix", None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn set_permission_mode(&mut self, mode: &str) -> Result<()> {
        let query = self
            .query
            .as_mut()
            .ok_or_else(|| ClaudeSDKError::cli_connection_error("Not connected. Call connect() first.".to_string()))?;

        query.set_permission_mode(mode).await
    }

    /// Change the AI model during conversation
    ///
    /// # Arguments
    /// * `model` - The model to use (e.g., "claude-sonnet-4-5")
    ///
    /// # Example
    /// ```no_run
    /// use claude::ClaudeSDKClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = ClaudeSDKClient::new(None);
    ///     client.connect().await?;
    ///
    ///     client.query("Help me understand this problem", None).await?;
    ///
    ///     // Switch model
    ///     client.set_model(Some("claude-sonnet-4-5")).await?;
    ///     client.query("Now implement the solution", None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn set_model(&mut self, model: Option<&str>) -> Result<()> {
        let query = self
            .query
            .as_mut()
            .ok_or_else(|| ClaudeSDKError::cli_connection_error("Not connected. Call connect() first.".to_string()))?;

        query.set_model(model).await
    }

    // Session Management Methods

    /// Create a client that resumes from an existing session
    ///
    /// # Arguments
    /// * `session_id` - The session ID to resume from
    /// * `options` - Optional additional configuration
    ///
    /// # Example
    /// ```no_run
    /// use claude::ClaudeSDKClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = ClaudeSDKClient::resume("session-123", None);
    ///     client.connect().await?;
    ///     client.query("What did we discuss earlier?", None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn resume(session_id: impl Into<String>, options: Option<ClaudeAgentOptions>) -> Self {
        let mut opts = options.unwrap_or_default();
        opts.resume = Some(session_id.into());

        Self {
            options: opts,
            query: None,
            can_use_tool: None,
        }
    }

    /// Create a client that forks from an existing session
    ///
    /// # Arguments
    /// * `session_id` - The session ID to fork from
    /// * `options` - Optional additional configuration
    ///
    /// # Example
    /// ```no_run
    /// use claude::ClaudeSDKClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Fork creates a new branch from the original session
    ///     let mut fork = ClaudeSDKClient::fork("session-123", None);
    ///     fork.connect().await?;
    ///     fork.query("Let's try a different approach", None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn fork(session_id: impl Into<String>, options: Option<ClaudeAgentOptions>) -> Self {
        let mut opts = options.unwrap_or_default();
        opts.resume = Some(session_id.into());
        opts.fork_session = true;

        Self {
            options: opts,
            query: None,
            can_use_tool: None,
        }
    }

    /// Create a client with continuous conversation enabled
    ///
    /// This maintains context across multiple query() calls within the same session.
    ///
    /// # Example
    /// ```no_run
    /// use claude::ClaudeSDKClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = ClaudeSDKClient::with_continuous_conversation(None);
    ///     client.connect().await?;
    ///
    ///     client.query("I'm working on a Rust project", None).await?;
    ///     client.query("What language am I using?", None).await?; // Remembers context
    ///     Ok(())
    /// }
    /// ```
    pub fn with_continuous_conversation(options: Option<ClaudeAgentOptions>) -> Self {
        let mut opts = options.unwrap_or_default();
        opts.continue_conversation = true;

        Self {
            options: opts,
            query: None,
            can_use_tool: None,
        }
    }

    /// Disconnect from Claude Code and clean up resources
    pub async fn close(mut self) -> Result<()> {
        if let Some(query) = self.query.take() {
            query.close().await?;
        }
        Ok(())
    }
}

/// Stream of messages from Claude
pub struct MessageStream {
    receiver: mpsc::UnboundedReceiver<Result<Message>>,
}

impl Stream for MessageStream {
    type Item = Result<Message>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

/// Stream of messages that terminates after a ResultMessage
pub struct ResponseStream {
    inner: MessageStream,
    terminated: bool,
}

impl Stream for ResponseStream {
    type Item = Result<Message>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.terminated {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Ready(Some(Ok(msg))) => {
                // Check if this is a ResultMessage
                if matches!(msg, Message::Result(_)) {
                    self.terminated = true;
                }
                Poll::Ready(Some(Ok(msg)))
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => {
                self.terminated = true;
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ClaudeSDKClient::new(None);
        assert!(client.query.is_none());
    }

    #[test]
    fn test_client_with_options() {
        let opts = ClaudeAgentOptions {
            max_turns: Some(10),
            ..Default::default()
        };
        let client = ClaudeSDKClient::new(Some(opts));
        assert_eq!(client.options.max_turns, Some(10));
    }
}
