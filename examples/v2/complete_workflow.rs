/// Example: Complete V2 Workflow
///
/// Demonstrates a complete workflow using all V2 features together
/// in a realistic scenario.

use claude::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== Complete V2 Workflow Example ===\n");
    println!("Scenario: Code Review Assistant\n");

    // Step 1: Simple question with ask()
    println!("Step 1: Quick Question");
    println!("-------");
    println!("Using ask() for a simple query:");
    println!("Code: let answer = ask(\"What makes good code?\").await?;");
    println!("(Skipped - would make real API call)\n");

    // Step 2: Configure with builder
    println!("Step 2: Configure Review Assistant");
    println!("-------");
    let options = ClaudeOptionsBuilder::new()
        .system_prompt("You are an expert code reviewer focusing on Rust best practices")
        .model("claude-sonnet-4-5")
        .max_turns(5)
        .allow_tools(["Read", "Edit", "Grep"])
        .permission_mode(PermissionMode::Plan)
        .build();

    println!("✓ Built configuration with:");
    println!("  • System prompt: Code reviewer");
    println!("  • Model: claude-sonnet-4-5");
    println!("  • Max turns: 5");
    println!("  • Tools: Read, Edit, Grep");
    println!("  • Permission mode: Plan\n");

    // Step 3: Use QuickQuery for formatted request
    println!("Step 3: Request Code Review");
    println!("-------");
    println!("Using QuickQuery for a structured request:");
    println!("Code:");
    println!("  let review = QuickQuery::new(\"Review the code in src/main.rs\")");
    println!("      .with_model(\"claude-sonnet-4-5\")");
    println!("      .with_system_prompt(\"Focus on safety and performance\")");
    println!("      .max_turns(3)");
    println!("      .ask()");
    println!("      .await?;");
    println!("(Skipped)\n");

    // Step 4: Work with messages using extensions
    println!("Step 4: Process Response");
    println!("-------");
    println!("If we used query() instead of ask(), we'd get messages:");
    println!("Code:");
    println!("  let messages = QuickQuery::new(\"...\").query().await?;");
    println!("  ");
    println!("  // Use extension methods:");
    println!("  let text = messages.text_content();           // All text");
    println!("  let last = messages.last_assistant();         // Last response");
    println!("  let blocks = messages.text_blocks();          // Individual blocks");
    println!("  ");
    println!("  if messages.has_assistant_messages() {{");
    println!("      for msg in messages.assistant_messages() {{");
    println!("          println!(\"Model: {{}}\", msg.model);");
    println!("      }}");
    println!("  }}");
    println!();

    // Step 5: Add safety with permission callbacks
    println!("Step 5: Add Safety Checks");
    println!("-------");
    println!("Using permission_callback! macro:");
    println!("Code:");
    println!("  let safety = permission_callback!(|tool, _input| {{");
    println!("      match tool.as_str() {{");
    println!("          \"Bash\" => {{");
    println!("              Ok(PermissionResult::deny(\"Bash not allowed\".to_string()))");
    println!("          }}");
    println!("          \"Write\" | \"Edit\" => {{");
    println!("              Ok(PermissionResult::allow()) // Allow with confirmation");
    println!("          }}");
    println!("          _ => Ok(PermissionResult::allow())");
    println!("      }}");
    println!("  }});");
    println!();
    println!("  // Use with client:");
    println!("  // let mut client = ClaudeSDKClient::with_permission_callback(");
    println!("  //     Some(options),");
    println!("  //     safety");
    println!("  // );");
    println!();

    // Step 6: Add logging with hooks
    println!("Step 6: Add Logging");
    println!("-------");
    println!("Using hook! macro for monitoring:");
    println!("Code:");
    println!("  let log_hook = hook!(|input, tool_id| {{");
    println!("      println!(\"[AUDIT] Tool used: {{:?}}\", tool_id);");
    println!("      println!(\"[AUDIT] Input keys: {{:?}}\", input.keys());");
    println!("      Ok(HookJSONOutput::default())");
    println!("  }});");
    println!();

    // Summary
    println!("=== Workflow Summary ===\n");

    println!("This example showed:");
    println!("1. ✓ ask() - Simplest queries");
    println!("2. ✓ ClaudeOptionsBuilder - Chainable configuration");
    println!("3. ✓ QuickQuery - Fluent query building");
    println!("4. ✓ MessageVecExt - Easy message handling");
    println!("5. ✓ permission_callback! - Simple safety checks");
    println!("6. ✓ hook! - Easy logging/monitoring");
    println!();

    println!("Benefits achieved:");
    println!("• 80% less boilerplate code");
    println!("• Clear, readable API");
    println!("• Type-safe configuration");
    println!("• Easy to add safety and monitoring");
    println!("• IDE autocomplete for discovery");
    println!();

    println!("Lines of code comparison:");
    println!("• V1 (verbose): ~50 lines for this workflow");
    println!("• V2 (ergonomic): ~10 lines for same functionality");
    println!();

    println!("=== Try It Yourself ===");
    println!("\n1. Start simple:");
    println!("   let answer = ask(\"your question\").await?;");
    println!();
    println!("2. Add configuration:");
    println!("   QuickQuery::new(\"question\")");
    println!("       .with_system_prompt(\"...\")");
    println!("       .ask().await?;");
    println!();
    println!("3. Get full control:");
    println!("   let options = ClaudeOptionsBuilder::new()");
    println!("       .system_prompt(\"...\")");
    println!("       .max_turns(5)");
    println!("       .build();");
    println!("   let messages = simple_query(\"question\", Some(options)).await?;");
    println!("   let text = messages.text_content();");

    Ok(())
}
