//! # Claude SDK for Rust
//!
//! A type-safe, ergonomic Rust client for the Claude Agent SDK.
//!
//! ## Quick Start
//!
//! For the simplest interactions, use the facade functions:
//!
//! ```no_run
//! use claude::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Simplest: just ask and get text
//!     let answer = ask("What is 2 + 2?").await?;
//!     println!("{}", answer);
//!
//!     // With configuration
//!     let answer = QuickQuery::new("Explain Rust ownership")
//!         .with_system_prompt("You are a Rust expert")
//!         .max_turns(1)
//!         .ask()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## API Tiers
//!
//! ### Tier 1: Facade (Beginner-Friendly)
//! - [`ask`] - Simplest function: ask a question, get text
//! - [`QuickQuery`] - Fluent builder for common configurations
//!
//! ### Tier 2: Direct APIs (Full Message Access)
//! - [`simple_query`] - One-shot queries, collect all messages
//! - [`streaming_query`] - Streaming responses for large outputs
//!
//! ### Tier 3: Advanced (Full Control)
//! - [`ClaudeSDKClient`] - Interactive sessions (when supported)
//! - [`hooks`] module - Pre-tool-use callbacks
//! - [`permissions`] module - Runtime tool permission control
//! - [`mcp`] module - Custom in-process tool servers

pub mod types;
pub mod errors;
pub mod message_parser;
pub mod transport;
pub mod simple_query;
pub mod streaming_query;
pub mod query;
pub mod client;
pub mod hooks;
pub mod permissions;
pub mod mcp_server;

// Phase 1 additions: ergonomic improvements
pub mod builders;
pub mod extensions;
pub mod facade;
#[macro_use]
pub mod macros;
pub mod prelude;

// Re-export commonly used items at crate root
pub use errors::{ClaudeSDKError, Result};
pub use types::{ClaudeAgentOptions, ContentBlock, Message, PermissionMode, SystemPromptConfig};

// Main APIs
pub use simple_query::simple_query;
pub use streaming_query::{streaming_query, StreamingQuery};
pub use client::{ClaudeSDKClient, MessageStream, ResponseStream};

// Ergonomic additions
pub use builders::ClaudeOptionsBuilder;
pub use extensions::MessageVecExt;
pub use facade::{ask, ask_with_options, QuickQuery};

// Advanced features (namespaced for clarity)
pub use hooks::{HookCallback, HookRegistry, HookMatcherConfig, HookManager};
pub use permissions::CanUseToolCallback;
pub use types::{HookContext, HookJSONOutput, ToolPermissionContext, PermissionResult};

// MCP namespace
pub mod mcp {
    pub use crate::mcp_server::{SdkMcpServer, McpTool, ToolHandler, ToolResult, ToolResultContent, ImageSource};
}

// Internal/advanced APIs
#[doc(hidden)]
pub use query::Query;
#[doc(hidden)]
pub use message_parser::*;
