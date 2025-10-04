/// Example: Simplified callback creation with macros
///
/// This demonstrates the new hook! and permission_callback! macros
/// that hide the complex type signatures.

use claude::{hook, permission_callback, HookJSONOutput, PermissionResult};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== V2 Simplified Callbacks Example ===\n");

    // Before: Complex Arc<dyn Fn...> + Pin<Box<Future>> syntax
    // After: Simple hook! macro

    println!("1. Creating a simple hook:");
    let _log_hook = hook!(|input, tool_id| {
        println!("   Hook called!");
        println!("   Tool ID: {:?}", tool_id);
        println!("   Input keys: {:?}", input.keys().collect::<Vec<_>>());
        Ok::<HookJSONOutput, claude::ClaudeSDKError>(HookJSONOutput::default())
    });

    println!("   Hook created with simple macro syntax!");
    println!("   No need for Arc<dyn Fn...> + Pin<Box<Future>>\n");

    // Permission callback example
    println!("2. Creating a permission callback:");
    let _safety_callback = permission_callback!(|tool, input| {
        let _ = input; // Silence unused warning in example
        println!("   Permission check for tool: {}", tool);

        if tool == "Bash" {
            println!("   DENIED: Bash execution not allowed");
            Ok::<PermissionResult, claude::ClaudeSDKError>(PermissionResult::deny("Bash execution is not allowed for safety".to_string()))
        } else {
            println!("   ALLOWED: {} is permitted", tool);
            Ok(PermissionResult::allow())
        }
    });

    println!("   Permission callback created!\n");

    // Using it with ClaudeSDKClient would look like this:
    // let mut client = ClaudeSDKClient::with_permission_callback(None, safety_callback);
    // client.connect().await?;

    // Even simpler - just the tool name
    println!("3. Creating a tool-based permission filter:");
    let _simple_filter = permission_callback!(|tool| {
        if tool.starts_with("Write") || tool == "Bash" {
            Ok::<PermissionResult, claude::ClaudeSDKError>(PermissionResult::deny(format!("{} is blocked", tool)))
        } else {
            Ok(PermissionResult::allow())
        }
    });

    println!("   Simple filter created!");
    println!("   Blocks: Write*, Bash");
    println!("   Allows: Everything else\n");

    println!("=== Example Complete ===");
    println!("\nNote: These callbacks can be used with ClaudeSDKClient");
    println!("when interactive mode is fully supported.");

    Ok(())
}
