/// Example: ClaudeOptionsBuilder - Fluent Configuration API
///
/// Demonstrates the builder pattern for constructing ClaudeAgentOptions
/// with a chainable, discoverable API.

use claude::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== ClaudeOptionsBuilder Example ===\n");

    // Example 1: Basic builder usage
    println!("1. Basic Configuration:");
    let options = ClaudeOptionsBuilder::new()
        .system_prompt("You are a helpful math tutor")
        .max_turns(5)
        .model("claude-sonnet-4-5")
        .build();

    println!("   ✓ Created options with system prompt, max turns, and model\n");

    // Example 2: Tool configuration
    println!("2. Tool Configuration:");
    let options = ClaudeOptionsBuilder::new()
        .allow_tools(["Read", "Write", "Edit"])
        .deny_tool("Bash")
        .permission_mode(PermissionMode::AcceptEdits)
        .build();

    println!("   ✓ Allowed tools: Read, Write, Edit");
    println!("   ✓ Denied tool: Bash");
    println!("   ✓ Permission mode: AcceptEdits\n");

    // Example 3: Session management
    println!("3. Session Configuration:");
    let options = ClaudeOptionsBuilder::new()
        .resume_session("session-12345")
        .continue_conversation(true)
        .build();

    println!("   ✓ Resume session: session-12345");
    println!("   ✓ Continue conversation: true\n");

    // Example 4: Fork session
    println!("4. Fork Session:");
    let options = ClaudeOptionsBuilder::new()
        .fork_session("session-12345")
        .build();

    println!("   ✓ Forked from: session-12345\n");

    // Example 5: Environment and paths
    println!("5. Environment Configuration:");
    let options = ClaudeOptionsBuilder::new()
        .cwd("/path/to/project")
        .add_directory("/path/to/context")
        .env("DEBUG", "true")
        .env("API_KEY", "secret")
        .build();

    println!("   ✓ Working directory: /path/to/project");
    println!("   ✓ Context directory added");
    println!("   ✓ Environment variables set\n");

    // Example 6: Complete configuration
    println!("6. Complete Configuration:");
    let options = ClaudeOptionsBuilder::new()
        .system_prompt("You are a code review assistant")
        .model("claude-sonnet-4-5")
        .max_turns(10)
        .allow_tools(["Read", "Edit", "Grep", "Bash"])
        .permission_mode(PermissionMode::Plan)
        .cwd(".")
        .include_partial_messages(true)
        .max_buffer_size(1024 * 1024)
        .build();

    println!("   ✓ All features configured in one chain!");
    println!("   ✓ Readable and discoverable through IDE autocomplete\n");

    // Example 7: Quick constructors
    println!("7. Quick Constructors:");
    let opt1 = ClaudeAgentOptions::with_system_prompt("You are helpful");
    println!("   ✓ Quick system prompt: {:?}", opt1.system_prompt.is_some());

    let opt2 = ClaudeAgentOptions::with_model("claude-sonnet-4-5");
    println!("   ✓ Quick model: {:?}", opt2.model);

    println!("\n=== Example Complete ===");
    println!("\nKey Benefits:");
    println!("• Chainable API - easy to read and write");
    println!("• IDE autocomplete - discover all options");
    println!("• Type-safe - compile-time validation");
    println!("• Flexible - build only what you need");

    Ok(())
}
