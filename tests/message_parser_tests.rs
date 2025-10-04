use claude::message_parser::parse_message;
use claude::types::*;
use serde_json::json;

#[test]
fn test_parse_user_message_text() {
    let data = json!({
        "type": "user",
        "message": {
            "content": "Hello, Claude!"
        }
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::User(msg) => match msg.content {
            UserMessageContent::Text(text) => assert_eq!(text, "Hello, Claude!"),
            _ => panic!("Expected text content"),
        },
        _ => panic!("Expected user message"),
    }
}

#[test]
fn test_parse_user_message_with_blocks() {
    let data = json!({
        "type": "user",
        "message": {
            "content": [
                {
                    "type": "text",
                    "text": "Test message"
                }
            ]
        },
        "parent_tool_use_id": "parent123"
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::User(msg) => {
            assert_eq!(msg.parent_tool_use_id, Some("parent123".to_string()));
            match msg.content {
                UserMessageContent::Blocks(blocks) => {
                    assert_eq!(blocks.len(), 1);
                }
                _ => panic!("Expected blocks content"),
            }
        }
        _ => panic!("Expected user message"),
    }
}

#[test]
fn test_parse_assistant_message() {
    let data = json!({
        "type": "assistant",
        "message": {
            "model": "claude-3-sonnet",
            "content": [
                {
                    "type": "text",
                    "text": "Hello!"
                }
            ]
        }
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::Assistant(msg) => {
            assert_eq!(msg.model, "claude-3-sonnet");
            assert_eq!(msg.content.len(), 1);
        }
        _ => panic!("Expected assistant message"),
    }
}

#[test]
fn test_parse_assistant_message_with_thinking() {
    let data = json!({
        "type": "assistant",
        "message": {
            "model": "claude-3-opus",
            "content": [
                {
                    "type": "thinking",
                    "thinking": "Let me think...",
                    "signature": "sig123"
                },
                {
                    "type": "text",
                    "text": "Here's my answer"
                }
            ]
        }
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::Assistant(msg) => {
            assert_eq!(msg.content.len(), 2);
            match &msg.content[0] {
                ContentBlock::Thinking { thinking, signature } => {
                    assert_eq!(thinking, "Let me think...");
                    assert_eq!(signature, "sig123");
                }
                _ => panic!("Expected thinking block"),
            }
        }
        _ => panic!("Expected assistant message"),
    }
}

#[test]
fn test_parse_assistant_message_with_tool_use() {
    let data = json!({
        "type": "assistant",
        "message": {
            "model": "claude-3-sonnet",
            "content": [
                {
                    "type": "tool_use",
                    "id": "tool123",
                    "name": "bash",
                    "input": {
                        "command": "ls -la"
                    }
                }
            ]
        }
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::Assistant(msg) => {
            assert_eq!(msg.content.len(), 1);
            match &msg.content[0] {
                ContentBlock::ToolUse { id, name, input } => {
                    assert_eq!(id, "tool123");
                    assert_eq!(name, "bash");
                    assert!(input.contains_key("command"));
                }
                _ => panic!("Expected tool use block"),
            }
        }
        _ => panic!("Expected assistant message"),
    }
}

#[test]
fn test_parse_system_message() {
    let data = json!({
        "type": "system",
        "subtype": "status",
        "message": "System ready"
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::System(msg) => {
            assert_eq!(msg.subtype, "status");
            assert!(msg.data.contains_key("subtype"));
        }
        _ => panic!("Expected system message"),
    }
}

#[test]
fn test_parse_result_message() {
    let data = json!({
        "type": "result",
        "subtype": "final",
        "duration_ms": 1500,
        "duration_api_ms": 1200,
        "is_error": false,
        "num_turns": 3,
        "session_id": "session123",
        "total_cost_usd": 0.05,
        "result": "Success"
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::Result(msg) => {
            assert_eq!(msg.subtype, "final");
            assert_eq!(msg.duration_ms, 1500);
            assert_eq!(msg.duration_api_ms, 1200);
            assert!(!msg.is_error);
            assert_eq!(msg.num_turns, 3);
            assert_eq!(msg.session_id, "session123");
            assert_eq!(msg.total_cost_usd, Some(0.05));
            assert_eq!(msg.result, Some("Success".to_string()));
        }
        _ => panic!("Expected result message"),
    }
}

#[test]
fn test_parse_result_message_with_usage() {
    let data = json!({
        "type": "result",
        "subtype": "final",
        "duration_ms": 1000,
        "duration_api_ms": 800,
        "is_error": false,
        "num_turns": 1,
        "session_id": "session456",
        "usage": {
            "input_tokens": 100,
            "output_tokens": 50
        }
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::Result(msg) => {
            assert!(msg.usage.is_some());
            let usage = msg.usage.unwrap();
            assert!(usage.contains_key("input_tokens"));
            assert!(usage.contains_key("output_tokens"));
        }
        _ => panic!("Expected result message"),
    }
}

#[test]
fn test_parse_stream_event() {
    let data = json!({
        "type": "stream_event",
        "uuid": "event123",
        "session_id": "session789",
        "event": {
            "type": "content_delta",
            "delta": "partial text"
        }
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::Stream(msg) => {
            assert_eq!(msg.uuid, "event123");
            assert_eq!(msg.session_id, "session789");
            assert!(msg.event.contains_key("type"));
        }
        _ => panic!("Expected stream event"),
    }
}

#[test]
fn test_parse_stream_event_with_parent() {
    let data = json!({
        "type": "stream_event",
        "uuid": "event456",
        "session_id": "session123",
        "event": {
            "type": "delta"
        },
        "parent_tool_use_id": "tool999"
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::Stream(msg) => {
            assert_eq!(msg.parent_tool_use_id, Some("tool999".to_string()));
        }
        _ => panic!("Expected stream event"),
    }
}

#[test]
fn test_parse_tool_result_block() {
    let data = json!({
        "type": "assistant",
        "message": {
            "model": "claude-3-sonnet",
            "content": [
                {
                    "type": "tool_result",
                    "tool_use_id": "tool123",
                    "content": "Result content",
                    "is_error": false
                }
            ]
        }
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::Assistant(msg) => {
            match &msg.content[0] {
                ContentBlock::ToolResult { tool_use_id, is_error, .. } => {
                    assert_eq!(tool_use_id, "tool123");
                    assert_eq!(*is_error, Some(false));
                }
                _ => panic!("Expected tool result block"),
            }
        }
        _ => panic!("Expected assistant message"),
    }
}

#[test]
fn test_parse_missing_required_field() {
    let data = json!({
        "type": "result",
        "subtype": "final"
        // Missing required fields like duration_ms, etc.
    });

    let result = parse_message(&data);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_content_block_type() {
    let data = json!({
        "type": "assistant",
        "message": {
            "model": "claude-3-sonnet",
            "content": [
                {
                    "type": "unknown_block_type",
                    "data": "test"
                }
            ]
        }
    });

    let result = parse_message(&data);
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_data_type() {
    let data = json!("not an object");
    let result = parse_message(&data);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_type_field() {
    let data = json!({
        "message": "test"
    });
    let result = parse_message(&data);
    assert!(result.is_err());
}

#[test]
fn test_parse_unknown_message_type() {
    let data = json!({
        "type": "unknown_type",
        "data": "test"
    });
    let result = parse_message(&data);
    assert!(result.is_err());
}

#[test]
fn test_parse_mixed_content_blocks() {
    let data = json!({
        "type": "assistant",
        "message": {
            "model": "claude-3-opus",
            "content": [
                {
                    "type": "text",
                    "text": "First"
                },
                {
                    "type": "tool_use",
                    "id": "t1",
                    "name": "bash",
                    "input": {}
                },
                {
                    "type": "thinking",
                    "thinking": "Hmm",
                    "signature": "s1"
                },
                {
                    "type": "text",
                    "text": "Last"
                }
            ]
        }
    });

    let result = parse_message(&data).unwrap();
    match result {
        Message::Assistant(msg) => {
            assert_eq!(msg.content.len(), 4);
        }
        _ => panic!("Expected assistant message"),
    }
}
