/// Example: MessageVecExt - Extension Trait Methods
///
/// Demonstrates all the helper methods provided by the MessageVecExt trait
/// for working with message vectors without verbose pattern matching.

use claude::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== MessageVecExt Extension Trait Example ===\n");

    // For demonstration, we'll create mock messages
    // In real usage, these would come from simple_query() or streaming_query()

    let messages = create_mock_messages();

    // Example 1: text_content() - Extract all text
    println!("1. text_content() - Get All Text:");
    let text = messages.text_content();
    println!("   Full text content:");
    println!("   {}\n", text);

    // Example 2: assistant_messages() - Get all assistant messages
    println!("2. assistant_messages() - Filter Assistant Messages:");
    let assistants = messages.assistant_messages();
    println!("   Found {} assistant messages", assistants.len());
    for (i, msg) in assistants.iter().enumerate() {
        println!("   Message {}: Model = {}", i + 1, msg.model);
    }
    println!();

    // Example 3: last_assistant() - Get most recent assistant message
    println!("3. last_assistant() - Get Last Response:");
    if let Some(last) = messages.last_assistant() {
        println!("   Last assistant message:");
        println!("   Model: {}", last.model);
        println!("   Content blocks: {}", last.content.len());
    }
    println!();

    // Example 4: result_message() - Get result metadata
    println!("4. result_message() - Get Result Metadata:");
    if let Some(result) = messages.result_message() {
        println!("   Result found:");
        println!("   Duration: {}ms", result.duration_ms);
        println!("   Turns: {}", result.num_turns);
        println!("   Session ID: {}", result.session_id);
    } else {
        println!("   No result message in this example");
    }
    println!();

    // Example 5: has_assistant_messages() - Quick check
    println!("5. has_assistant_messages() - Quick Check:");
    if messages.has_assistant_messages() {
        println!("   ✓ Yes, we have assistant responses!");
    } else {
        println!("   ✗ No assistant messages found");
    }
    println!();

    // Example 6: text_blocks() - Get individual text blocks
    println!("6. text_blocks() - Get Text Blocks:");
    let blocks = messages.text_blocks();
    println!("   Found {} text blocks:", blocks.len());
    for (i, block) in blocks.iter().enumerate() {
        let preview = block.chars().take(50).collect::<String>();
        println!("   Block {}: {}", i + 1, preview);
    }
    println!();

    // Example 7: Individual Message methods
    println!("7. Individual Message Methods:");
    if let Some(msg) = messages.first() {
        println!("   is_assistant(): {}", msg.is_assistant());
        println!("   is_user(): {}", msg.is_user());
        println!("   is_result(): {}", msg.is_result());

        if let Some(text) = msg.text_content() {
            println!("   text_content(): {}", text.chars().take(50).collect::<String>());
        }
    }
    println!();

    // Example 8: Before/After Comparison
    println!("8. Before/After Comparison:");
    println!("\n   BEFORE (V1) - Manual iteration:");
    println!("   ```rust");
    println!("   let mut text = String::new();");
    println!("   for msg in &messages {{");
    println!("       if let Message::Assistant(assistant) = msg {{");
    println!("           for content in &assistant.content {{");
    println!("               if let ContentBlock::Text(text_block) = content {{");
    println!("                   text.push_str(&text_block.text);");
    println!("               }}");
    println!("           }}");
    println!("       }}");
    println!("   }}");
    println!("   ```\n");

    println!("   AFTER (V2) - One method call:");
    println!("   ```rust");
    println!("   let text = messages.text_content();");
    println!("   ```\n");

    println!("=== Example Complete ===");
    println!("\nMessageVecExt Methods:");
    println!("• text_content() - Extract all text as String");
    println!("• assistant_messages() - Get Vec of assistant messages");
    println!("• last_assistant() - Get most recent assistant message");
    println!("• result_message() - Get result metadata");
    println!("• has_assistant_messages() - Quick boolean check");
    println!("• text_blocks() - Get Vec of text strings");
    println!("\nMessage Methods:");
    println!("• is_assistant/is_user/is_result() - Type checks");
    println!("• as_assistant/as_result() - Safe casting");
    println!("• text_content() - Extract text from single message");

    Ok(())
}

// Helper function to create mock messages for demonstration
fn create_mock_messages() -> Vec<Message> {
    use claude::types::{AssistantMessage, ContentBlock};

    vec![
        Message::Assistant(AssistantMessage {
            content: vec![
                ContentBlock::Text {
                    text: "Hello! I'm Claude, an AI assistant.".to_string(),
                },
                ContentBlock::Text {
                    text: "I can help you with various tasks.".to_string(),
                },
            ],
            model: "claude-sonnet-4-5".to_string(),
            parent_tool_use_id: None,
        }),
        Message::Assistant(AssistantMessage {
            content: vec![ContentBlock::Text {
                text: "Is there anything specific you'd like help with?".to_string(),
            }],
            model: "claude-sonnet-4-5".to_string(),
            parent_tool_use_id: None,
        }),
    ]
}
