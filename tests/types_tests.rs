use claude::types::*;
use serde_json;
use std::collections::HashMap;

#[test]
fn test_permission_mode_serialization() {
    let mode = PermissionMode::Default;
    let json = serde_json::to_string(&mode).unwrap();
    assert_eq!(json, "\"default\"");

    let mode = PermissionMode::AcceptEdits;
    let json = serde_json::to_string(&mode).unwrap();
    assert_eq!(json, "\"acceptEdits\"");
}

#[test]
fn test_permission_mode_deserialization() {
    let json = "\"default\"";
    let mode: PermissionMode = serde_json::from_str(json).unwrap();
    assert_eq!(mode, PermissionMode::Default);

    let json = "\"acceptEdits\"";
    let mode: PermissionMode = serde_json::from_str(json).unwrap();
    assert_eq!(mode, PermissionMode::AcceptEdits);
}

#[test]
fn test_setting_source_serialization() {
    let source = SettingSource::User;
    let json = serde_json::to_string(&source).unwrap();
    assert_eq!(json, "\"user\"");

    let source = SettingSource::Project;
    let json = serde_json::to_string(&source).unwrap();
    assert_eq!(json, "\"project\"");
}

#[test]
fn test_permission_behavior_serialization() {
    let behavior = PermissionBehavior::Allow;
    let json = serde_json::to_string(&behavior).unwrap();
    assert_eq!(json, "\"allow\"");

    let behavior = PermissionBehavior::Deny;
    let json = serde_json::to_string(&behavior).unwrap();
    assert_eq!(json, "\"deny\"");
}

#[test]
fn test_agent_definition() {
    let agent = AgentDefinition {
        description: "Test agent".to_string(),
        prompt: "Test prompt".to_string(),
        tools: Some(vec!["tool1".to_string(), "tool2".to_string()]),
        model: Some("sonnet".to_string()),
    };

    let json = serde_json::to_string(&agent).unwrap();
    let deserialized: AgentDefinition = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.description, "Test agent");
    assert_eq!(deserialized.prompt, "Test prompt");
    assert_eq!(deserialized.tools, Some(vec!["tool1".to_string(), "tool2".to_string()]));
    assert_eq!(deserialized.model, Some("sonnet".to_string()));
}

#[test]
fn test_permission_rule_value() {
    let rule = PermissionRuleValue {
        tool_name: "Bash".to_string(),
        rule_content: Some("allow all".to_string()),
    };

    let json = serde_json::to_string(&rule).unwrap();
    let deserialized: PermissionRuleValue = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.tool_name, "Bash");
    assert_eq!(deserialized.rule_content, Some("allow all".to_string()));
}

#[test]
fn test_permission_result_allow() {
    let result = PermissionResult::Allow {
        updated_input: None,
        updated_permissions: None,
    };

    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("\"behavior\":\"allow\""));
}

#[test]
fn test_permission_result_deny() {
    let result = PermissionResult::Deny {
        message: "Access denied".to_string(),
        interrupt: true,
    };

    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("\"behavior\":\"deny\""));
    assert!(json.contains("\"message\":\"Access denied\""));
    assert!(json.contains("\"interrupt\":true"));
}

#[test]
fn test_content_block_text() {
    let block = ContentBlock::Text {
        text: "Hello, world!".to_string(),
    };

    let json = serde_json::to_string(&block).unwrap();
    assert!(json.contains("\"type\":\"text\""));
    assert!(json.contains("\"text\":\"Hello, world!\""));
}

#[test]
fn test_content_block_tool_use() {
    let mut input = HashMap::new();
    input.insert("param1".to_string(), serde_json::json!("value1"));

    let block = ContentBlock::ToolUse {
        id: "tool123".to_string(),
        name: "test_tool".to_string(),
        input,
    };

    let json = serde_json::to_string(&block).unwrap();
    assert!(json.contains("\"type\":\"tool_use\""));
    assert!(json.contains("\"id\":\"tool123\""));
    assert!(json.contains("\"name\":\"test_tool\""));
}

#[test]
fn test_user_message_text() {
    let msg = UserMessage {
        content: UserMessageContent::Text("Hello".to_string()),
        parent_tool_use_id: None,
    };

    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"content\":\"Hello\""));
}

#[test]
fn test_user_message_blocks() {
    let blocks = vec![ContentBlock::Text {
        text: "Test".to_string(),
    }];

    let msg = UserMessage {
        content: UserMessageContent::Blocks(blocks),
        parent_tool_use_id: Some("parent123".to_string()),
    };

    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"parent_tool_use_id\":\"parent123\""));
}

#[test]
fn test_assistant_message() {
    let content = vec![ContentBlock::Text {
        text: "Response".to_string(),
    }];

    let msg = AssistantMessage {
        content,
        model: "claude-3-sonnet".to_string(),
        parent_tool_use_id: None,
    };

    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"model\":\"claude-3-sonnet\""));
}

#[test]
fn test_result_message() {
    let msg = ResultMessage {
        subtype: "final".to_string(),
        duration_ms: 1000,
        duration_api_ms: 800,
        is_error: false,
        num_turns: 3,
        session_id: "session123".to_string(),
        total_cost_usd: Some(0.05),
        usage: None,
        result: Some("Success".to_string()),
    };

    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: ResultMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.duration_ms, 1000);
    assert_eq!(deserialized.num_turns, 3);
    assert_eq!(deserialized.session_id, "session123");
}

#[test]
fn test_mcp_server_config_stdio() {
    let config = McpServerConfig::Stdio {
        command: "node".to_string(),
        args: Some(vec!["server.js".to_string()]),
        env: None,
    };

    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"type\":\"stdio\""));
    assert!(json.contains("\"command\":\"node\""));
}

#[test]
fn test_mcp_server_config_sse() {
    let config = McpServerConfig::SSE {
        url: "https://example.com/sse".to_string(),
        headers: None,
    };

    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"type\":\"sse\""));
    assert!(json.contains("\"url\":\"https://example.com/sse\""));
}

#[test]
fn test_hook_event_serialization() {
    let event = HookEvent::PreToolUse;
    let json = serde_json::to_string(&event).unwrap();
    assert_eq!(json, "\"PreToolUse\"");
}

#[test]
fn test_control_request_interrupt() {
    let request = ControlRequest::Interrupt {};
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"subtype\":\"interrupt\""));
}

#[test]
fn test_control_request_can_use_tool() {
    let mut input = HashMap::new();
    input.insert("param".to_string(), serde_json::json!("value"));

    let request = ControlRequest::CanUseTool {
        tool_name: "Bash".to_string(),
        input,
        permission_suggestions: None,
        blocked_path: None,
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"subtype\":\"can_use_tool\""));
    assert!(json.contains("\"tool_name\":\"Bash\""));
}

#[test]
fn test_control_response_success() {
    let response = ControlResponseType::Success {
        request_id: "req123".to_string(),
        response: None,
    };

    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("\"subtype\":\"success\""));
    assert!(json.contains("\"request_id\":\"req123\""));
}

#[test]
fn test_control_response_error() {
    let response = ControlResponseType::Error {
        request_id: "req123".to_string(),
        error: "Something went wrong".to_string(),
    };

    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("\"subtype\":\"error\""));
    assert!(json.contains("\"error\":\"Something went wrong\""));
}

#[test]
fn test_claude_agent_options_default() {
    let options = ClaudeAgentOptions::default();
    assert!(options.allowed_tools.is_empty());
    assert!(!options.continue_conversation);
    assert!(!options.include_partial_messages);
    assert!(!options.fork_session);
}

#[test]
fn test_hook_json_output() {
    let output = HookJSONOutput {
        decision: Some("block".to_string()),
        system_message: Some("Blocked by hook".to_string()),
        hook_specific_output: None,
    };

    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("\"decision\":\"block\""));
    assert!(json.contains("\"system_message\":\"Blocked by hook\""));
}
