use crate::errors::{ClaudeSDKError, Result};
use crate::types::*;
use serde_json::Value;
use std::collections::HashMap;

/// Parse message from CLI output into typed Message objects
///
/// # Arguments
/// * `data` - Raw message dictionary from CLI output
///
/// # Returns
/// Parsed Message object
///
/// # Errors
/// Returns `ClaudeSDKError::MessageParseError` if parsing fails or message type is unrecognized
pub fn parse_message(data: &Value) -> Result<Message> {
    if !data.is_object() {
        return Err(ClaudeSDKError::message_parse_error(
            format!(
                "Invalid message data type (expected object, got {})",
                match data {
                    Value::Null => "null",
                    Value::Bool(_) => "bool",
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::Array(_) => "array",
                    Value::Object(_) => "object",
                }
            ),
            Some(data.clone()),
        ));
    }

    let obj = data.as_object().unwrap();
    let message_type = obj
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ClaudeSDKError::message_parse_error("Message missing 'type' field", Some(data.clone())))?;

    match message_type {
        "user" => parse_user_message(obj, data),
        "assistant" => parse_assistant_message(obj, data),
        "system" => parse_system_message(obj, data),
        "result" => parse_result_message(obj, data),
        "stream_event" => parse_stream_event(obj, data),
        _ => Err(ClaudeSDKError::message_parse_error(
            format!("Unknown message type: {}", message_type),
            Some(data.clone()),
        )),
    }
}

fn parse_user_message(obj: &serde_json::Map<String, Value>, data: &Value) -> Result<Message> {
    let message = obj
        .get("message")
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in user message: message",
                Some(data.clone()),
            )
        })?
        .as_object()
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Invalid message field type",
                Some(data.clone()),
            )
        })?;

    let parent_tool_use_id = obj.get("parent_tool_use_id").and_then(|v| v.as_str()).map(String::from);

    let content_val = message.get("content").ok_or_else(|| {
        ClaudeSDKError::message_parse_error(
            "Missing required field in user message: content",
            Some(data.clone()),
        )
    })?;

    let content = if let Some(arr) = content_val.as_array() {
        let mut blocks = Vec::new();
        for block in arr {
            blocks.push(parse_content_block(block, data)?);
        }
        UserMessageContent::Blocks(blocks)
    } else if let Some(text) = content_val.as_str() {
        UserMessageContent::Text(text.to_string())
    } else {
        return Err(ClaudeSDKError::message_parse_error(
            "Invalid content type in user message",
            Some(data.clone()),
        ));
    };

    Ok(Message::User(UserMessage {
        content,
        parent_tool_use_id,
    }))
}

fn parse_assistant_message(obj: &serde_json::Map<String, Value>, data: &Value) -> Result<Message> {
    let message = obj
        .get("message")
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in assistant message: message",
                Some(data.clone()),
            )
        })?
        .as_object()
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Invalid message field type",
                Some(data.clone()),
            )
        })?;

    let content_arr = message
        .get("content")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing or invalid content field in assistant message",
                Some(data.clone()),
            )
        })?;

    let mut content_blocks = Vec::new();
    for block in content_arr {
        content_blocks.push(parse_content_block(block, data)?);
    }

    let model = message
        .get("model")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in assistant message: model",
                Some(data.clone()),
            )
        })?
        .to_string();

    let parent_tool_use_id = obj.get("parent_tool_use_id").and_then(|v| v.as_str()).map(String::from);

    Ok(Message::Assistant(AssistantMessage {
        content: content_blocks,
        model,
        parent_tool_use_id,
    }))
}

fn parse_system_message(obj: &serde_json::Map<String, Value>, data: &Value) -> Result<Message> {
    let subtype = obj
        .get("subtype")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in system message: subtype",
                Some(data.clone()),
            )
        })?
        .to_string();

    // Convert the entire object to a HashMap
    let mut data_map = HashMap::new();
    for (key, value) in obj.iter() {
        data_map.insert(key.clone(), value.clone());
    }

    Ok(Message::System(SystemMessage {
        subtype,
        data: data_map,
    }))
}

fn parse_result_message(obj: &serde_json::Map<String, Value>, data: &Value) -> Result<Message> {
    let subtype = obj
        .get("subtype")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in result message: subtype",
                Some(data.clone()),
            )
        })?
        .to_string();

    let duration_ms = obj
        .get("duration_ms")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in result message: duration_ms",
                Some(data.clone()),
            )
        })?;

    let duration_api_ms = obj
        .get("duration_api_ms")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in result message: duration_api_ms",
                Some(data.clone()),
            )
        })?;

    let is_error = obj
        .get("is_error")
        .and_then(|v| v.as_bool())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in result message: is_error",
                Some(data.clone()),
            )
        })?;

    let num_turns = obj
        .get("num_turns")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in result message: num_turns",
                Some(data.clone()),
            )
        })? as i32;

    let session_id = obj
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in result message: session_id",
                Some(data.clone()),
            )
        })?
        .to_string();

    let total_cost_usd = obj.get("total_cost_usd").and_then(|v| v.as_f64());

    let usage = obj.get("usage").and_then(|v| {
        v.as_object().map(|o| {
            o.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<String, Value>>()
        })
    });

    let result = obj.get("result").and_then(|v| v.as_str()).map(String::from);

    Ok(Message::Result(ResultMessage {
        subtype,
        duration_ms,
        duration_api_ms,
        is_error,
        num_turns,
        session_id,
        total_cost_usd,
        usage,
        result,
    }))
}

fn parse_stream_event(obj: &serde_json::Map<String, Value>, data: &Value) -> Result<Message> {
    let uuid = obj
        .get("uuid")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in stream_event message: uuid",
                Some(data.clone()),
            )
        })?
        .to_string();

    let session_id = obj
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in stream_event message: session_id",
                Some(data.clone()),
            )
        })?
        .to_string();

    let event = obj
        .get("event")
        .and_then(|v| v.as_object())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Missing required field in stream_event message: event",
                Some(data.clone()),
            )
        })?
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<HashMap<String, Value>>();

    let parent_tool_use_id = obj.get("parent_tool_use_id").and_then(|v| v.as_str()).map(String::from);

    Ok(Message::Stream(StreamEvent {
        uuid,
        session_id,
        event,
        parent_tool_use_id,
    }))
}

fn parse_content_block(block: &Value, data: &Value) -> Result<ContentBlock> {
    let block_obj = block.as_object().ok_or_else(|| {
        ClaudeSDKError::message_parse_error(
            "Invalid content block type (expected object)",
            Some(data.clone()),
        )
    })?;

    let block_type = block_obj
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ClaudeSDKError::message_parse_error(
                "Content block missing 'type' field",
                Some(data.clone()),
            )
        })?;

    match block_type {
        "text" => {
            let text = block_obj
                .get("text")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    ClaudeSDKError::message_parse_error(
                        "Text block missing 'text' field",
                        Some(data.clone()),
                    )
                })?
                .to_string();
            Ok(ContentBlock::Text { text })
        }
        "thinking" => {
            let thinking = block_obj
                .get("thinking")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    ClaudeSDKError::message_parse_error(
                        "Thinking block missing 'thinking' field",
                        Some(data.clone()),
                    )
                })?
                .to_string();
            let signature = block_obj
                .get("signature")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    ClaudeSDKError::message_parse_error(
                        "Thinking block missing 'signature' field",
                        Some(data.clone()),
                    )
                })?
                .to_string();
            Ok(ContentBlock::Thinking { thinking, signature })
        }
        "tool_use" => {
            let id = block_obj
                .get("id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    ClaudeSDKError::message_parse_error(
                        "Tool use block missing 'id' field",
                        Some(data.clone()),
                    )
                })?
                .to_string();
            let name = block_obj
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    ClaudeSDKError::message_parse_error(
                        "Tool use block missing 'name' field",
                        Some(data.clone()),
                    )
                })?
                .to_string();
            let input = block_obj
                .get("input")
                .and_then(|v| v.as_object())
                .ok_or_else(|| {
                    ClaudeSDKError::message_parse_error(
                        "Tool use block missing or invalid 'input' field",
                        Some(data.clone()),
                    )
                })?
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<String, Value>>();
            Ok(ContentBlock::ToolUse { id, name, input })
        }
        "tool_result" => {
            let tool_use_id = block_obj
                .get("tool_use_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    ClaudeSDKError::message_parse_error(
                        "Tool result block missing 'tool_use_id' field",
                        Some(data.clone()),
                    )
                })?
                .to_string();
            let content = block_obj.get("content").cloned();
            let is_error = block_obj.get("is_error").and_then(|v| v.as_bool());
            Ok(ContentBlock::ToolResult {
                tool_use_id,
                content,
                is_error,
            })
        }
        _ => Err(ClaudeSDKError::message_parse_error(
            format!("Unknown content block type: {}", block_type),
            Some(data.clone()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_invalid_type() {
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
    fn test_parse_unknown_type() {
        let data = json!({
            "type": "unknown_type"
        });
        let result = parse_message(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_text_block() {
        let block = json!({
            "type": "text",
            "text": "Hello, world!"
        });
        let data = json!({});
        let result = parse_content_block(&block, &data).unwrap();
        match result {
            ContentBlock::Text { text } => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected Text block"),
        }
    }

    #[test]
    fn test_parse_tool_use_block() {
        let block = json!({
            "type": "tool_use",
            "id": "tool123",
            "name": "test_tool",
            "input": {
                "param": "value"
            }
        });
        let data = json!({});
        let result = parse_content_block(&block, &data).unwrap();
        match result {
            ContentBlock::ToolUse { id, name, .. } => {
                assert_eq!(id, "tool123");
                assert_eq!(name, "test_tool");
            }
            _ => panic!("Expected ToolUse block"),
        }
    }
}
