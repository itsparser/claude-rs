# Claude SDK for Rust

A type-safe, high-performance Rust implementation of the [Claude Agent SDK](https://github.com/anthropics/claude-agent-sdk-python), providing seamless integration with Claude Code CLI for AI-powered automation and development workflows.

## Features

### ✅ Core Capabilities

- **Simple Queries** - Execute one-shot prompts and collect responses
- **Streaming Queries** - Process responses as they arrive with async iterators
- **Type-Safe Messages** - Complete type system for all Claude message types
- **Error Handling** - Comprehensive error types with detailed context
- **Message Parsing** - Robust JSON deserialization for all content blocks

### ✅ Advanced Features

- **Hook System** - PreToolUse callbacks for logging, safety checks, and monitoring
- **Permission Control** - Runtime tool approval/denial with `can_use_tool` callbacks
- **SDK MCP Servers** - In-process custom tools without subprocess overhead
- **Session Management** - Resume, fork, and continue conversations (types available)

### ✅ Production Quality

- **111 Passing Tests** - Comprehensive test coverage across all modules
- **Zero Unsafe Code** - Memory safe with Rust's ownership system
- **Async/Await** - Built on tokio for high-performance concurrent operations
- **Type Safety** - Compile-time guarantees prevent runtime errors

### ✅ V2 Ergonomic Improvements

- **Facade Functions** - `ask()` for one-line queries, `QuickQuery` builder for common patterns
- **Builder Pattern** - Chainable configuration with `ClaudeOptionsBuilder`
- **Extension Traits** - `MessageVecExt` for easier message handling without pattern matching
- **Simplified Macros** - `hook!` and `permission_callback!` hide complex type signatures
- **Prelude Module** - Import everything with `use claude::prelude::*;`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
claude = { git = "https://github.com/itsparser/claude-rs" }
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"
```

**Prerequisites:**
- Rust 1.70 or later
- Claude Code CLI: `npm install -g @anthropic-ai/claude-code`
- Anthropic API key configured

**Recommended:** Use the V2 API for the best developer experience. See the Quick Start section below.

## Quick Start

### Tier 1: Beginner-Friendly (1-5 lines) - **Recommended for New Users**

The simplest way to use Claude - just ask a question and get an answer:

```rust
use claude::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simplest possible usage
    let answer = ask("What is 2 + 2?").await?;
    println!("Answer: {}", answer);

    Ok(())
}
```

With system prompt and configuration using the builder:

```rust
use claude::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let answer = QuickQuery::new("Explain Rust ownership in one sentence")
        .with_system_prompt("You are a Rust expert. Be concise.")
        .max_turns(1)
        .ask()
        .await?;

    println!("{}", answer);
    Ok(())
}
```

### Tier 2: Intermediate (Full Message Access)

When you need access to complete message objects and metadata:

```rust
use claude::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get full messages with metadata
    let messages = simple_query("Explain async/await in Rust", None).await?;

    // Use extension methods for easy access
    let text = messages.text_content();  // Get all text
    let last_msg = messages.last_assistant();  // Get last assistant message

    for msg in messages.assistant_messages() {
        println!("Model: {}, Content: {:?}", msg.model, msg.content);
    }

    Ok(())
}
```

Streaming responses:

```rust
use claude::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = streaming_query("Explain quantum computing", None).await?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => println!("{:?}", message),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
```

### Tier 3: Advanced (Full Control)

For production use cases requiring hooks, permissions, and full control:

```rust
use claude::prelude::*;
use claude::{hook, permission_callback};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build complex configuration
    let options = ClaudeOptionsBuilder::new()
        .system_prompt("You are a helpful coding assistant")
        .model("claude-sonnet-4-5")
        .max_turns(10)
        .allow_tools(["Read", "Write", "Edit"])
        .build();

    let messages = simple_query("Refactor this code", Some(options)).await?;

    // Or use ClaudeSDKClient for interactive sessions (when supported)
    let safety_check = permission_callback!(|tool, _input| {
        if tool == "Bash" {
            Ok(PermissionResult::deny("Bash not allowed".to_string()))
        } else {
            Ok(PermissionResult::allow())
        }
    });

    // let mut client = ClaudeSDKClient::with_permission_callback(None, safety_check);
    // client.connect().await?;

    Ok(())
}
```

## V2 API Improvements

The V2 API represents a major ergonomic upgrade, reducing boilerplate and making the SDK more accessible. Here are the key improvements:

### 1. Facade Functions - From Complex to Simple

**Before (V1):**
```rust
use claude::simple_query;

let messages = simple_query("What is 2 + 2?", None).await?;
for msg in messages {
    if let Message::Assistant(assistant) = msg {
        for content in &assistant.content {
            if let ContentBlock::Text(text) = content {
                println!("{}", text.text);
            }
        }
    }
}
```

**After (V2):**
```rust
use claude::prelude::*;

let answer = ask("What is 2 + 2?").await?;
println!("{}", answer);
```

### 2. Builder Pattern - Chainable Configuration

**Before (V1):**
```rust
use claude::{simple_query, ClaudeAgentOptions};

let options = ClaudeAgentOptions {
    model: Some("claude-sonnet-4-5".to_string()),
    max_turns: Some(10),
    system_prompt: Some(SystemPromptConfig::Simple("You are helpful".to_string())),
    allowed_tools: vec!["Read".to_string(), "Write".to_string()],
    ..Default::default()
};

let messages = simple_query("Help me", Some(options)).await?;
```

**After (V2):**
```rust
use claude::prelude::*;

let answer = QuickQuery::new("Help me")
    .with_model("claude-sonnet-4-5")
    .max_turns(10)
    .with_system_prompt("You are helpful")
    .allow_tools(["Read", "Write"])
    .ask()
    .await?;
```

### 3. Message Extension Traits - No More Pattern Matching

**Before (V1):**
```rust
let messages = simple_query("Explain async", None).await?;

let mut text_content = String::new();
for msg in &messages {
    if let Message::Assistant(assistant) = msg {
        for content in &assistant.content {
            if let ContentBlock::Text(text) = content {
                text_content.push_str(&text.text);
            }
        }
    }
}
```

**After (V2):**
```rust
let messages = simple_query("Explain async", None).await?;

let text = messages.text_content();  // One line!
let last = messages.last_assistant();
let blocks = messages.text_blocks();
```

### 4. Simplified Callback Macros

**Before (V1):**
```rust
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

let callback: Arc<
    dyn Fn(String, serde_json::Map<String, serde_json::Value>)
        -> Pin<Box<dyn Future<Output = Result<PermissionResult, ClaudeSDKError>> + Send>>
        + Send
        + Sync
> = Arc::new(move |tool, _input| {
    Box::pin(async move {
        if tool == "Bash" {
            Ok(PermissionResult::deny("Not allowed".to_string()))
        } else {
            Ok(PermissionResult::allow())
        }
    })
});
```

**After (V2):**
```rust
use claude::permission_callback;

let callback = permission_callback!(|tool, _input| {
    if tool == "Bash" {
        Ok(PermissionResult::deny("Not allowed".to_string()))
    } else {
        Ok(PermissionResult::allow())
    }
});
```

### 5. Prelude Module - One Import, Everything You Need

**Before (V1):**
```rust
use claude::simple_query;
use claude::streaming_query::{streaming_query, StreamingQuery};
use claude::types::{ClaudeAgentOptions, Message, ContentBlock, PermissionMode};
use claude::errors::{ClaudeSDKError, Result};
use claude::client::ClaudeSDKClient;
```

**After (V2):**
```rust
use claude::prelude::*;  // That's it!
```

## Architecture

### Type System

The SDK provides a complete type hierarchy for Claude messages:

- **Message Types:** User, Assistant, System, Result, Stream
- **Content Blocks:** Text, ToolUse, ToolResult, Thinking, Image
- **Tool Results:** Success, Error, with structured data
- **Control Protocol:** Bidirectional communication types

### Transport Layer

- **Subprocess Transport** - Communicates with Claude Code CLI via stdin/stdout
- **Stream Processing** - Efficient async message parsing and routing
- **Lifecycle Management** - Automatic resource cleanup and error handling

### Error Model

```rust
pub enum ClaudeSDKError {
    CLINotFound(String),
    CLIConnectionError(String),
    ProcessError { message: String, exit_code: Option<i32>, stderr: Option<String> },
    MessageParseError { message: String, raw_json: Option<String> },
    JSONDecodeError { raw_input: String, error: String },
}
```

## Roadmap

### Phase 1: Ergonomic Improvements ✅ COMPLETE
- [x] Facade functions (`ask()`, `ask_with_options()`)
- [x] `QuickQuery` builder for common patterns
- [x] `ClaudeOptionsBuilder` with chainable API
- [x] `MessageVecExt` trait for message handling
- [x] Simplified `hook!` and `permission_callback!` macros
- [x] Prelude module (`use claude::prelude::*`)
- [x] V2 examples and documentation

### Phase 2: Core Foundation ✅ COMPLETE
- [x] Type system implementation
- [x] Subprocess transport
- [x] Message parsing
- [x] Simple & streaming queries
- [x] Error handling
- [x] Unit test coverage

### Phase 3: Advanced Features ✅ COMPLETE
- [x] Hook system (PreToolUse callbacks)
- [x] Permission callbacks (can_use_tool)
- [x] SDK MCP servers
- [x] Session types (resume, fork, continue)
- [x] Control protocol types

### Phase 4: Interactive Communication ⚠️ IN PROGRESS
- [x] ClaudeSDKClient implementation
- [x] Bidirectional message routing
- [x] Control protocol handling
- [ ] **Working interactive sessions** (blocked by CLI integration)
- [ ] Session resuming/forking runtime support
- [ ] Real-time tool permission callbacks

### Phase 5: Production Hardening 📋 PLANNED
- [ ] HTTP/WebSocket transport (alternative to subprocess)
- [ ] Connection pooling and retry logic
- [ ] Metrics and observability hooks
- [ ] Rate limiting and backpressure
- [ ] Comprehensive integration tests

### Phase 6: Developer Experience 📋 PLANNED
- [ ] CLI tool for SDK management
- [ ] Project templates and scaffolding
- [ ] Interactive examples and tutorials
- [ ] Performance benchmarks
- [ ] Migration guides from Python SDK

### Phase 7: Ecosystem Integration 📋 PLANNED
- [ ] Actix-web middleware
- [ ] Axum integration
- [ ] Tokio-console support
- [ ] OpenTelemetry instrumentation
- [ ] Docker and Kubernetes examples

## Current Limitations

### Interactive Mode
The `ClaudeSDKClient` for persistent bidirectional communication is implemented but not yet functional due to Claude Code CLI integration challenges with `--input-format stream-json`.

**Workaround:** Use `simple_query()` for each interaction. Each query is independent but fully functional.

**Status:** Investigating alternative transport mechanisms (HTTP/WebSocket) for interactive sessions.

### Features Blocked by Interactive Mode
- Session resuming/forking (types exist, runtime pending)
- Runtime permission callbacks (types exist, runtime pending)
- Persistent conversation context (types exist, runtime pending)

## Performance

- **Memory Efficient:** O(1) memory usage with streaming queries
- **Zero-Cost Abstractions:** No runtime overhead from type safety
- **Concurrent:** Fully async with tokio, supports parallel operations
- **Fast Compilation:** Minimal dependencies, clean module structure

## Use Cases

**Perfect For:**
- 🔧 Automation scripts and CI/CD pipelines
- 📦 Batch processing and data transformation
- 🤖 Code generation and analysis tools
- 📊 Report generation and summarization
- 🧪 Testing and validation workflows

**Coming Soon:**
- 💬 Interactive chat applications
- 🔄 Stateful conversation systems
- 🎛️ Real-time tool integration
- 📱 Web service integrations

## Documentation

- [TESTING_STATUS.md](TESTING_STATUS.md) - Test results and verification
- [FINAL_STATUS.md](FINAL_STATUS.md) - Complete implementation details
- [COMPLETION_REPORT.md](COMPLETION_REPORT.md) - Feature inventory
- [CLAUDE_CLI_ISSUE.md](CLAUDE_CLI_ISSUE.md) - Known issues and workarounds

## Testing

```bash
# Run all unit tests
cargo test

# Run V2 API examples (recommended)
cargo run --example v2_simple_ask
cargo run --example v2_message_extensions
cargo run --example v2_simplified_callbacks

# Run working examples
cargo run --example simple_real_query
cargo run --example streaming_query_example

# Run standalone examples (no CLI needed)
cargo run --example verify_library
cargo run --example types_demo
cargo run --example hooks_example
cargo run --example sdk_mcp_example
```

## Contributing

Contributions are welcome! Areas of focus:
- Alternative transport implementations (HTTP/WebSocket)
- Integration tests with real Claude Code CLI
- Performance optimizations
- Documentation improvements
- Example applications

## License

MIT

## Acknowledgments

Based on the official [Claude Agent SDK for Python](https://github.com/anthropics/claude-agent-sdk-python) by Anthropic.
