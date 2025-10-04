use claude::{ClaudeSDKClient, CanUseToolCallback, Message, ContentBlock, PermissionResult};
use futures::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== can_use_tool Permission Callback Example ===\n");
    println!("This example demonstrates runtime permission control over tool usage.");
    println!("The callback will deny 'Bash' tool usage but allow other tools.\n");

    // Create permission callback
    let can_use_tool: CanUseToolCallback = Arc::new(|tool_name, input, _context| {
        Box::pin(async move {
            println!("\n🔒 Permission check for tool: {}", tool_name);
            println!("   Input: {:?}", input);

            match tool_name.as_str() {
                "Bash" => {
                    println!("   ❌ DENIED: Bash operations not allowed in this session");
                    Ok(PermissionResult::deny(
                        "Bash operations are not allowed for security reasons".to_string()
                    ))
                }
                "Write" => {
                    println!("   ⚠️  DENIED: Write operations not allowed");
                    Ok(PermissionResult::deny(
                        "Write operations are restricted".to_string()
                    ))
                }
                _ => {
                    println!("   ✅ ALLOWED: Tool approved");
                    Ok(PermissionResult::allow())
                }
            }
        })
    });

    // Create client with permission callback
    let mut client = ClaudeSDKClient::with_can_use_tool(None, can_use_tool);

    println!("Connecting to Claude Code...");
    client.connect().await?;
    println!("✓ Connected!\n");

    // Send a query that will trigger tool usage
    let query = "Read the file /etc/hosts and then try to write to /tmp/test.txt";
    println!("Query: {}\n", query);

    client.query(query, None).await?;

    // Receive and display the response
    let mut response = client.receive_response();
    let mut tool_uses = 0;
    let mut tool_results = 0;

    while let Some(result) = response.next().await {
        match result {
            Ok(Message::Assistant(msg)) => {
                for block in msg.content {
                    match block {
                        ContentBlock::Text { text } => {
                            println!("\n💬 Claude: {}", text);
                        }
                        ContentBlock::ToolUse { name, input, .. } => {
                            tool_uses += 1;
                            println!("\n🔧 Tool Use Request: {}", name);
                            println!("   Input: {:?}", input);
                        }
                        ContentBlock::Thinking { thinking, .. } => {
                            println!("\n🤔 Claude's thinking: {}", thinking);
                        }
                        ContentBlock::ToolResult { tool_use_id, content, is_error } => {
                            tool_results += 1;
                            println!("\n📊 Tool Result (ID: {})", tool_use_id);
                            if is_error.unwrap_or(false) {
                                println!("   ❌ Error: {:?}", content);
                            } else {
                                println!("   ✓ Success: {:?}", content);
                            }
                        }
                    }
                }
            }
            Ok(Message::Result(r)) => {
                println!("\n--- Session Summary ---");
                println!("Session ID: {}", r.session_id);
                println!("Duration: {}ms", r.duration_ms);
                println!("Turns: {}", r.num_turns);
                if let Some(cost) = r.total_cost_usd {
                    println!("Cost: ${:.6}", cost);
                }
                println!("Tool uses: {}", tool_uses);
                println!("Tool results: {}", tool_results);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Clean up
    client.close().await?;
    println!("\n✓ Connection closed");

    Ok(())
}
