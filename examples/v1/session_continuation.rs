use claude::{ClaudeSDKClient, ClaudeAgentOptions, Message, ContentBlock};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Session Continuation Example ===\n");
    println!("This example demonstrates continue_conversation option.");
    println!("This allows multiple queries in the same conversational context.\n");

    println!("--- Creating Continuous Conversation ---\n");

    let options = ClaudeAgentOptions {
        continue_conversation: true,  // Keep conversation context
        ..Default::default()
    };

    let mut client = ClaudeSDKClient::new(Some(options));
    client.connect().await?;
    println!("✓ Connected with continue_conversation enabled\n");

    // Query 1: Set context
    println!("=== Query 1: Setting Context ===");
    println!("Sending: 'I'm building a web server in Rust using Actix-web.'\n");

    client.query("I'm building a web server in Rust using Actix-web. Just acknowledge this.", None).await?;

    let mut response1 = client.receive_response();
    while let Some(result) = response1.next().await {
        match result {
            Ok(Message::Assistant(msg)) => {
                for block in msg.content {
                    if let ContentBlock::Text { text } = block {
                        println!("Claude: {}\n", text);
                    }
                }
            }
            Ok(Message::Result(r)) => {
                println!("Session ID: {}", r.session_id);
                println!("Turns: {}\n", r.num_turns);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Query 2: Follow-up question (should remember context)
    println!("=== Query 2: Follow-up (Testing Context) ===");
    println!("Sending: 'What web framework am I using?'\n");

    client.query("What web framework am I using?", None).await?;

    let mut response2 = client.receive_response();
    while let Some(result) = response2.next().await {
        match result {
            Ok(Message::Assistant(msg)) => {
                for block in msg.content {
                    if let ContentBlock::Text { text } = block {
                        println!("Claude: {}\n", text);
                    }
                }
            }
            Ok(Message::Result(r)) => {
                println!("Session ID: {}", r.session_id);
                println!("Turns: {}\n", r.num_turns);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Query 3: Another follow-up
    println!("=== Query 3: Another Follow-up ===");
    println!("Sending: 'Show me a simple route handler example.'\n");

    client.query("Show me a simple route handler example for that framework.", None).await?;

    let mut response3 = client.receive_response();
    while let Some(result) = response3.next().await {
        match result {
            Ok(Message::Assistant(msg)) => {
                for block in msg.content {
                    if let ContentBlock::Text { text } = block {
                        println!("Claude: {}\n", text);
                    }
                }
            }
            Ok(Message::Result(r)) => {
                println!("--- Conversation Summary ---");
                println!("Session ID: {}", r.session_id);
                println!("Total Turns: {}", r.num_turns);
                println!("Duration: {}ms", r.duration_ms);
                if let Some(cost) = r.total_cost_usd {
                    println!("Total Cost: ${:.6}", cost);
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

    println!("✓ Continuous conversation complete!");
    println!("\nNote: Claude maintained context across all three queries.");
    println!("Without continue_conversation, each query would be independent.");

    Ok(())
}
