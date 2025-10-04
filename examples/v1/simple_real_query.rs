/// Simplified Real Claude Code connection example
///
/// This example actually connects to Claude Code CLI and gets real responses.
/// It uses the simple_query function that collects all messages into a vector.
///
/// Prerequisites:
/// - Claude Code must be installed: npm install -g @anthropic-ai/claude-code
/// - ANTHROPIC_API_KEY must be set in environment
///
/// Run with: cargo run --example simple_real_query

use claude::{simple_query, ClaudeAgentOptions, ContentBlock, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple Real Claude Code Query Example ===\n");

    // Check if Claude Code is available
    if which::which("claude").is_err() {
        eprintln!("❌ Error: Claude Code CLI not found!");
        eprintln!("\nPlease install it with:");
        eprintln!("  npm install -g @anthropic-ai/claude-code");
        std::process::exit(1);
    }

    println!("✓ Claude Code CLI found\n");

    // Example 1: Simple query
    basic_query().await?;

    // Example 2: Query with options
    query_with_options().await?;

    println!("\n=== All Examples Complete ===");

    Ok(())
}

async fn basic_query() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 1: Basic Query ---");
    println!("Asking: What is 2 + 2?\n");

    let messages = simple_query("What is 2 + 2?", None).await?;

    for message in messages {
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

    println!();
    Ok(())
}

async fn query_with_options() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 2: Query with Options ---");
    println!("Asking with custom system prompt\n");

    let mut options = ClaudeAgentOptions::default();
    options.system_prompt = Some(claude::SystemPromptConfig::Text(
        "You are a helpful assistant that explains things simply in one sentence.".to_string(),
    ));
    options.max_turns = Some(1);

    let messages = simple_query("What is Rust programming language?", Some(options)).await?;

    for message in messages {
        match message {
            Message::Assistant(msg) => {
                println!("🤖 Claude:");
                for block in msg.content {
                    if let ContentBlock::Text { text } = block {
                        println!("  {}", text);
                    }
                }
            }
            Message::Result(result_msg) => {
                println!("\n✅ Query completed");
                println!("  Duration: {}ms", result_msg.duration_ms);
                if let Some(cost) = result_msg.total_cost_usd {
                    println!("  Cost: ${:.6}", cost);
                }
            }
            _ => {}
        }
    }

    println!();
    Ok(())
}
