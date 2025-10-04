use claude::{SdkMcpServer, McpTool, ToolHandler, ToolResult};
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SDK MCP Server Example ===\n");
    println!("This example demonstrates creating custom tools with SDK MCP servers.");
    println!("SDK servers run in-process, eliminating subprocess overhead.\n");

    // Create a simple greeting tool
    let greet_tool = McpTool::new(
        "greet".to_string(),
        "Greet a user by name".to_string(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The name of the person to greet"
                }
            },
            "required": ["name"]
        }),
        Arc::new(|args| {
            Box::pin(async move {
                let name = args
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("World");
                Ok(ToolResult::text(format!("Hello, {}! Welcome to SDK MCP servers!", name)))
            })
        }),
    );

    // Create a calculator tool
    let calc_tool = McpTool::new(
        "calculate".to_string(),
        "Perform basic arithmetic operations".to_string(),
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["add", "subtract", "multiply", "divide"],
                    "description": "The operation to perform"
                },
                "a": {
                    "type": "number",
                    "description": "First operand"
                },
                "b": {
                    "type": "number",
                    "description": "Second operand"
                }
            },
            "required": ["operation", "a", "b"]
        }),
        Arc::new(|args| {
            Box::pin(async move {
                let operation = args
                    .get("operation")
                    .and_then(|v| v.as_str())
                    .unwrap_or("add");

                let a = args
                    .get("a")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);

                let b = args
                    .get("b")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);

                let result = match operation {
                    "add" => a + b,
                    "subtract" => a - b,
                    "multiply" => a * b,
                    "divide" => {
                        if b == 0.0 {
                            return Ok(ToolResult::error("Division by zero".to_string()));
                        }
                        a / b
                    }
                    _ => return Ok(ToolResult::error(format!("Unknown operation: {}", operation))),
                };

                Ok(ToolResult::text(format!("{} {} {} = {}", a, operation, b, result)))
            })
        }),
    );

    // Create a system info tool
    let sysinfo_tool: McpTool = McpTool::new(
        "system_info".to_string(),
        "Get system information".to_string(),
        serde_json::json!({
            "type": "object",
            "properties": {}
        }),
        Arc::new(|_args| {
            Box::pin(async move {
                let os = std::env::consts::OS;
                let arch = std::env::consts::ARCH;
                let family = std::env::consts::FAMILY;

                let info = format!(
                    "System Information:\n- OS: {}\n- Architecture: {}\n- Family: {}",
                    os, arch, family
                );

                Ok(ToolResult::text(info))
            })
        }),
    );

    // Create SDK MCP server with all tools
    let server = SdkMcpServer::new(
        "example-tools".to_string(),
        "1.0.0".to_string(),
        vec![greet_tool, calc_tool, sysinfo_tool],
    );

    println!("✓ Created SDK MCP Server: {}", server.name);
    println!("✓ Version: {}", server.version);
    println!("✓ Registered tools: {}\n", server.list_tools().len());

    // List all available tools
    println!("Available Tools:");
    for tool in server.list_tools() {
        println!("  - {}: {}", tool.name, tool.description);
    }
    println!();

    // Demonstrate tool execution
    println!("--- Tool Execution Examples ---\n");

    // 1. Greet tool
    println!("1. Testing greet tool:");
    let mut greet_args = HashMap::new();
    greet_args.insert("name".to_string(), serde_json::json!("Alice"));

    match server.execute_tool("greet", greet_args).await {
        Ok(result) => {
            println!("   Result: {}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }
    println!();

    // 2. Calculator tool - addition
    println!("2. Testing calculate tool (addition):");
    let mut calc_args = HashMap::new();
    calc_args.insert("operation".to_string(), serde_json::json!("add"));
    calc_args.insert("a".to_string(), serde_json::json!(15));
    calc_args.insert("b".to_string(), serde_json::json!(27));

    match server.execute_tool("calculate", calc_args).await {
        Ok(result) => {
            println!("   Result: {}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }
    println!();

    // 3. Calculator tool - division by zero
    println!("3. Testing calculate tool (division by zero):");
    let mut calc_args2 = HashMap::new();
    calc_args2.insert("operation".to_string(), serde_json::json!("divide"));
    calc_args2.insert("a".to_string(), serde_json::json!(10));
    calc_args2.insert("b".to_string(), serde_json::json!(0));

    match server.execute_tool("calculate", calc_args2).await {
        Ok(result) => {
            println!("   Result: {}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }
    println!();

    // 4. System info tool
    println!("4. Testing system_info tool:");
    match server.execute_tool("system_info", HashMap::new()).await {
        Ok(result) => {
            println!("   Result: {}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            eprintln!("   Error: {}", e);
        }
    }
    println!();

    // Show server configuration
    println!("--- Server Configuration ---");
    let config = server.to_config();
    println!("{}", serde_json::to_string_pretty(&config)?);
    println!();

    println!("✓ SDK MCP Server example complete!");
    println!("\nNote: To use this server with ClaudeSDKClient, pass it via");
    println!("ClaudeAgentOptions.mcp_servers and add tool names to allowed_tools.");

    Ok(())
}
