/// Example: Simplest way to use the SDK with v2 API
///
/// This demonstrates the new `ask()` facade function which is the
/// most straightforward way to interact with Claude.

use claude::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== V2 Simple Ask Example ===\n");

    // Tier 1: Facade API - Simplest possible usage
    println!("1. Simple ask:");
    let answer = ask("What is 2 + 2? Give just the number.").await?;
    println!("   Answer: {}\n", answer);

    // With system prompt using quick constructor
    println!("2. With system prompt:");
    let options = ClaudeAgentOptions::with_system_prompt("You are a helpful math tutor");
    let answer = ask_with_options("What is the Pythagorean theorem?", options).await?;
    println!("   Answer: {}\n", answer);

    // Using QuickQuery builder for more control
    println!("3. Using QuickQuery builder:");
    let answer = QuickQuery::new("Explain Rust ownership in one sentence")
        .with_system_prompt("You are a Rust expert. Be concise.")
        .max_turns(1)
        .ask()
        .await?;
    println!("   Answer: {}\n", answer);

    // Builder pattern for complex configuration
    println!("4. Using full builder pattern:");
    let options = ClaudeOptionsBuilder::new()
        .system_prompt("You are a helpful coding assistant")
        .model("claude-sonnet-4-5")
        .max_turns(1)
        .allow_tools(["Read", "Write"])
        .build();

    let answer = ask_with_options("What are the benefits of type safety?", options).await?;
    println!("   Answer: {}\n", answer);

    println!("=== Example Complete ===");
    Ok(())
}
