use claude::{ClaudeSDKClient, ClaudeAgentOptions, Message, ContentBlock};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Session Forking Example ===\n");
    println!("This example demonstrates forking a session to explore different paths.");
    println!("Forking creates a new branch from an existing session's state.\n");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  Step 1: cargo run --example session_forking new");
        println!("          (Creates base session)");
        println!();
        println!("  Step 2: cargo run --example session_forking fork <session-id> approach-a");
        println!("          (Fork 1: Try approach A)");
        println!();
        println!("  Step 3: cargo run --example session_forking fork <session-id> approach-b");
        println!("          (Fork 2: Try approach B)\n");
        return Ok(());
    }

    let mode = &args[1];

    if mode == "new" {
        // Create base session
        println!("--- Creating Base Session ---\n");

        let mut client = ClaudeSDKClient::new(None);
        client.connect().await?;

        println!("Sending: 'I need to solve this problem: How to optimize a slow database query?'\n");
        client.query(
            "I need to solve this problem: How to optimize a slow database query? Just acknowledge the problem for now.",
            None
        ).await?;

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
                    println!("--- Base Session Created ---");
                    println!("Session ID: {}", r.session_id);
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
            println!("🎯 Now you can fork this session:");
            println!("   Fork A: cargo run --example session_forking fork {} approach-a", sid);
            println!("   Fork B: cargo run --example session_forking fork {} approach-b", sid);
        }

    } else if mode == "fork" && args.len() >= 4 {
        let session_id = &args[2];
        let approach = &args[3];

        println!("--- Forking Session ({}) ---\n", approach);

        let options = ClaudeAgentOptions {
            resume: Some(session_id.clone()),
            fork_session: true,  // Enable forking
            ..Default::default()
        };

        let mut client = ClaudeSDKClient::new(Some(options));
        client.connect().await?;
        println!("✓ Connected and forked session\n");

        // Try different approaches based on fork
        let query = match approach.as_str() {
            "approach-a" => {
                "Great! Let's try optimizing with indexing. What indexes should we add?"
            }
            "approach-b" => {
                "Great! Let's try query rewriting instead. How can we refactor the query?"
            }
            _ => "Let's explore this problem."
        };

        println!("Sending: '{}'\n", query);
        client.query(query, None).await?;

        let mut response = client.receive_response();
        let mut forked_session_id: Option<String> = None;

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
                    forked_session_id = Some(r.session_id.clone());
                    println!("--- Forked Session Created ---");
                    println!("Original Session: {}", session_id);
                    println!("Forked Session: {}", r.session_id);
                    println!("Approach: {}", approach);
                    println!("Turns: {}", r.num_turns);
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

        println!("✓ Fork created successfully!");
        if let Some(fid) = forked_session_id {
            println!("\n💡 You can continue this fork:");
            println!("   cargo run --example session_resuming {}", fid);
        }

    } else {
        println!("Invalid arguments. Run without arguments for usage help.");
    }

    Ok(())
}
