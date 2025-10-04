use crate::errors::Result;
use crate::message_parser::parse_message;
use crate::transport::{SubprocessTransport, Transport};
use crate::types::{ClaudeAgentOptions, Message};
use futures::stream::StreamExt;

/// Simple query function that collects all messages from Claude Code
///
/// This is a simplified version that collects all messages into a Vec
/// for easier use in examples.
///
/// # Arguments
///
/// * `prompt` - The prompt to send to Claude
/// * `options` - Optional configuration
///
/// # Returns
///
/// A vector of all messages from the conversation
pub async fn simple_query(
    prompt: &str,
    options: Option<ClaudeAgentOptions>,
) -> Result<Vec<Message>> {
    let opts = options.unwrap_or_default();
    let mut transport = SubprocessTransport::new(prompt.to_string(), opts);

    // Connect to Claude Code
    transport.connect().await?;

    // Close stdin immediately for one-shot queries (CLI needs EOF to start)
    transport.end_input().await?;

    // Collect all messages
    let mut messages = Vec::new();
    let stream = transport.read_messages();
    futures::pin_mut!(stream);

    while let Some(result) = stream.next().await {
        let json_value = result?;
        let message = parse_message(&json_value)?;
        messages.push(message);
    }

    Ok(messages)
}
