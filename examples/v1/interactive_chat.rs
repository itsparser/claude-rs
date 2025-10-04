/// Interactive chat example using ClaudeSDKClient
///
/// This demonstrates bidirectional conversation with Claude Code.
/// It shows how to:
/// - Connect to Claude
/// - Send queries
/// - Receive and process responses
/// - Use receive_response() to get a complete response
///
/// Run with: cargo run --example interactive_chat

use claude::{ClaudeSDKClient, ClaudeAgentOptions, Message, ContentBlock};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Interactive Chat Example ===\n");

    // Create options for the client
    let options = ClaudeAgentOptions {
        max_turns: Some(5),
        ..Default::default()
    };

    // Create and connect the client
    let mut client = ClaudeSDKClient::new(Some(options));
    println!("Connecting to Claude Code...");
    client.connect().await?;
    println!("Connected!\n");

    // Example 1: Simple question and response
    println!("Example 1: Simple Q&A");
    println!("---");
    client.query("What is 2 + 2?", None).await?;

    let mut response = client.receive_response();
    while let Some(result) = response.next().await {
        match result {
            Ok(Message::Assistant(msg)) => {
                for block in msg.content {
                    if let ContentBlock::Text { text } = block {
                        println!("Claude: {}", text);
                    }
                }
            }
            Ok(Message::Result(result)) => {
                println!("\nCost: ${:.6}", result.total_cost_usd.unwrap_or(0.0));
                println!("Turns: {}", result.num_turns);
            }
            Ok(_) => {} // Ignore other message types
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    println!("\n---\n");

    // Example 2: Follow-up question
    println!("Example 2: Follow-up question");
    println!("---");
    client.query("What is that number squared?", None).await?;

    let mut response2 = client.receive_response();
    while let Some(result) = response2.next().await {
        match result {
            Ok(Message::Assistant(msg)) => {
                for block in msg.content {
                    if let ContentBlock::Text { text } = block {
                        println!("Claude: {}", text);
                    }
                }
            }
            Ok(Message::Result(result)) => {
                println!("\nCost: ${:.6}", result.total_cost_usd.unwrap_or(0.0));
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    println!("\n---\n");

    // Clean up
    println!("Disconnecting...");
    client.close().await?;
    println!("Done!");

    Ok(())
}
