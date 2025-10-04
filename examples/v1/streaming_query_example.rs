/// Streaming query example - demonstrates true async iteration
///
/// Unlike simple_query which collects all messages into a Vec,
/// this example shows streaming where messages are processed as they arrive.
///
/// Prerequisites:
/// - Claude Code must be installed: npm install -g @anthropic-ai/claude-code
///
/// Run with: cargo run --example streaming_query_example

use claude::{streaming_query, ContentBlock, Message};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Streaming Query Example ===\n");

    // Check if Claude Code is available
    if which::which("claude").is_err() {
        eprintln!("❌ Error: Claude Code CLI not found!");
        eprintln!("\nPlease install it with:");
        eprintln!("  npm install -g @anthropic-ai/claude-code");
        std::process::exit(1);
    }

    println!("✓ Claude Code CLI found\n");
    println!("Asking: What is 2 + 2?\n");
    println!("Processing messages as they stream in...\n");

    // Create streaming query - messages will arrive as they're generated
    let mut stream = streaming_query("What is 2 + 2?", None).await?;

    let mut message_count = 0;

    // Process messages as they arrive (true streaming, not collected)
    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => {
                message_count += 1;

                match message {
                    Message::Assistant(msg) => {
                        println!("🤖 Claude (model: {}):", msg.model);
                        for block in msg.content {
                            match block {
                                ContentBlock::Text { text } => {
                                    println!("  {}", text);
                                }
                                ContentBlock::Thinking { thinking, .. } => {
                                    println!("  💭 Thinking: {}", thinking);
                                }
                                _ => {}
                            }
                        }
                    }
                    Message::Result(result_msg) => {
                        println!("\n✅ Query completed");
                        println!("  Duration: {}ms", result_msg.duration_ms);
                        println!("  Turns: {}", result_msg.num_turns);
                        if let Some(cost) = result_msg.total_cost_usd {
                            println!("  Cost: ${:.6}", cost);
                        }
                    }
                    Message::System(sys) => {
                        if sys.subtype == "status" {
                            println!("  📋 Status update");
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                break;
            }
        }
    }

    println!("\n✅ Processed {} messages via streaming", message_count);
    println!("\nNote: This used true streaming - messages were processed");
    println!("as they arrived, not collected into a Vec first!");

    Ok(())
}
