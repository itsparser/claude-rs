# Rust SDK Developer Experience Analysis

## Executive Summary

This analysis identifies key pain points in the current Claude Rust SDK and proposes concrete improvements following Rust best practices. The SDK currently exposes three different APIs (`simple_query`, `streaming_query`, `ClaudeSDKClient`) with overlapping use cases, verbose configuration, and complex callback types that create cognitive overhead for developers.

---

## 1. Current Pain Points

### 1.1 API Ergonomics Issues

#### Problem: Confusing API Surface with Multiple Entry Points

**Current State:**
```rust
// Three different APIs with unclear distinctions:

// 1. simple_query - collects all messages
let messages = simple_query("prompt", None).await?;

// 2. streaming_query - returns stream
let mut stream = streaming_query("prompt", None).await?;

// 3. ClaudeSDKClient - full featured
let mut client = ClaudeSDKClient::new(None);
client.connect().await?;
client.query("prompt", None).await?;
```

**Issues:**
- Unclear when to use which API
- `simple_query` and `streaming_query` have almost identical signatures
- No clear "default" or "recommended" path
- Naming doesn't convey capability differences well

#### Problem: Verbose Configuration with Many Optional Fields

**Current State:**
```rust
#[derive(Debug, Clone, Default)]
pub struct ClaudeAgentOptions {
    pub allowed_tools: Vec<String>,
    pub system_prompt: Option<SystemPromptConfig>,
    pub mcp_servers: HashMap<String, McpServerConfig>,
    pub permission_mode: Option<PermissionMode>,
    pub continue_conversation: bool,
    pub resume: Option<String>,
    pub max_turns: Option<i32>,
    pub disallowed_tools: Vec<String>,
    pub model: Option<String>,
    pub permission_prompt_tool_name: Option<String>,
    pub cwd: Option<PathBuf>,
    pub settings: Option<String>,
    pub add_dirs: Vec<PathBuf>,
    pub env: HashMap<String, String>,
    pub extra_args: HashMap<String, Option<String>>,
    pub max_buffer_size: Option<usize>,
    pub user: Option<String>,
    pub include_partial_messages: bool,
    pub fork_session: bool,
    pub agents: Option<HashMap<String, AgentDefinition>>,
    pub setting_sources: Option<Vec<SettingSource>>,
}

// Usage is verbose
let mut options = ClaudeAgentOptions::default();
options.system_prompt = Some(SystemPromptConfig::Text("...".to_string()));
options.max_turns = Some(1);
options.model = Some("claude-sonnet-4-5".to_string());
```

**Issues:**
- 21 fields, many optional
- No builder pattern
- Field names mix styles (`allowed_tools` vs `continue_conversation`)
- Requires understanding entire configuration space upfront
- No validation until runtime

#### Problem: Complex Callback Types

**Current State:**
```rust
// Hook callback - extremely verbose
pub type HookCallback = Arc<
    dyn Fn(
            HashMap<String, serde_json::Value>,
            Option<String>,
            HookContext,
        ) -> Pin<Box<dyn Future<Output = Result<HookJSONOutput>> + Send>>
        + Send
        + Sync,
>;

// Permission callback - equally complex
pub type CanUseToolCallback = Arc<
    dyn Fn(
            String,
            HashMap<String, serde_json::Value>,
            ToolPermissionContext,
        ) -> Pin<Box<dyn Future<Output = Result<PermissionResult>> + Send>>
        + Send
        + Sync,
>;

// Usage is intimidating
let callback: HookCallback = Arc::new(|input, tool_id, ctx| {
    Box::pin(async move {
        // Implementation
        Ok(HookJSONOutput::default())
    })
});
```

**Issues:**
- Overwhelming type signature for users
- Forces understanding of `Arc`, `Pin`, `Box`, async traits
- No helper macros or builder functions
- Copy-paste-modify pattern in examples

### 1.2 Type System Issues

#### Problem: No Type Aliases for Common Result Types

**Current State:**
```rust
// Every function returns this
pub async fn simple_query(
    prompt: &str,
    options: Option<ClaudeAgentOptions>,
) -> Result<Vec<Message>> { ... }

// Where Result is just:
pub type Result<T> = std::result::Result<T, ClaudeSDKError>;

// Users see:
let messages: Result<Vec<Message>> = simple_query(...).await;
```

**Issues:**
- No specialized result types for common operations
- `Result<Vec<Message>>` appears everywhere but isn't aliased
- Missing semantic types like `QueryResult`, `StreamResult`

#### Problem: Verbose Error Handling

**Current State:**
```rust
#[derive(Debug)]
pub enum ClaudeSDKError {
    CLIConnectionError(String),
    CLINotFoundError { message: String, cli_path: Option<String> },
    ProcessError { message: String, exit_code: Option<i32>, stderr: Option<String> },
    CLIJSONDecodeError { line: String, original_error: String },
    MessageParseError { message: String, data: Option<serde_json::Value> },
}
```

**Issues:**
- Constructor functions exist but aren't ergonomic
- No error context chaining (no `anyhow` or `thiserror` integration)
- Users must match on all variants even when they just want error message

#### Problem: Complex Generic Bounds on Async Traits

**Current State:**
```rust
// In query.rs - many constructors with overlapping functionality
impl Query {
    pub fn new(transport: SubprocessTransport, is_streaming_mode: bool) -> Self { ... }

    pub fn with_hooks(
        transport: SubprocessTransport,
        is_streaming_mode: bool,
        hook_manager: HookManager,
    ) -> Self { ... }

    pub fn with_can_use_tool(
        transport: SubprocessTransport,
        is_streaming_mode: bool,
        can_use_tool: CanUseToolCallback,
    ) -> Self { ... }

    pub fn with_mcp_servers(
        transport: SubprocessTransport,
        is_streaming_mode: bool,
        mcp_servers: HashMap<String, SdkMcpServer>,
    ) -> Self { ... }

    pub fn with_options(
        transport: SubprocessTransport,
        is_streaming_mode: bool,
        can_use_tool: Option<CanUseToolCallback>,
        mcp_servers: Option<HashMap<String, SdkMcpServer>>,
    ) -> Self { ... }
}
```

**Issues:**
- Constructor explosion without builder pattern
- Difficult to add new optional configuration
- No clear "recommended" constructor

### 1.3 Developer Experience Issues

#### Problem: Unclear API Selection

**From Examples:**
```rust
// simple_real_query.rs - when to use this?
let messages = simple_query("What is 2 + 2?", None).await?;

// streaming_query_example.rs - when to use this instead?
let mut stream = streaming_query("What is 2 + 2?", None).await?;

// interactive_chat.rs - when to use this?
let mut client = ClaudeSDKClient::new(Some(options));
client.connect().await?;
client.query("What is 2 + 2?", None).await?;
```

**Issues:**
- Documentation doesn't clearly guide users
- All three can do similar things
- Performance implications not documented
- Migration path between APIs unclear

#### Problem: Missing Convenience Methods

**Current State:**
```rust
// To send a query with custom system prompt:
let mut options = ClaudeAgentOptions::default();
options.system_prompt = Some(SystemPromptConfig::Text("...".to_string()));
options.max_turns = Some(1);
let messages = simple_query("prompt", Some(options)).await?;

// To extract just text from messages:
for message in messages {
    match message {
        Message::Assistant(msg) => {
            for block in msg.content {
                match block {
                    ContentBlock::Text { text } => println!("{}", text),
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
```

**Issues:**
- No helper methods like `messages.assistant_text()` or `messages.latest_response()`
- No quick constructors like `ClaudeAgentOptions::with_system_prompt(s)`
- Common patterns require boilerplate

#### Problem: No Prelude Module

**Current State:**
```rust
// Users must know what to import
use claude::{
    simple_query,
    ClaudeAgentOptions,
    ContentBlock,
    Message,
    SystemPromptConfig
};
```

**Issues:**
- No `claude::prelude::*` for common imports
- Not clear what's essential vs advanced
- Examples show different import styles

#### Problem: Callback Setup is Complex

**From examples/hooks_example.rs:**
```rust
let callback: HookCallback = Arc::new(|input_data, tool_use_id, _context| {
    Box::pin(async move {
        println!("Hook called with input: {:?}", input_data);
        Ok(HookJSONOutput {
            decision: Some("allow".to_string()),
            system_message: Some("Hook executed successfully".to_string()),
            hook_specific_output: None,
        })
    })
});
```

**Issues:**
- Requires understanding `Arc::new`, `Box::pin`, async closures
- No helper macros like `hook!` or `permission_callback!`
- Examples use raw construction everywhere

### 1.4 Architecture Issues

#### Problem: Direct Exposure of Transport Layer

**Current State:**
```rust
// In simple_query.rs
pub async fn simple_query(
    prompt: &str,
    options: Option<ClaudeAgentOptions>,
) -> Result<Vec<Message>> {
    let opts = options.unwrap_or_default();
    let mut transport = SubprocessTransport::new(prompt.to_string(), opts);
    transport.connect().await?;
    transport.end_input().await?;
    // ...
}
```

**Issues:**
- Transport implementation details leak into high-level APIs
- No abstraction layer for testing
- Hard to add alternative transports (HTTP, WebSocket)

#### Problem: No Clear Separation Between Low-Level and High-Level APIs

**Current State:**
```rust
// lib.rs exports everything at the same level
pub use types::*;           // ~100 types
pub use errors::*;          // All errors
pub use message_parser::*;  // Parser functions
pub use simple_query::simple_query;
pub use streaming_query::{streaming_query, StreamingQuery};
pub use query::Query;       // Internal control protocol
pub use client::{ClaudeSDKClient, MessageStream, ResponseStream};
pub use hooks::{HookCallback, HookRegistry, HookMatcherConfig, HookManager};
```

**Issues:**
- No modules separating beginner vs advanced APIs
- `Query` is internal but public
- All types in global namespace

#### Problem: Missing Facade Pattern for Common Operations

**Current Need:**
```rust
// Common pattern: ask a question, get text response
// Currently requires:
let messages = simple_query("What is 2+2?", None).await?;
let text = messages.iter()
    .filter_map(|m| match m {
        Message::Assistant(msg) => Some(msg.content.iter()
            .filter_map(|b| match b {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None
            })
            .collect::<Vec<_>>()
            .join("\n")),
        _ => None
    })
    .collect::<Vec<_>>()
    .join("\n");

// Should be:
let text = claude::ask("What is 2+2?").await?;
```

---

## 2. Proposed Improvements

### 2.1 Builder Pattern for Configuration

**Recommendation:** Implement a fluent builder API

```rust
// New builders/options.rs
#[derive(Default)]
pub struct ClaudeOptionsBuilder {
    inner: ClaudeAgentOptions,
}

impl ClaudeOptionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.inner.system_prompt = Some(SystemPromptConfig::Text(prompt.into()));
        self
    }

    pub fn system_prompt_preset(mut self, preset: &str) -> Self {
        self.inner.system_prompt = Some(SystemPromptConfig::Preset(SystemPromptPreset {
            r#type: "preset".to_string(),
            preset: preset.to_string(),
            append: None,
        }));
        self
    }

    pub fn max_turns(mut self, turns: i32) -> Self {
        self.inner.max_turns = Some(turns);
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.inner.model = Some(model.into());
        self
    }

    pub fn permission_mode(mut self, mode: PermissionMode) -> Self {
        self.inner.permission_mode = Some(mode);
        self
    }

    pub fn allow_tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.inner.allowed_tools = tools.into_iter().map(Into::into).collect();
        self
    }

    pub fn deny_tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.inner.disallowed_tools = tools.into_iter().map(Into::into).collect();
        self
    }

    pub fn cwd(mut self, path: impl Into<PathBuf>) -> Self {
        self.inner.cwd = Some(path.into());
        self
    }

    pub fn add_directory(mut self, dir: impl Into<PathBuf>) -> Self {
        self.inner.add_dirs.push(dir.into());
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.env.insert(key.into(), value.into());
        self
    }

    pub fn continue_conversation(mut self, enable: bool) -> Self {
        self.inner.continue_conversation = enable;
        self
    }

    pub fn resume_session(mut self, session_id: impl Into<String>) -> Self {
        self.inner.resume = Some(session_id.into());
        self
    }

    pub fn fork_session(mut self, session_id: impl Into<String>) -> Self {
        self.inner.resume = Some(session_id.into());
        self.inner.fork_session = true;
        self
    }

    pub fn build(self) -> ClaudeAgentOptions {
        self.inner
    }
}

// Add to ClaudeAgentOptions
impl ClaudeAgentOptions {
    pub fn builder() -> ClaudeOptionsBuilder {
        ClaudeOptionsBuilder::new()
    }
}

// Usage:
let options = ClaudeAgentOptions::builder()
    .system_prompt("You are a helpful assistant")
    .max_turns(5)
    .model("claude-sonnet-4-5")
    .permission_mode(PermissionMode::AcceptEdits)
    .allow_tools(["Read", "Write", "Bash"])
    .cwd("/path/to/project")
    .build();
```

**Benefits:**
- Discoverable through IDE autocomplete
- Chainable, readable configuration
- Type-safe
- Easy to extend
- Clear separation of concerns

### 2.2 Simplified Callback Helpers

**Recommendation:** Provide macro and builder functions for callbacks

```rust
// New macros/callbacks.rs
#[macro_export]
macro_rules! hook {
    (|$input:ident, $tool_id:ident, $ctx:ident| $body:block) => {
        std::sync::Arc::new(
            |$input: std::collections::HashMap<String, serde_json::Value>,
             $tool_id: Option<String>,
             $ctx: $crate::HookContext| {
                Box::pin(async move { $body })
            }
        )
    };
    (|$input:ident, $tool_id:ident| $body:block) => {
        std::sync::Arc::new(
            |$input: std::collections::HashMap<String, serde_json::Value>,
             $tool_id: Option<String>,
             _ctx: $crate::HookContext| {
                Box::pin(async move { $body })
            }
        )
    };
    (|$input:ident| $body:block) => {
        std::sync::Arc::new(
            |$input: std::collections::HashMap<String, serde_json::Value>,
             _tool_id: Option<String>,
             _ctx: $crate::HookContext| {
                Box::pin(async move { $body })
            }
        )
    };
}

#[macro_export]
macro_rules! permission_callback {
    (|$tool:ident, $input:ident, $ctx:ident| $body:block) => {
        std::sync::Arc::new(
            |$tool: String,
             $input: std::collections::HashMap<String, serde_json::Value>,
             $ctx: $crate::ToolPermissionContext| {
                Box::pin(async move { $body })
            }
        )
    };
    (|$tool:ident, $input:ident| $body:block) => {
        std::sync::Arc::new(
            |$tool: String,
             $input: std::collections::HashMap<String, serde_json::Value>,
             _ctx: $crate::ToolPermissionContext| {
                Box::pin(async move { $body })
            }
        )
    };
}

// Usage:
let callback = hook!(|input, tool_id| {
    println!("Hook called: {:?}", input);
    Ok(HookJSONOutput::default())
});

let permission = permission_callback!(|tool, input| {
    if tool == "Bash" {
        Ok(PermissionResult::deny("Bash not allowed".to_string()))
    } else {
        Ok(PermissionResult::allow())
    }
});
```

**Benefits:**
- Hides complex type signatures
- Familiar syntax for Rust developers
- Reduces cognitive load
- Makes examples clearer

### 2.3 Type Aliases and Semantic Types

**Recommendation:** Add semantic type aliases

```rust
// New types/aliases.rs
pub type QueryResult = Result<Vec<Message>>;
pub type StreamResult = Result<impl Stream<Item = Result<Message>>>;
pub type TextResponse = Result<String>;
pub type MessageResult = Result<Message>;

// Enhanced Message type with helper methods
impl Message {
    pub fn as_assistant(&self) -> Option<&AssistantMessage> {
        match self {
            Message::Assistant(msg) => Some(msg),
            _ => None,
        }
    }

    pub fn as_result(&self) -> Option<&ResultMessage> {
        match self {
            Message::Result(msg) => Some(msg),
            _ => None,
        }
    }

    pub fn is_assistant(&self) -> bool {
        matches!(self, Message::Assistant(_))
    }

    pub fn is_result(&self) -> bool {
        matches!(self, Message::Result(_))
    }
}

// Enhanced Vec<Message> operations
pub trait MessageVecExt {
    fn assistant_messages(&self) -> Vec<&AssistantMessage>;
    fn text_content(&self) -> String;
    fn last_assistant(&self) -> Option<&AssistantMessage>;
    fn result_message(&self) -> Option<&ResultMessage>;
}

impl MessageVecExt for Vec<Message> {
    fn assistant_messages(&self) -> Vec<&AssistantMessage> {
        self.iter()
            .filter_map(|m| m.as_assistant())
            .collect()
    }

    fn text_content(&self) -> String {
        self.iter()
            .filter_map(|m| m.as_assistant())
            .flat_map(|msg| &msg.content)
            .filter_map(|block| match block {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn last_assistant(&self) -> Option<&AssistantMessage> {
        self.iter().rev().find_map(|m| m.as_assistant())
    }

    fn result_message(&self) -> Option<&ResultMessage> {
        self.iter().find_map(|m| m.as_result())
    }
}

// Usage:
let messages = simple_query("What is 2+2?", None).await?;
let text = messages.text_content();  // Much cleaner!
let last = messages.last_assistant();
```

### 2.4 Prelude Module

**Recommendation:** Add a prelude for common imports

```rust
// New prelude.rs
pub use crate::{
    // Main APIs
    simple_query,
    streaming_query,
    ClaudeSDKClient,

    // Core types
    Message,
    ContentBlock,
    ClaudeAgentOptions,

    // Builders
    ClaudeOptionsBuilder,

    // Error handling
    Result,
    ClaudeSDKError,

    // Extension traits
    MessageVecExt,

    // Permission types
    PermissionMode,
    PermissionResult,

    // Common enums
    SystemPromptConfig,
};

// Usage in user code:
use claude::prelude::*;

// Or for advanced users:
use claude::{
    prelude::*,
    hooks::{HookCallback, HookManager},
    permissions::CanUseToolCallback,
};
```

### 2.5 Facade Pattern for Common Operations

**Recommendation:** Add high-level convenience functions

```rust
// New facade.rs
use crate::prelude::*;

/// Ask Claude a simple question and get text response
///
/// This is the simplest way to interact with Claude.
/// For more control, use `simple_query` or `ClaudeSDKClient`.
///
/// # Example
/// ```no_run
/// use claude::ask;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let answer = ask("What is 2 + 2?").await?;
///     println!("{}", answer);
///     Ok(())
/// }
/// ```
pub async fn ask(prompt: impl AsRef<str>) -> Result<String> {
    let messages = simple_query(prompt.as_ref(), None).await?;
    Ok(messages.text_content())
}

/// Ask Claude with custom options
pub async fn ask_with_options(
    prompt: impl AsRef<str>,
    options: ClaudeAgentOptions,
) -> Result<String> {
    let messages = simple_query(prompt.as_ref(), Some(options)).await?;
    Ok(messages.text_content())
}

/// Quick query builder
pub struct QuickQuery {
    prompt: String,
    options: ClaudeOptionsBuilder,
}

impl QuickQuery {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            options: ClaudeOptionsBuilder::new(),
        }
    }

    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.options = self.options.system_prompt(prompt);
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.options = self.options.model(model);
        self
    }

    pub fn max_turns(mut self, turns: i32) -> Self {
        self.options = self.options.max_turns(turns);
        self
    }

    pub async fn ask(self) -> Result<String> {
        ask_with_options(self.prompt, self.options.build()).await
    }

    pub async fn query(self) -> QueryResult {
        simple_query(&self.prompt, Some(self.options.build())).await
    }

    pub async fn stream(self) -> StreamResult {
        streaming_query(&self.prompt, Some(self.options.build())).await
    }
}

// Usage:
let answer = ask("What is Rust?").await?;

let answer = QuickQuery::new("Explain monads")
    .with_system_prompt("You are a Haskell expert")
    .max_turns(1)
    .ask()
    .await?;
```

### 2.6 Improved Error Handling

**Recommendation:** Use `thiserror` for better errors

```rust
// Update errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClaudeSDKError {
    #[error("Failed to connect to Claude CLI: {0}")]
    CLIConnectionError(String),

    #[error("Claude CLI not found{}", cli_path.as_ref().map(|p| format!(": {}", p)).unwrap_or_default())]
    CLINotFoundError {
        #[source]
        message: String,
        cli_path: Option<String>,
    },

    #[error("Process failed: {message}{}", exit_code.map(|c| format!(" (exit code: {})", c)).unwrap_or_default())]
    ProcessError {
        message: String,
        exit_code: Option<i32>,
        stderr: Option<String>,
    },

    #[error("Failed to decode JSON: {original_error}")]
    CLIJSONDecodeError {
        line: String,
        #[source]
        original_error: String,
    },

    #[error("Failed to parse message: {message}")]
    MessageParseError {
        message: String,
        data: Option<serde_json::Value>,
    },
}

// Benefits:
// - Automatic Display implementation
// - Source error chaining
// - Better error messages out of the box
```

### 2.7 Clear API Hierarchy

**Recommendation:** Reorganize module structure

```rust
// New lib.rs structure
pub mod prelude;

// Core high-level APIs (beginner-friendly)
pub use simple_query::simple_query;
pub use streaming_query::streaming_query;
pub use client::ClaudeSDKClient;
pub use facade::{ask, ask_with_options, QuickQuery};

// Configuration
pub use types::{
    ClaudeAgentOptions,
    PermissionMode,
    SystemPromptConfig,
    Message,
    ContentBlock,
};
pub use builders::ClaudeOptionsBuilder;

// Error handling
pub use errors::{ClaudeSDKError, Result};

// Extension traits
pub use extensions::MessageVecExt;

// Advanced APIs (explicitly namespaced)
pub mod hooks {
    pub use crate::hooks_impl::{
        HookCallback,
        HookRegistry,
        HookManager,
        HookMatcherConfig,
    };
}

pub mod permissions {
    pub use crate::permissions_impl::{
        CanUseToolCallback,
        ToolPermissionContext,
        PermissionResult,
    };
}

pub mod mcp {
    pub use crate::mcp_server::{
        SdkMcpServer,
        McpTool,
        ToolHandler,
    };
}

// Internal APIs (not in docs)
#[doc(hidden)]
pub mod internal {
    pub use crate::query::Query;
    pub use crate::transport::{Transport, SubprocessTransport};
    pub use crate::message_parser;
}

// Re-exports of all types for advanced users
pub mod types {
    pub use crate::types_impl::*;
}
```

### 2.8 Type State Pattern for Client

**Recommendation:** Use type states to enforce correct usage

```rust
// New client with type states
pub struct Disconnected;
pub struct Connected;

pub struct ClaudeClient<State = Disconnected> {
    options: ClaudeAgentOptions,
    query: Option<Query>,
    can_use_tool: Option<CanUseToolCallback>,
    _state: PhantomData<State>,
}

impl ClaudeClient<Disconnected> {
    pub fn new() -> Self {
        Self::with_options(ClaudeAgentOptions::default())
    }

    pub fn with_options(options: ClaudeAgentOptions) -> Self {
        Self {
            options,
            query: None,
            can_use_tool: None,
            _state: PhantomData,
        }
    }

    pub fn builder() -> ClaudeClientBuilder {
        ClaudeClientBuilder::new()
    }

    pub async fn connect(self) -> Result<ClaudeClient<Connected>> {
        // Connection logic...
        Ok(ClaudeClient {
            options: self.options,
            query: Some(query),
            can_use_tool: self.can_use_tool,
            _state: PhantomData,
        })
    }
}

impl ClaudeClient<Connected> {
    // These methods only available on connected client
    pub async fn query(&mut self, prompt: &str) -> Result<()> { ... }
    pub fn receive_messages(&mut self) -> MessageStream { ... }
    pub async fn interrupt(&mut self) -> Result<()> { ... }

    pub async fn disconnect(self) -> Result<ClaudeClient<Disconnected>> {
        // Cleanup...
        Ok(ClaudeClient {
            options: self.options,
            query: None,
            can_use_tool: self.can_use_tool,
            _state: PhantomData,
        })
    }
}

// Prevents compile-time errors:
// let client = ClaudeClient::new();
// client.query("hello").await?;  // ERROR: method not found
//
// let client = ClaudeClient::new().connect().await?;
// client.query("hello").await?;  // OK!
```

---

## 3. Specific Design Patterns to Apply

### 3.1 Builder Pattern
- **Where:** `ClaudeAgentOptions`, `HookMatcherConfig`, client construction
- **Why:** Fluent, discoverable API for complex configuration
- **Priority:** High

### 3.2 Facade Pattern
- **Where:** New `facade.rs` module with `ask()`, `QuickQuery`
- **Why:** Simple entry point for common use cases
- **Priority:** High

### 3.3 Extension Trait Pattern
- **Where:** `MessageVecExt` for Vec<Message> helpers
- **Why:** Add methods without modifying core types
- **Priority:** Medium

### 3.4 Type State Pattern
- **Where:** `ClaudeClient` to enforce connection state
- **Why:** Compile-time safety, prevents runtime errors
- **Priority:** Medium

### 3.5 Newtype Pattern
- **Where:** Wrap complex callbacks in semantic types
- **Why:** Better error messages, hide complexity
- **Priority:** Low

### 3.6 Strategy Pattern
- **Where:** Transport layer abstraction
- **Why:** Enable testing, alternative transports
- **Priority:** Low

---

## 4. API Design Recommendations

### 4.1 API Layers

Organize into three clear tiers:

**Tier 1: Beginner (Facade)**
```rust
use claude::prelude::*;

// Simplest possible
let answer = ask("What is Rust?").await?;

// With basic config
let answer = QuickQuery::new("Explain async")
    .with_system_prompt("You are a teacher")
    .ask()
    .await?;
```

**Tier 2: Intermediate (Direct APIs)**
```rust
use claude::prelude::*;

// Full message access
let messages = simple_query("What is 2+2?", None).await?;
let text = messages.text_content();

// Streaming for large responses
let mut stream = streaming_query("Explain...", None).await?;
while let Some(msg) = stream.next().await {
    // Process incrementally
}
```

**Tier 3: Advanced (Full Control)**
```rust
use claude::prelude::*;
use claude::hooks;
use claude::permissions;

// Interactive sessions
let client = ClaudeClient::builder()
    .options(ClaudeAgentOptions::builder()
        .model("claude-sonnet-4-5")
        .build())
    .permission_callback(permission_callback!(|tool, input| {
        // Custom logic
        Ok(PermissionResult::allow())
    }))
    .build();

let mut client = client.connect().await?;
client.query("first question", None).await?;
// ... interactive flow
```

### 4.2 Migration Path

Provide clear upgrade path:

```rust
// Step 1: Start simple
let answer = ask("question").await?;

// Step 2: Need full messages? Easy upgrade
let messages = simple_query("question", None).await?;

// Step 3: Need streaming? Similar API
let stream = streaming_query("question", None).await?;

// Step 4: Need interactivity? Different but documented
let mut client = ClaudeClient::new().connect().await?;
```

### 4.3 Documentation Strategy

**For each API level:**

1. **Facade functions:** Emphasized in README, shown first
2. **Direct APIs:** "If you need more control..." section
3. **Advanced APIs:** Separate "Advanced Usage" guide

**Naming conventions:**
- Simple: `ask()`, `query()`
- Advanced: `ClaudeClient`, `HookManager`
- Internal: Prefix with `_` or in `internal` module

### 4.4 Example Improvements

**Before (current):**
```rust
let mut options = ClaudeAgentOptions::default();
options.system_prompt = Some(SystemPromptConfig::Text("...".to_string()));
options.max_turns = Some(1);

let messages = simple_query("What is Rust?", Some(options)).await?;

for message in messages {
    match message {
        Message::Assistant(msg) => {
            for block in msg.content {
                match block {
                    ContentBlock::Text { text } => println!("{}", text),
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
```

**After (proposed):**
```rust
let answer = QuickQuery::new("What is Rust?")
    .with_system_prompt("You are a programming expert")
    .max_turns(1)
    .ask()
    .await?;

println!("{}", answer);
```

---

## 5. Implementation Roadmap

### Phase 1: Foundation (No Breaking Changes)
1. Add `ClaudeOptionsBuilder` alongside existing struct
2. Add `MessageVecExt` trait
3. Add facade functions (`ask`, `QuickQuery`)
4. Add prelude module
5. Add callback macros

**Deliverable:** Improved ergonomics while maintaining compatibility

### Phase 2: Enhanced Types
1. Add semantic type aliases
2. Add helper methods to core types
3. Improve error types with `thiserror`
4. Add builder for `HookMatcherConfig`

**Deliverable:** Better type system support

### Phase 3: Restructure (Breaking Changes)
1. Reorganize module hierarchy
2. Rename APIs for consistency
3. Implement type state pattern for client
4. Hide internal APIs

**Deliverable:** Clean, hierarchical API

### Phase 4: Advanced Features
1. Abstract transport layer
2. Add mock transport for testing
3. Add more convenience methods
4. Comprehensive examples and guides

**Deliverable:** Production-ready SDK

---

## 6. Success Metrics

**Developer Experience:**
- Time to first successful query: < 5 minutes
- Lines of code for common tasks: < 10
- Number of imports needed: < 5

**API Quality:**
- Compile-time error rate: > 80% (vs runtime)
- API discovery through IDE: > 90% of features
- Breaking changes per release: < 3

**Documentation:**
- Example coverage: 100% of public APIs
- Common use case examples: > 20
- Migration guide completeness: 100%

---

## 7. Conclusion

The current SDK is functionally complete but has significant developer experience issues. The proposed improvements focus on:

1. **Progressive disclosure:** Simple tasks are simple, complex tasks are possible
2. **Type safety:** Leverage Rust's type system for better errors
3. **Discoverability:** Clear API hierarchy and good defaults
4. **Familiarity:** Follow Rust ecosystem conventions

By implementing these changes in phases, we can significantly improve the SDK while maintaining backward compatibility during the transition.

---

## Appendix: Quick Reference

### Current Pain Points Summary
1. Three confusing entry points
2. 21-field configuration struct with no builder
3. Complex callback type signatures
4. No prelude or convenience methods
5. Flat module structure
6. Direct transport exposure

### Proposed Solutions Summary
1. Clear 3-tier API: Facade → Direct → Advanced
2. Builder pattern for all complex types
3. Macros for callback creation
4. Prelude module + extension traits
5. Hierarchical modules
6. Type state pattern for safety
7. `ask()` function for simplest case

### Priority Implementation Order
1. **High:** Builder, Prelude, Facade functions
2. **Medium:** Extension traits, Type aliases, Module reorganization
3. **Low:** Type states, Transport abstraction
