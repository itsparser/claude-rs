use claude::{ClaudeSDKClient, ClaudeAgentOptions, Message, ContentBlock};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("=== Test Interactive Chat ===\n");

    let options = ClaudeAgentOptions {
        max_turns: Some(2),
        ..Default::default()
    };

    eprintln!("Creating client...");
    let mut client = ClaudeSDKClient::new(Some(options));

    eprintln!("Connecting...");
    client.connect().await?;
    eprintln!("Connected!\n");

    eprintln!("Sending query: 'What is 2 + 2?'");
    client.query("What is 2 + 2?", None).await?;
    eprintln!("Query sent!");

    eprintln!("Waiting for messages...");
    let mut response = client.receive_response();
    let mut msg_count = 0;

    while let Some(result) = response.next().await {
        msg_count += 1;
        eprintln!("Received message #{}", msg_count);

        match result {
            Ok(Message::Assistant(msg)) => {
                for block in msg.content {
                    if let ContentBlock::Text { text } = block {
                        println!("Claude: {}", text);
                    }
                }
            }
            Ok(Message::Result(r)) => {
                println!("Done! Cost: ${:.6}", r.total_cost_usd.unwrap_or(0.0));
            }
            Ok(msg) => {
                eprintln!("Other message: {:?}", msg);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    eprintln!("\nClosing...");
    client.close().await?;
    eprintln!("Closed!");

    Ok(())
}
