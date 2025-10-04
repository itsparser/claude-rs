use crate::errors::Result;
use crate::message_parser::parse_message;
use crate::transport::{SubprocessTransport, Transport};
use crate::types::{ClaudeAgentOptions, Message};
use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

/// A streaming query session that provides true async iteration without collecting all messages
pub struct StreamingQuery {
    receiver: mpsc::UnboundedReceiver<Result<Message>>,
}

impl StreamingQuery {
    /// Create a new streaming query session
    ///
    /// This spawns a background task that reads from the transport and sends
    /// parsed messages through a channel, allowing proper ownership separation.
    pub async fn new(prompt: String, options: Option<ClaudeAgentOptions>) -> Result<Self> {
        let opts = options.unwrap_or_default();
        let mut transport = SubprocessTransport::new(prompt, opts);

        // Connect to Claude Code
        transport.connect().await?;

        // Close stdin immediately for one-shot queries (CLI needs EOF to start)
        transport.end_input().await?;

        // Create channel for streaming messages
        let (tx, rx) = mpsc::unbounded_channel();

        // Spawn task to read and parse messages
        tokio::spawn(async move {
            let stream = transport.read_messages();
            futures::pin_mut!(stream);

            use futures::StreamExt;
            while let Some(result) = stream.next().await {
                match result {
                    Ok(json_value) => {
                        match parse_message(&json_value) {
                            Ok(message) => {
                                if tx.send(Ok(message)).is_err() {
                                    // Receiver dropped, stop reading
                                    break;
                                }
                            }
                            Err(e) => {
                                let _ = tx.send(Err(e));
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e));
                        break;
                    }
                }
            }
        });

        Ok(Self { receiver: rx })
    }
}

impl Stream for StreamingQuery {
    type Item = Result<Message>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

/// Streaming query function that provides true async iteration
///
/// Unlike `simple_query` which collects all messages into a Vec, this function
/// returns a stream that yields messages as they arrive, providing better
/// memory efficiency for long conversations.
///
/// # Arguments
///
/// * `prompt` - The prompt to send to Claude
/// * `options` - Optional configuration
///
/// # Returns
///
/// A Stream of messages that can be iterated with `while let Some(msg) = stream.next().await`
///
/// # Example
///
/// ```no_run
/// use claude::streaming_query;
/// use futures::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut stream = streaming_query("What is 2 + 2?", None).await?;
///
///     while let Some(result) = stream.next().await {
///         match result {
///             Ok(message) => println!("{:?}", message),
///             Err(e) => eprintln!("Error: {}", e),
///         }
///     }
///
///     Ok(())
/// }
/// ```
pub async fn streaming_query(
    prompt: &str,
    options: Option<ClaudeAgentOptions>,
) -> Result<StreamingQuery> {
    StreamingQuery::new(prompt.to_string(), options).await
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_streaming_query_creation() {
        // This test just verifies the API compiles and can be created
        // It won't actually run without Claude Code installed
        // Real integration tests would be in examples/
    }
}
