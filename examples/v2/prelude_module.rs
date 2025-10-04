/// Example: Prelude Module - One Import for Everything
///
/// Demonstrates the prelude module that provides all commonly used
/// types and functions in a single import.

// The magic line - one import for everything!
use claude::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== Prelude Module Example ===\n");

    println!("This example demonstrates the prelude module.");
    println!("With just `use claude::prelude::*`, you get:\n");

    // Show what's available
    println!("✓ Available Types and Functions:");
    println!();

    println!("1. Facade Functions:");
    println!("   • ask() - Simplest query function");
    println!("   • ask_with_options() - Query with configuration");
    println!("   • QuickQuery - Fluent builder for queries");
    println!();

    println!("2. Core Query Functions:");
    println!("   • simple_query() - One-shot query, collect all messages");
    println!("   • streaming_query() - Streaming response");
    println!("   • StreamingQuery - Stream type");
    println!();

    println!("3. Client:");
    println!("   • ClaudeSDKClient - Interactive session client");
    println!();

    println!("4. Configuration:");
    println!("   • ClaudeAgentOptions - Configuration struct");
    println!("   • ClaudeOptionsBuilder - Builder for configuration");
    println!("   • PermissionMode - Permission mode enum");
    println!("   • SystemPromptConfig - System prompt configuration");
    println!();

    println!("5. Types:");
    println!("   • Message - Message enum");
    println!("   • ContentBlock - Content block enum");
    println!();

    println!("6. Error Handling:");
    println!("   • ClaudeSDKError - Error type");
    println!("   • Result<T> - Shorthand for Result<T, ClaudeSDKError>");
    println!();

    println!("7. Extension Traits:");
    println!("   • MessageVecExt - Methods for Vec<Message>");
    println!();

    println!("8. Macros (re-exported):");
    println!("   • hook! - Create hook callbacks");
    println!("   • permission_callback! - Create permission callbacks");
    println!();

    // Demonstrate usage
    println!("=== Usage Examples ===\n");

    println!("Example 1: Quick Query");
    println!("```rust");
    println!("use claude::prelude::*;");
    println!();
    println!("let answer = ask(\"What is Rust?\").await?;");
    println!("```\n");

    println!("Example 2: With Builder");
    println!("```rust");
    println!("use claude::prelude::*;");
    println!();
    println!("let answer = QuickQuery::new(\"Explain ownership\")");
    println!("    .with_system_prompt(\"You are a Rust expert\")");
    println!("    .max_turns(1)");
    println!("    .ask()");
    println!("    .await?;");
    println!("```\n");

    println!("Example 3: Full Configuration");
    println!("```rust");
    println!("use claude::prelude::*;");
    println!();
    println!("let options = ClaudeOptionsBuilder::new()");
    println!("    .system_prompt(\"You are helpful\")");
    println!("    .model(\"claude-sonnet-4-5\")");
    println!("    .max_turns(5)");
    println!("    .build();");
    println!();
    println!("let messages = simple_query(\"Help me\", Some(options)).await?;");
    println!("let text = messages.text_content(); // Extension trait method!");
    println!("```\n");

    println!("Example 4: Before/After Comparison");
    println!();
    println!("BEFORE (V1) - Multiple imports:");
    println!("```rust");
    println!("use claude::simple_query;");
    println!("use claude::streaming_query::{{streaming_query, StreamingQuery}};");
    println!("use claude::types::{{ClaudeAgentOptions, Message, ContentBlock}};");
    println!("use claude::errors::{{ClaudeSDKError, Result}};");
    println!("use claude::client::ClaudeSDKClient;");
    println!("```\n");

    println!("AFTER (V2) - One import:");
    println!("```rust");
    println!("use claude::prelude::*;");
    println!("```\n");

    println!("=== Example Complete ===");
    println!("\nBenefits of the Prelude:");
    println!("• Single import - no need to remember individual paths");
    println!("• Contains 90% of what you need for typical usage");
    println!("• Clean and simple - reduces boilerplate");
    println!("• Follows Rust conventions (like std::prelude)");
    println!("\nFor advanced features:");
    println!("```rust");
    println!("use claude::prelude::*;");
    println!("use claude::hooks::HookManager;  // Advanced: if needed");
    println!("use claude::mcp::SdkMcpServer;   // Advanced: if needed");
    println!("```");

    Ok(())
}
