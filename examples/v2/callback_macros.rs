/// Example: Callback Macros - Simplified hook! and permission_callback!
///
/// Demonstrates how the macros hide complex type signatures
/// and make callback creation much simpler.

use claude::{hook, permission_callback, HookJSONOutput, PermissionResult};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== Callback Macros Example ===\n");

    // HOOK MACRO EXAMPLES

    println!("1. hook! - Basic Usage (all parameters):");
    println!("   Code:");
    println!("   let callback = hook!(|input, tool_id, ctx| {{");
    println!("       println!(\"Hook called!\");");
    println!("       Ok(HookJSONOutput::default())");
    println!("   }});");

    let _callback1 = hook!(|input, tool_id, ctx| {
        let _ = (input, tool_id, ctx);
        Ok::<HookJSONOutput, claude::ClaudeSDKError>(HookJSONOutput::default())
    });
    println!("   ✓ Created hook with all parameters\n");

    println!("2. hook! - Without Context:");
    println!("   Code:");
    println!("   let callback = hook!(|input, tool_id| {{");
    println!("       // Use input and tool_id");
    println!("       Ok(HookJSONOutput::default())");
    println!("   }});");

    let _callback2 = hook!(|input, tool_id| {
        let _ = (input, tool_id);
        Ok::<HookJSONOutput, claude::ClaudeSDKError>(HookJSONOutput::default())
    });
    println!("   ✓ Created hook without context parameter\n");

    println!("3. hook! - Just Input:");
    println!("   Code:");
    println!("   let callback = hook!(|input| {{");
    println!("       println!(\"Input: {{:?}}\", input.keys());");
    println!("       Ok(HookJSONOutput::default())");
    println!("   }});");

    let _callback3 = hook!(|input| {
        let _ = input;
        Ok::<HookJSONOutput, claude::ClaudeSDKError>(HookJSONOutput::default())
    });
    println!("   ✓ Created hook with just input\n");

    // PERMISSION CALLBACK MACRO EXAMPLES

    println!("4. permission_callback! - Full Parameters:");
    println!("   Code:");
    println!("   let callback = permission_callback!(|tool, input, ctx| {{");
    println!("       if tool == \"Bash\" {{");
    println!("           Ok(PermissionResult::deny(\"Unsafe\".to_string()))");
    println!("       }} else {{");
    println!("           Ok(PermissionResult::allow())");
    println!("       }}");
    println!("   }});");

    let _perm1 = permission_callback!(|tool, input, ctx| {
        let _ = (input, ctx);
        if tool == "Bash" {
            Ok::<PermissionResult, claude::ClaudeSDKError>(PermissionResult::deny(
                "Unsafe".to_string()
            ))
        } else {
            Ok(PermissionResult::allow())
        }
    });
    println!("   ✓ Created permission callback with all parameters\n");

    println!("5. permission_callback! - Without Context:");
    println!("   Code:");
    println!("   let callback = permission_callback!(|tool, input| {{");
    println!("       // Check tool and input");
    println!("       Ok(PermissionResult::allow())");
    println!("   }});");

    let _perm2 = permission_callback!(|tool, input| {
        let _ = (tool, input);
        Ok::<PermissionResult, claude::ClaudeSDKError>(PermissionResult::allow())
    });
    println!("   ✓ Created permission callback without context\n");

    println!("6. permission_callback! - Just Tool Name:");
    println!("   Code:");
    println!("   let callback = permission_callback!(|tool| {{");
    println!("       if tool.starts_with(\"Write\") {{");
    println!("           Ok(PermissionResult::deny(tool))");
    println!("       }} else {{");
    println!("           Ok(PermissionResult::allow())");
    println!("       }}");
    println!("   }});");

    let _perm3 = permission_callback!(|tool| {
        if tool.starts_with("Write") {
            Ok::<PermissionResult, claude::ClaudeSDKError>(PermissionResult::deny(tool))
        } else {
            Ok(PermissionResult::allow())
        }
    });
    println!("   ✓ Created simple tool filter\n");

    // REAL-WORLD EXAMPLES

    println!("7. Real-World Example - Logging Hook:");
    println!("   Code:");
    println!("   let log_hook = hook!(|input, tool_id| {{");
    println!("       println!(\"[HOOK] Tool: {{:?}}\", tool_id);");
    println!("       println!(\"[HOOK] Input keys: {{:?}}\", input.keys());");
    println!("       Ok(HookJSONOutput::default())");
    println!("   }});");
    println!("   ✓ Useful for debugging and monitoring\n");

    println!("8. Real-World Example - Safety Filter:");
    println!("   Code:");
    println!("   let safety = permission_callback!(|tool| {{");
    println!("       match tool.as_str() {{");
    println!("           \"Bash\" | \"Write\" => {{");
    println!("               Ok(PermissionResult::deny(\"Blocked for safety\".to_string()))");
    println!("           }}");
    println!("           _ => Ok(PermissionResult::allow())");
    println!("       }}");
    println!("   }});");
    println!("   ✓ Block dangerous operations\n");

    // COMPARISON

    println!("9. Before/After Comparison:");
    println!("\n   BEFORE (V1) - Complex type signature:");
    println!("   ```rust");
    println!("   use std::sync::Arc;");
    println!("   use std::pin::Pin;");
    println!("   use std::future::Future;");
    println!();
    println!("   let callback: Arc<");
    println!("       dyn Fn(String, HashMap<String, Value>, Context)");
    println!("           -> Pin<Box<dyn Future<Output = Result<PermissionResult>> + Send>>");
    println!("           + Send + Sync");
    println!("   > = Arc::new(move |tool, input, ctx| {{");
    println!("       Box::pin(async move {{");
    println!("           // Your logic here");
    println!("       }})");
    println!("   }});");
    println!("   ```\n");

    println!("   AFTER (V2) - Simple macro:");
    println!("   ```rust");
    println!("   let callback = permission_callback!(|tool, input, ctx| {{");
    println!("       // Your logic here");
    println!("   }});");
    println!("   ```\n");

    println!("=== Example Complete ===");
    println!("\nMacro Variants:");
    println!("hook! macro:");
    println!("  • hook!(|input, tool_id, ctx| {{ ... }}) - All parameters");
    println!("  • hook!(|input, tool_id| {{ ... }}) - No context");
    println!("  • hook!(|input| {{ ... }}) - Just input");
    println!();
    println!("permission_callback! macro:");
    println!("  • permission_callback!(|tool, input, ctx| {{ ... }}) - All parameters");
    println!("  • permission_callback!(|tool, input| {{ ... }}) - No context");
    println!("  • permission_callback!(|tool| {{ ... }}) - Just tool name");
    println!();
    println!("Benefits:");
    println!("• Hide complex Arc<dyn Fn...> + Pin<Box<Future>> types");
    println!("• Familiar closure syntax");
    println!("• Auto-infer parameters you don't need");
    println!("• Much easier to read and write!");

    Ok(())
}
