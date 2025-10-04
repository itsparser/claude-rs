/// Example demonstrating Claude SDK types
///
/// This example shows how to work with:
/// - Message types (User, Assistant, System, Result)
/// - Content blocks (Text, ToolUse, ToolResult, Thinking)
/// - ClaudeAgentOptions configuration
/// - Permission types
/// - Serialization/deserialization

use claude::types::*;
use serde_json;
use std::collections::HashMap;

fn main() {
    println!("=== Claude SDK Types Demo ===\n");

    // Example 1: Creating and using ClaudeAgentOptions
    example_agent_options();

    // Example 2: Working with messages
    example_messages();

    // Example 3: Content blocks
    example_content_blocks();

    // Example 4: Permission types
    example_permissions();

    // Example 5: MCP server configuration
    example_mcp_config();

    // Example 6: Serialization/Deserialization
    example_serialization();
}

fn example_agent_options() {
    println!("--- Example 1: ClaudeAgentOptions ---");

    let mut options = ClaudeAgentOptions::default();
    options.allowed_tools = vec!["Read".to_string(), "Write".to_string(), "Bash".to_string()];
    options.max_turns = Some(5);
    options.permission_mode = Some(PermissionMode::AcceptEdits);
    options.model = Some("claude-3-sonnet".to_string());

    println!("Created options with:");
    println!("  - Allowed tools: {:?}", options.allowed_tools);
    println!("  - Max turns: {:?}", options.max_turns);
    println!("  - Permission mode: {:?}", options.permission_mode);
    println!("  - Model: {:?}", options.model);
    println!();
}

fn example_messages() {
    println!("--- Example 2: Working with Messages ---");

    // Create a user message
    let user_msg = UserMessage {
        content: UserMessageContent::Text("Hello, Claude!".to_string()),
        parent_tool_use_id: None,
    };
    println!("User message: {:?}", user_msg);

    // Create an assistant message with multiple content blocks
    let assistant_msg = AssistantMessage {
        content: vec![
            ContentBlock::Text {
                text: "Hello! How can I help you today?".to_string(),
            },
        ],
        model: "claude-3-sonnet".to_string(),
        parent_tool_use_id: None,
    };
    println!("Assistant message: {:?}", assistant_msg);

    // Create a result message
    let result_msg = ResultMessage {
        subtype: "final".to_string(),
        duration_ms: 1500,
        duration_api_ms: 1200,
        is_error: false,
        num_turns: 3,
        session_id: "session-123".to_string(),
        total_cost_usd: Some(0.05),
        usage: None,
        result: Some("Success".to_string()),
    };
    println!("Result message: {:?}", result_msg);
    println!();
}

fn example_content_blocks() {
    println!("--- Example 3: Content Blocks ---");

    // Text block
    let text_block = ContentBlock::Text {
        text: "This is a text response".to_string(),
    };
    println!("Text block: {:?}", text_block);

    // Thinking block
    let thinking_block = ContentBlock::Thinking {
        thinking: "Let me analyze this problem...".to_string(),
        signature: "sig-abc123".to_string(),
    };
    println!("Thinking block: {:?}", thinking_block);

    // Tool use block
    let mut tool_input = HashMap::new();
    tool_input.insert("command".to_string(), serde_json::json!("ls -la"));

    let tool_use_block = ContentBlock::ToolUse {
        id: "tool-use-123".to_string(),
        name: "Bash".to_string(),
        input: tool_input,
    };
    println!("Tool use block: {:?}", tool_use_block);

    // Tool result block
    let tool_result_block = ContentBlock::ToolResult {
        tool_use_id: "tool-use-123".to_string(),
        content: Some(serde_json::json!("total 48\ndrwxr-xr-x  8 user  staff   256 Oct  4 01:18 .")),
        is_error: Some(false),
    };
    println!("Tool result block: {:?}", tool_result_block);
    println!();
}

fn example_permissions() {
    println!("--- Example 4: Permission Types ---");

    // Permission modes
    let modes = vec![
        PermissionMode::Default,
        PermissionMode::AcceptEdits,
        PermissionMode::Plan,
        PermissionMode::BypassPermissions,
    ];
    println!("Permission modes: {:?}", modes);

    // Permission behaviors
    let behaviors = vec![
        PermissionBehavior::Allow,
        PermissionBehavior::Deny,
        PermissionBehavior::Ask,
    ];
    println!("Permission behaviors: {:?}", behaviors);

    // Permission result - Allow
    let allow_result = PermissionResult::Allow {
        updated_input: None,
        updated_permissions: None,
    };
    println!("Allow result: {:?}", allow_result);

    // Permission result - Deny
    let deny_result = PermissionResult::Deny {
        message: "Access denied: unsafe operation".to_string(),
        interrupt: true,
    };
    println!("Deny result: {:?}", deny_result);

    // Permission update
    let permission_update = PermissionUpdate {
        r#type: "addRules".to_string(),
        rules: Some(vec![PermissionRuleValue {
            tool_name: "Bash".to_string(),
            rule_content: Some("allow all".to_string()),
        }]),
        behavior: Some(PermissionBehavior::Allow),
        mode: None,
        directories: None,
        destination: Some(PermissionUpdateDestination::Session),
    };
    println!("Permission update: {:?}", permission_update);
    println!();
}

fn example_mcp_config() {
    println!("--- Example 5: MCP Server Configuration ---");

    // Stdio server config
    let stdio_config = McpServerConfig::Stdio {
        command: "node".to_string(),
        args: Some(vec!["server.js".to_string()]),
        env: None,
    };
    println!("Stdio server: {:?}", stdio_config);

    // SSE server config
    let sse_config = McpServerConfig::SSE {
        url: "https://example.com/sse".to_string(),
        headers: None,
    };
    println!("SSE server: {:?}", sse_config);

    // HTTP server config
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token123".to_string());

    let http_config = McpServerConfig::Http {
        url: "https://api.example.com".to_string(),
        headers: Some(headers),
    };
    println!("HTTP server: {:?}", http_config);
    println!();
}

fn example_serialization() {
    println!("--- Example 6: Serialization/Deserialization ---");

    // Create a complex message
    let assistant_msg = AssistantMessage {
        content: vec![
            ContentBlock::Thinking {
                thinking: "I need to read the file first".to_string(),
                signature: "sig-001".to_string(),
            },
            ContentBlock::ToolUse {
                id: "tool-123".to_string(),
                name: "Read".to_string(),
                input: {
                    let mut map = HashMap::new();
                    map.insert("file_path".to_string(), serde_json::json!("/tmp/test.txt"));
                    map
                },
            },
            ContentBlock::Text {
                text: "I've read the file contents.".to_string(),
            },
        ],
        model: "claude-3-sonnet".to_string(),
        parent_tool_use_id: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&assistant_msg).expect("Failed to serialize");
    println!("Serialized assistant message:");
    println!("{}", json);
    println!();

    // Deserialize back
    let deserialized: AssistantMessage =
        serde_json::from_str(&json).expect("Failed to deserialize");
    println!("Deserialized successfully!");
    println!("Model: {}", deserialized.model);
    println!("Content blocks: {}", deserialized.content.len());
    println!();

    // Serialize ClaudeAgentOptions
    let mut options = ClaudeAgentOptions::default();
    options.allowed_tools = vec!["Read".to_string(), "Write".to_string()];
    options.max_turns = Some(3);

    println!("ClaudeAgentOptions example:");
    println!("  Allowed tools: {:?}", options.allowed_tools);
    println!("  Max turns: {:?}", options.max_turns);
    println!();

    println!("=== Demo Complete ===");
}
