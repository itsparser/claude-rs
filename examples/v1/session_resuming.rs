use claude::{ClaudeSDKClient, ClaudeAgentOptions, Message, ContentBlock};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Session Resuming Example ===\n");
    println!("This example demonstrates resuming a previous Claude Code session.");
    println!("Sessions are stored by Claude Code and can be resumed later.\n");

    // First, let's check if we have a session ID from command line
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  Step 1: cargo run --example session_resuming new");
        println!("          (Creates a new session and saves the session ID)");
        println!();
        println!("  Step 2: cargo run --example session_resuming <session-id>");
        println!("          (Resumes the previous session)\n");
        return Ok(());
    }

    let mode = &args[1];

    if mode == "new" {
        // Create a NEW session
        println!("--- Creating NEW Session ---\n");

        let mut client = ClaudeSDKClient::new(None);
        client.connect().await?;
        println!("✓ Connected to Claude Code\n");

        // Send initial query
        println!("Sending: 'Remember this number: 42. What is it?'\n");
        client.query("Remember this number: 42. What is it?", None).await?;

        // Receive response and extract session ID
        let mut response = client.receive_response();
        let mut session_id: Option<String> = None;

        while let Some(result) = response.next().await {
            match result {
                Ok(Message::Assistant(msg)) => {
                    for block in msg.content {
                        if let ContentBlock::Text { text } = block {
                            println!("Claude: {}\n", text);
                        }
                    }
                }
                Ok(Message::Result(r)) => {
                    session_id = Some(r.session_id.clone());
                    println!("--- Session Created ---");
                    println!("Session ID: {}", r.session_id);
                    println!("Duration: {}ms", r.duration_ms);
                    println!("Turns: {}", r.num_turns);
                    if let Some(cost) = r.total_cost_usd {
                        println!("Cost: ${:.6}", cost);
                    }
                    println!();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        client.close().await?;

        if let Some(sid) = session_id {
            println!("\n🎯 To resume this session, run:");
            println!("   cargo run --example session_resuming {}", sid);
        }

    } else {
        // RESUME existing session
        let session_id = mode;
        println!("--- Resuming Session: {} ---\n", session_id);

        let options = ClaudeAgentOptions {
            resume: Some(session_id.clone()),
            ..Default::default()
        };

        let mut client = ClaudeSDKClient::new(Some(options));
        client.connect().await?;
        println!("✓ Connected and resumed session\n");

        // Ask about the previous conversation
        println!("Sending: 'What number did I ask you to remember?'\n");
        client.query("What number did I ask you to remember?", None).await?;

        // Receive response
        let mut response = client.receive_response();

        while let Some(result) = response.next().await {
            match result {
                Ok(Message::Assistant(msg)) => {
                    for block in msg.content {
                        if let ContentBlock::Text { text } = block {
                            println!("Claude: {}\n", text);
                        }
                    }
                }
                Ok(Message::Result(r)) => {
                    println!("--- Session Resumed Successfully ---");
                    println!("Session ID: {}", r.session_id);
                    println!("Duration: {}ms", r.duration_ms);
                    println!("Turns: {}", r.num_turns);
                    if let Some(cost) = r.total_cost_usd {
                        println!("Cost: ${:.6}", cost);
                    }
                    println!();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        client.close().await?;

        println!("✓ Session resumed successfully!");
        println!("\nNote: Claude should remember the number 42 from the first session.");
    }

    Ok(())
}
