/// Example demonstrating message parsing functionality
///
/// This example shows how to:
/// - Parse different message types from JSON
/// - Handle various content blocks
/// - Work with streaming events
/// - Error handling during parsing

use claude::message_parser::parse_message;
use claude::types::*;
use serde_json::json;

fn main() {
    println!("=== Message Parser Demo ===\n");

    // Example 1: Parse user message with text
    example_user_message_text();

    // Example 2: Parse user message with content blocks
    example_user_message_blocks();

    // Example 3: Parse assistant message with multiple block types
    example_assistant_message();

    // Example 4: Parse system message
    example_system_message();

    // Example 5: Parse result message
    example_result_message();

    // Example 6: Parse stream event
    example_stream_event();

    // Example 7: Error handling
    example_error_handling();
}

fn example_user_message_text() {
    println!("--- Example 1: User Message (Text) ---");

    let json = json!({
        "type": "user",
        "message": {
            "content": "What is 2 + 2?"
        }
    });

    match parse_message(&json) {
        Ok(Message::User(msg)) => {
            println!("✓ Parsed user message successfully");
            match msg.content {
                UserMessageContent::Text(text) => println!("  Content: {}", text),
                _ => println!("  Unexpected content type"),
            }
        }
        Ok(_) => println!("✗ Unexpected message type"),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn example_user_message_blocks() {
    println!("--- Example 2: User Message (Content Blocks) ---");

    let json = json!({
        "type": "user",
        "message": {
            "content": [
                {
                    "type": "text",
                    "text": "Please analyze this code"
                },
                {
                    "type": "tool_result",
                    "tool_use_id": "tool-123",
                    "content": "def hello(): print('Hello')",
                    "is_error": false
                }
            ]
        },
        "parent_tool_use_id": "parent-456"
    });

    match parse_message(&json) {
        Ok(Message::User(msg)) => {
            println!("✓ Parsed user message with blocks");
            println!("  Parent tool use ID: {:?}", msg.parent_tool_use_id);
            match msg.content {
                UserMessageContent::Blocks(blocks) => {
                    println!("  Content blocks: {}", blocks.len());
                    for (i, block) in blocks.iter().enumerate() {
                        match block {
                            ContentBlock::Text { text } => {
                                println!("    Block {}: Text - {}", i, text)
                            }
                            ContentBlock::ToolResult { tool_use_id, .. } => {
                                println!("    Block {}: ToolResult - {}", i, tool_use_id)
                            }
                            _ => println!("    Block {}: Other", i),
                        }
                    }
                }
                _ => println!("  Unexpected content type"),
            }
        }
        Ok(_) => println!("✗ Unexpected message type"),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn example_assistant_message() {
    println!("--- Example 3: Assistant Message (Multiple Block Types) ---");

    let json = json!({
        "type": "assistant",
        "message": {
            "model": "claude-3-sonnet",
            "content": [
                {
                    "type": "thinking",
                    "thinking": "I should use the calculator tool for this",
                    "signature": "sig-abc"
                },
                {
                    "type": "tool_use",
                    "id": "tool-789",
                    "name": "calculator",
                    "input": {
                        "operation": "add",
                        "a": 2,
                        "b": 2
                    }
                },
                {
                    "type": "text",
                    "text": "The answer is 4."
                }
            ]
        }
    });

    match parse_message(&json) {
        Ok(Message::Assistant(msg)) => {
            println!("✓ Parsed assistant message");
            println!("  Model: {}", msg.model);
            println!("  Content blocks: {}", msg.content.len());

            for (i, block) in msg.content.iter().enumerate() {
                match block {
                    ContentBlock::Thinking { thinking, .. } => {
                        println!("    Block {}: Thinking - {}", i, thinking)
                    }
                    ContentBlock::ToolUse { id, name, .. } => {
                        println!("    Block {}: ToolUse - {} ({})", i, name, id)
                    }
                    ContentBlock::Text { text } => {
                        println!("    Block {}: Text - {}", i, text)
                    }
                    _ => println!("    Block {}: Other", i),
                }
            }
        }
        Ok(_) => println!("✗ Unexpected message type"),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn example_system_message() {
    println!("--- Example 4: System Message ---");

    let json = json!({
        "type": "system",
        "subtype": "status",
        "status": "ready",
        "message": "Claude Code is ready"
    });

    match parse_message(&json) {
        Ok(Message::System(msg)) => {
            println!("✓ Parsed system message");
            println!("  Subtype: {}", msg.subtype);
            println!("  Data fields: {}", msg.data.len());
        }
        Ok(_) => println!("✗ Unexpected message type"),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn example_result_message() {
    println!("--- Example 5: Result Message ---");

    let json = json!({
        "type": "result",
        "subtype": "final",
        "duration_ms": 2500,
        "duration_api_ms": 2000,
        "is_error": false,
        "num_turns": 5,
        "session_id": "session-abc123",
        "total_cost_usd": 0.075,
        "usage": {
            "input_tokens": 150,
            "output_tokens": 75
        },
        "result": "Task completed successfully"
    });

    match parse_message(&json) {
        Ok(Message::Result(msg)) => {
            println!("✓ Parsed result message");
            println!("  Subtype: {}", msg.subtype);
            println!("  Duration: {}ms", msg.duration_ms);
            println!("  API Duration: {}ms", msg.duration_api_ms);
            println!("  Turns: {}", msg.num_turns);
            println!("  Session ID: {}", msg.session_id);
            println!("  Cost: ${:.6}", msg.total_cost_usd.unwrap_or(0.0));
            println!("  Is error: {}", msg.is_error);
            if let Some(result) = &msg.result {
                println!("  Result: {}", result);
            }
            if let Some(usage) = &msg.usage {
                println!("  Usage data fields: {}", usage.len());
            }
        }
        Ok(_) => println!("✗ Unexpected message type"),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn example_stream_event() {
    println!("--- Example 6: Stream Event ---");

    let json = json!({
        "type": "stream_event",
        "uuid": "event-xyz789",
        "session_id": "session-abc123",
        "event": {
            "type": "content_block_delta",
            "index": 0,
            "delta": {
                "type": "text_delta",
                "text": "Hello"
            }
        },
        "parent_tool_use_id": "tool-parent"
    });

    match parse_message(&json) {
        Ok(Message::Stream(msg)) => {
            println!("✓ Parsed stream event");
            println!("  UUID: {}", msg.uuid);
            println!("  Session ID: {}", msg.session_id);
            println!("  Parent tool use ID: {:?}", msg.parent_tool_use_id);
            println!("  Event data fields: {}", msg.event.len());
        }
        Ok(_) => println!("✗ Unexpected message type"),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn example_error_handling() {
    println!("--- Example 7: Error Handling ---");

    // Test 1: Invalid data type
    let invalid_type = json!("not an object");
    match parse_message(&invalid_type) {
        Ok(_) => println!("✗ Should have failed for invalid type"),
        Err(e) => println!("✓ Correctly caught error: {}", e),
    }

    // Test 2: Missing type field
    let missing_type = json!({
        "message": "test"
    });
    match parse_message(&missing_type) {
        Ok(_) => println!("✗ Should have failed for missing type"),
        Err(e) => println!("✓ Correctly caught error: {}", e),
    }

    // Test 3: Unknown message type
    let unknown_type = json!({
        "type": "unknown_message_type",
        "data": "test"
    });
    match parse_message(&unknown_type) {
        Ok(_) => println!("✗ Should have failed for unknown type"),
        Err(e) => println!("✓ Correctly caught error: {}", e),
    }

    // Test 4: Missing required field in result message
    let missing_field = json!({
        "type": "result",
        "subtype": "final"
        // Missing duration_ms and other required fields
    });
    match parse_message(&missing_field) {
        Ok(_) => println!("✗ Should have failed for missing required fields"),
        Err(e) => println!("✓ Correctly caught error: {}", e),
    }

    println!();
    println!("=== Demo Complete ===");
}
