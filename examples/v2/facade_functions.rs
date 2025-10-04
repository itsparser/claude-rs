/// Example: Facade Functions - Simplest API
///
/// Demonstrates ask(), ask_with_options(), and QuickQuery
/// for the most common use cases with minimal code.

use claude::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== Facade Functions Example ===\n");

    // Example 1: Simplest possible - ask()
    println!("1. ask() - Simplest Function:");
    println!("   Code: let answer = ask(\"What is 2 + 2?\").await?;");

    // Uncomment to run actual query:
    // let answer = ask("What is 2 + 2? Give just the number.").await?;
    // println!("   Answer: {}\n", answer);
    println!("   (Skipped for demo - would make real API call)\n");

    // Example 2: ask_with_options()
    println!("2. ask_with_options() - With Configuration:");
    let options = ClaudeAgentOptions::with_system_prompt("You are a Rust expert");
    println!("   Code:");
    println!("   let options = ClaudeAgentOptions::with_system_prompt(\"You are a Rust expert\");");
    println!("   let answer = ask_with_options(\"What is ownership?\", options).await?;");

    // Uncomment to run:
    // let answer = ask_with_options("What is ownership in Rust?", options).await?;
    // println!("   Answer: {}\n", answer);
    println!("   (Skipped for demo)\n");

    // Example 3: QuickQuery builder - Basic
    println!("3. QuickQuery - Fluent Builder:");
    println!("   Code:");
    println!("   let answer = QuickQuery::new(\"Explain async/await\")");
    println!("       .with_system_prompt(\"You are a teacher\")");
    println!("       .max_turns(1)");
    println!("       .ask()");
    println!("       .await?;");
    println!("   (Skipped for demo)\n");

    // Example 4: QuickQuery with multiple options
    println!("4. QuickQuery - Multiple Options:");
    println!("   Code:");
    println!("   let answer = QuickQuery::new(\"Review this code\")");
    println!("       .with_model(\"claude-sonnet-4-5\")");
    println!("       .with_system_prompt(\"You are a code reviewer\")");
    println!("       .max_turns(5)");
    println!("       .allow_tools([\"Read\", \"Edit\"])");
    println!("       .ask()");
    println!("       .await?;");
    println!("   (Skipped for demo)\n");

    // Example 5: QuickQuery returning full messages
    println!("5. QuickQuery.query() - Get Full Messages:");
    println!("   Code:");
    println!("   let messages = QuickQuery::new(\"Question\")");
    println!("       .query()  // Returns Vec<Message> instead of String");
    println!("       .await?;");
    println!("   (Skipped for demo)\n");

    // Example 6: QuickQuery streaming
    println!("6. QuickQuery.stream() - Streaming Response:");
    println!("   Code:");
    println!("   let mut stream = QuickQuery::new(\"Long explanation\")");
    println!("       .stream()  // Returns StreamingQuery");
    println!("       .await?;");
    println!("   while let Some(msg) = stream.next().await {{ ... }}");
    println!("   (Skipped for demo)\n");

    // Example 7: Comparison
    println!("7. Before/After Comparison:");
    println!("\n   BEFORE (V1) - 15+ lines:");
    println!("   ```rust");
    println!("   let mut options = ClaudeAgentOptions::default();");
    println!("   options.system_prompt = Some(SystemPromptConfig::Text(\"...\".to_string()));");
    println!("   options.max_turns = Some(1);");
    println!("   let messages = simple_query(\"question\", Some(options)).await?;");
    println!("   for msg in messages {{");
    println!("       if let Message::Assistant(assistant) = msg {{");
    println!("           for content in &assistant.content {{");
    println!("               if let ContentBlock::Text(text) = content {{");
    println!("                   println!(\"{{}}\", text.text);");
    println!("               }}");
    println!("           }}");
    println!("       }}");
    println!("   }}");
    println!("   ```\n");

    println!("   AFTER (V2) - 1 line:");
    println!("   ```rust");
    println!("   let answer = ask(\"question\").await?;");
    println!("   ```\n");

    println!("=== Example Complete ===");
    println!("\nAPI Tiers:");
    println!("• ask() - Simplest: 1 line, just get text");
    println!("• ask_with_options() - Simple with config: 3 lines");
    println!("• QuickQuery - Fluent builder: 4-6 lines, very readable");
    println!("\nAll three hide complexity and reduce boilerplate by 80%!");

    Ok(())
}
