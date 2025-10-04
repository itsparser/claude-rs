use claude::errors::*;

#[test]
fn test_cli_connection_error() {
    let error = ClaudeSDKError::cli_connection_error("Connection failed");
    assert!(error.to_string().contains("Connection failed"));
}

#[test]
fn test_cli_not_found_without_path() {
    let error = ClaudeSDKError::cli_not_found(None);
    let msg = error.to_string();
    assert!(msg.contains("Claude Code not found"));
}

#[test]
fn test_cli_not_found_with_path() {
    let error = ClaudeSDKError::cli_not_found(Some("/usr/local/bin/claude".to_string()));
    let msg = error.to_string();
    assert!(msg.contains("Claude Code not found"));
    assert!(msg.contains("/usr/local/bin/claude"));
}

#[test]
fn test_process_error_minimal() {
    let error = ClaudeSDKError::process_error("Process failed", None, None);
    let msg = error.to_string();
    assert!(msg.contains("Process failed"));
}

#[test]
fn test_process_error_with_exit_code() {
    let error = ClaudeSDKError::process_error("Process failed", Some(1), None);
    let msg = error.to_string();
    assert!(msg.contains("Process failed"));
    assert!(msg.contains("exit code: 1"));
}

#[test]
fn test_process_error_with_stderr() {
    let error = ClaudeSDKError::process_error(
        "Process failed",
        Some(1),
        Some("Error details".to_string()),
    );
    let msg = error.to_string();
    assert!(msg.contains("Process failed"));
    assert!(msg.contains("exit code: 1"));
    assert!(msg.contains("Error output: Error details"));
}

#[test]
fn test_json_decode_error() {
    let error = ClaudeSDKError::json_decode_error(
        "{invalid json}",
        "expected value at line 1 column 2",
    );
    let msg = error.to_string();
    assert!(msg.contains("Failed to decode JSON"));
    assert!(msg.contains("{invalid json}"));
}

#[test]
fn test_json_decode_error_truncates_long_lines() {
    let long_line = "a".repeat(200);
    let error = ClaudeSDKError::json_decode_error(&long_line, "parse error");
    let msg = error.to_string();
    assert!(msg.contains("..."));
    // Message should be truncated
    assert!(msg.len() < long_line.len() + 100);
}

#[test]
fn test_message_parse_error_without_data() {
    let error = ClaudeSDKError::message_parse_error("Invalid message format", None);
    let msg = error.to_string();
    assert!(msg.contains("Invalid message format"));
}

#[test]
fn test_message_parse_error_with_data() {
    let data = serde_json::json!({
        "type": "unknown",
        "field": "value"
    });
    let error = ClaudeSDKError::message_parse_error("Unknown message type", Some(data));
    let msg = error.to_string();
    assert!(msg.contains("Unknown message type"));
}

#[test]
fn test_error_trait_implementation() {
    let error = ClaudeSDKError::cli_connection_error("test");
    // This should compile if Error trait is properly implemented
    let _: &dyn std::error::Error = &error;
}

#[test]
fn test_result_type() {
    fn returns_result() -> Result<String> {
        Ok("success".to_string())
    }

    fn returns_error() -> Result<String> {
        Err(ClaudeSDKError::cli_connection_error("failed"))
    }

    assert!(returns_result().is_ok());
    assert!(returns_error().is_err());
}

#[test]
fn test_error_chain() {
    fn inner_function() -> Result<()> {
        Err(ClaudeSDKError::process_error("inner error", None, None))
    }

    fn outer_function() -> Result<String> {
        inner_function()?;
        Ok("success".to_string())
    }

    let result = outer_function();
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("inner error"));
    }
}
