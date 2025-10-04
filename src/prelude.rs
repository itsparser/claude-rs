/// Prelude module with commonly used imports
///
/// Import everything you need to get started:
/// ```
/// use claude::prelude::*;
/// ```

// Main APIs
pub use crate::simple_query::simple_query;
pub use crate::streaming_query::{streaming_query, StreamingQuery};
pub use crate::client::ClaudeSDKClient;

// Facade (simple entry points)
pub use crate::facade::{ask, ask_with_options, QuickQuery};

// Core types
pub use crate::types::{
    ClaudeAgentOptions, ContentBlock, Message, PermissionMode, SystemPromptConfig,
};

// Builders
pub use crate::builders::ClaudeOptionsBuilder;

// Error handling
pub use crate::errors::{ClaudeSDKError, Result};

// Extension traits
pub use crate::extensions::MessageVecExt;

// Macros (re-exported at crate root, but mentioned here for discovery)
#[doc(inline)]
pub use crate::{hook, permission_callback};
