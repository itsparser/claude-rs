/// Hooks example demonstrating PreToolUse callbacks
///
/// This shows how to:
/// - Create hook callbacks
/// - Register them with HookManager
/// - Use them with ClaudeSDKClient (when integrated)
///
/// Run with: cargo run --example hooks_example

use claude::{HookCallback, HookManager, HookMatcherConfig, HookJSONOutput, HookContext};
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Hooks System Example ===\n");

    // Example 1: Create a simple hook callback
    println!("Example 1: Simple Hook Callback");
    println!("---");

    let callback: HookCallback = Arc::new(|input_data, tool_use_id, _context| {
        Box::pin(async move {
            println!("Hook called with input: {:?}", input_data);
            println!("Tool use ID: {:?}", tool_use_id);

            Ok(HookJSONOutput {
                decision: Some("allow".to_string()),
                system_message: Some("Hook executed successfully".to_string()),
                hook_specific_output: None,
            })
        })
    });

    let mut manager = HookManager::new();
    let callback_id = manager.register_callback(callback);
    println!("Registered callback with ID: {}", callback_id);

    println!("\n---\n");

    // Example 2: Add a matcher for Bash commands
    println!("Example 2: Hook Matcher for Bash");
    println!("---");

    let bash_callback: HookCallback = Arc::new(|input_data, _tool_use_id, _context| {
        Box::pin(async move {
            // Check if the command contains dangerous patterns
            if let Some(command) = input_data.get("command") {
                let cmd_str = command.as_str().unwrap_or("");
                if cmd_str.contains("rm -rf") {
                    println!("⚠️  Blocked dangerous command: {}", cmd_str);
                    return Ok(HookJSONOutput {
                        decision: Some("block".to_string()),
                        system_message: Some("Dangerous command blocked for safety".to_string()),
                        hook_specific_output: None,
                    });
                }
            }

            println!("✓ Command allowed");
            Ok(HookJSONOutput {
                decision: Some("allow".to_string()),
                system_message: None,
                hook_specific_output: None,
            })
        })
    });

    let bash_callback_id = manager.register_callback(bash_callback);
    let bash_matcher = HookMatcherConfig::new("Bash".to_string(), vec![bash_callback_id]);
    manager.add_matcher("PreToolUse".to_string(), bash_matcher);

    println!("Registered Bash command safety hook");

    println!("\n---\n");

    // Example 3: Wildcard matcher (matches all tools)
    println!("Example 3: Wildcard Hook");
    println!("---");

    let logging_callback: HookCallback = Arc::new(|input_data, tool_use_id, _context| {
        Box::pin(async move {
            println!("📝 Logging: Tool used - {:?}", tool_use_id);
            println!("📝 Input data keys: {:?}", input_data.keys().collect::<Vec<_>>());

            Ok(HookJSONOutput::default())
        })
    });

    let logging_id = manager.register_callback(logging_callback);
    let wildcard_matcher = HookMatcherConfig::new("*".to_string(), vec![logging_id]);
    manager.add_matcher("PreToolUse".to_string(), wildcard_matcher);

    println!("Registered wildcard logging hook");

    println!("\n---\n");

    // Example 4: Execute hooks
    println!("Example 4: Execute Hooks");
    println!("---");

    let mut test_input = HashMap::new();
    test_input.insert(
        "command".to_string(),
        serde_json::Value::String("ls -la".to_string()),
    );

    let context = HookContext { signal: None };

    println!("Testing safe command:");
    let results = manager
        .execute_hooks("PreToolUse", "Bash", test_input.clone(), Some("test-id".to_string()), context.clone())
        .await?;

    println!("Hook execution results: {} hooks executed", results.len());

    println!("\nTesting dangerous command:");
    let mut dangerous_input = HashMap::new();
    dangerous_input.insert(
        "command".to_string(),
        serde_json::Value::String("rm -rf /".to_string()),
    );

    let dangerous_results = manager
        .execute_hooks("PreToolUse", "Bash", dangerous_input, Some("test-id-2".to_string()), context)
        .await?;

    for (i, result) in dangerous_results.iter().enumerate() {
        println!("Hook {} result:", i + 1);
        if let Some(ref decision) = result.decision {
            println!("  Decision: {}", decision);
        }
        if let Some(ref msg) = result.system_message {
            println!("  Message: {}", msg);
        }
    }

    println!("\n---\n");

    // Example 5: Hook configuration
    println!("Example 5: Hook Configuration");
    println!("---");

    let config = manager.get_initialization_config();
    println!("Hook configuration for initialization:");
    println!("{}", serde_json::to_string_pretty(&config)?);

    println!("\n=== Examples Complete ===");

    Ok(())
}
