/// Example demonstrating error handling in the Claude SDK
///
/// This example shows:
/// - Different error types and their use cases
/// - Error creation and formatting
/// - Result type usage
/// - Error propagation

use claude::errors::{ClaudeSDKError, Result};

fn main() {
    println!("=== Error Handling Demo ===\n");

    // Example 1: CLI Not Found Error
    example_cli_not_found();

    // Example 2: Process Error
    example_process_error();

    // Example 3: JSON Decode Error
    example_json_decode_error();

    // Example 4: Message Parse Error
    example_message_parse_error();

    // Example 5: CLI Connection Error
    example_cli_connection_error();

    // Example 6: Error in Result type
    example_result_type();

    // Example 7: Error propagation
    example_error_propagation();
}

fn example_cli_not_found() {
    println!("--- Example 1: CLI Not Found Error ---");

    // Without path
    let error = ClaudeSDKError::cli_not_found(None);
    println!("Error without path: {}", error);

    // With path
    let error = ClaudeSDKError::cli_not_found(Some("/usr/local/bin/claude".to_string()));
    println!("Error with path: {}", error);
    println!();
}

fn example_process_error() {
    println!("--- Example 2: Process Error ---");

    // Minimal error
    let error = ClaudeSDKError::process_error("Process failed", None, None);
    println!("Minimal error: {}", error);

    // With exit code
    let error = ClaudeSDKError::process_error("Process failed", Some(1), None);
    println!("With exit code: {}", error);

    // With stderr
    let error = ClaudeSDKError::process_error(
        "Process failed",
        Some(127),
        Some("command not found: claude".to_string()),
    );
    println!("With stderr: {}", error);
    println!();
}

fn example_json_decode_error() {
    println!("--- Example 3: JSON Decode Error ---");

    // Short line
    let error = ClaudeSDKError::json_decode_error(
        "{invalid json}",
        "expected value at line 1 column 2",
    );
    println!("JSON error (short): {}", error);

    // Long line (will be truncated)
    let long_json = "a".repeat(200);
    let error = ClaudeSDKError::json_decode_error(&long_json, "invalid character");
    println!("JSON error (long, truncated): {}", error);
    println!();
}

fn example_message_parse_error() {
    println!("--- Example 4: Message Parse Error ---");

    // Without data
    let error = ClaudeSDKError::message_parse_error("Unknown message type", None);
    println!("Parse error (no data): {}", error);

    // With data
    let data = serde_json::json!({
        "type": "unknown",
        "field": "value"
    });
    let error = ClaudeSDKError::message_parse_error("Invalid message format", Some(data));
    println!("Parse error (with data): {}", error);
    println!();
}

fn example_cli_connection_error() {
    println!("--- Example 5: CLI Connection Error ---");

    let error = ClaudeSDKError::cli_connection_error("Failed to connect to Claude Code");
    println!("Connection error: {}", error);
    println!();
}

fn example_result_type() {
    println!("--- Example 6: Using Result Type ---");

    // Function that returns Result
    fn simulate_operation(should_fail: bool) -> Result<String> {
        if should_fail {
            Err(ClaudeSDKError::process_error(
                "Operation failed",
                Some(1),
                None,
            ))
        } else {
            Ok("Operation successful".to_string())
        }
    }

    // Success case
    match simulate_operation(false) {
        Ok(result) => println!("✓ Success: {}", result),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Error case
    match simulate_operation(true) {
        Ok(result) => println!("✓ Success: {}", result),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn example_error_propagation() {
    println!("--- Example 7: Error Propagation ---");

    // Nested functions demonstrating error propagation
    fn inner_function() -> Result<()> {
        Err(ClaudeSDKError::json_decode_error(
            "{bad json}",
            "parse error",
        ))
    }

    fn middle_function() -> Result<String> {
        inner_function()?; // Propagate error using ?
        Ok("This won't be reached".to_string())
    }

    fn outer_function() -> Result<()> {
        let _result = middle_function()?; // Propagate error again
        Ok(())
    }

    // Execute the chain
    match outer_function() {
        Ok(_) => println!("✓ All operations succeeded"),
        Err(e) => println!("✗ Error propagated through chain: {}", e),
    }

    println!();

    // Example with error recovery
    fn with_recovery() -> Result<String> {
        match inner_function() {
            Ok(_) => Ok("Success".to_string()),
            Err(_) => {
                println!("  Recovered from error, using fallback");
                Ok("Fallback value".to_string())
            }
        }
    }

    match with_recovery() {
        Ok(result) => println!("✓ Recovered: {}", result),
        Err(e) => println!("✗ Error: {}", e),
    }

    println!();
}

// Bonus: Demonstrate error trait implementation
fn bonus_example() {
    println!("--- Bonus: Error Trait Implementation ---");

    let error = ClaudeSDKError::process_error("Test error", Some(1), None);

    // Can use as std::error::Error
    let _error_ref: &dyn std::error::Error = &error;
    println!("✓ Error implements std::error::Error trait");

    // Display implementation
    println!("Display: {}", error);

    // Debug implementation
    println!("Debug: {:?}", error);

    println!();
    println!("=== Demo Complete ===");
}

#[allow(dead_code)]
fn run_bonus() {
    bonus_example();
}
