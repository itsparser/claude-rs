/// Example: Using MessageVecExt trait for easier message handling
///
/// This demonstrates the new extension methods that make it easier
/// to work with message vectors without verbose pattern matching.

use claude::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== V2 Message Extensions Example ===\n");

    // Get full messages for more control
    let messages = simple_query("Explain async/await in Rust briefly", None).await?;

    println!("1. Using text_content() extension:");
    let text = messages.text_content();
    println!("   {}\n", text);

    println!("2. Using assistant_messages() extension:");
    let assistants = messages.assistant_messages();
    println!("   Found {} assistant messages\n", assistants.len());

    println!("3. Using last_assistant() extension:");
    if let Some(last) = messages.last_assistant() {
        println!("   Last message from: {}\n", last.model);
    }

    println!("4. Using has_assistant_messages() check:");
    if messages.has_assistant_messages() {
        println!("   Yes, we have assistant responses!\n");
    }

    println!("5. Using text_blocks() extension:");
    let blocks = messages.text_blocks();
    println!("   Found {} text blocks:", blocks.len());
    for (i, block) in blocks.iter().enumerate() {
        println!("   Block {}: {}", i + 1, block.chars().take(50).collect::<String>());
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
