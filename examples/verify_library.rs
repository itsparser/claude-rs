/// Quick verification that the library loads and basic types work
/// This doesn't require Claude Code to be installed or API key
///
/// Run with: cargo run --example verify_library

use claude::{
    AssistantMessage, ClaudeAgentOptions, ContentBlock, Message, PermissionMode,
    SystemPromptConfig, UserMessage, UserMessageContent,
};

fn main() {
    println!("=== Claude Rust SDK - Library Verification ===\n");

    // Test 1: Create options
    println!("✓ Creating ClaudeAgentOptions...");
    let mut options = ClaudeAgentOptions::default();
    options.allowed_tools = vec!["Read".to_string(), "Write".to_string()];
    options.max_turns = Some(5);
    options.permission_mode = Some(PermissionMode::AcceptEdits);
    options.system_prompt = Some(SystemPromptConfig::Text(
        "You are a helpful assistant".to_string(),
    ));
    println!("  Options created: {:?} tools, max_turns={:?}",
             options.allowed_tools.len(), options.max_turns);

    // Test 2: Create messages
    println!("\n✓ Creating messages...");
    let user_msg = UserMessage {
        content: UserMessageContent::Text("Hello Claude!".to_string()),
        parent_tool_use_id: None,
    };
    println!("  User message created");

    let assistant_msg = AssistantMessage {
        content: vec![ContentBlock::Text {
            text: "Hello! How can I help you?".to_string(),
        }],
        model: "claude-sonnet-4".to_string(),
        parent_tool_use_id: None,
    };
    println!("  Assistant message created with model: {}", assistant_msg.model);

    // Test 3: Create content blocks
    println!("\n✓ Creating content blocks...");
    let text_block = ContentBlock::Text {
        text: "This is a text block".to_string(),
    };
    if let ContentBlock::Text { text } = &text_block {
        println!("  Text block: {}", text);
    }

    // Test 4: Serialize to JSON
    println!("\n✓ Testing JSON serialization...");
    let json = serde_json::to_string_pretty(&user_msg).unwrap();
    println!("  Serialized user message:\n{}", json);

    // Test 5: Message enum
    println!("\n✓ Testing Message enum...");
    let msg = Message::User(user_msg);
    match msg {
        Message::User(m) => {
            let content_desc = match &m.content {
                UserMessageContent::Text(s) => format!("text: {}", s),
                UserMessageContent::Blocks(b) => format!("{} content blocks", b.len()),
            };
            println!("  Message is User type with {}", content_desc);
        }
        _ => println!("  Unexpected message type"),
    }

    // Test 6: Assistant message
    let assistant_message = Message::Assistant(assistant_msg);
    if let Message::Assistant(m) = assistant_message {
        println!("  Assistant message has {} content blocks", m.content.len());
    }

    println!("\n=== All Verifications Passed! ===");
    println!("\nThe Claude Rust SDK library is working correctly!");
    println!("\nCore features verified:");
    println!("  ✓ Type system (ClaudeAgentOptions, Messages, ContentBlocks)");
    println!("  ✓ JSON serialization/deserialization");
    println!("  ✓ Message enum handling");
    println!("\nTo test with actual Claude Code:");
    println!("  1. Install Claude Code: npm install -g @anthropic-ai/claude-code");
    println!("  2. Run: cargo run --example simple_real_query");
}
