/// Macros for simplified callback creation

/// Create a hook callback with simplified syntax
///
/// # Examples
///
/// ```
/// use claude::hook;
/// use claude::HookJSONOutput;
///
/// // Full signature
/// let callback = hook!(|input, tool_id, ctx| {
///     println!("Hook called: {:?}", input);
///     Ok(HookJSONOutput::default())
/// });
///
/// // Without context
/// let callback = hook!(|input, tool_id| {
///     println!("Tool: {:?}", tool_id);
///     Ok(HookJSONOutput::default())
/// });
///
/// // Just input
/// let callback = hook!(|input| {
///     println!("Input: {:?}", input);
///     Ok(HookJSONOutput::default())
/// });
/// ```
#[macro_export]
macro_rules! hook {
    (|$input:ident, $tool_id:ident, $ctx:ident| $body:block) => {
        std::sync::Arc::new(
            |$input: std::collections::HashMap<String, serde_json::Value>,
             $tool_id: Option<String>,
             $ctx: $crate::HookContext| {
                Box::pin(async move { $body })
            },
        )
    };
    (|$input:ident, $tool_id:ident| $body:block) => {
        std::sync::Arc::new(
            |$input: std::collections::HashMap<String, serde_json::Value>,
             $tool_id: Option<String>,
             _ctx: $crate::HookContext| {
                Box::pin(async move { $body })
            },
        )
    };
    (|$input:ident| $body:block) => {
        std::sync::Arc::new(
            |$input: std::collections::HashMap<String, serde_json::Value>,
             _tool_id: Option<String>,
             _ctx: $crate::HookContext| {
                Box::pin(async move { $body })
            },
        )
    };
}

/// Create a permission callback with simplified syntax
///
/// # Examples
///
/// ```
/// use claude::permission_callback;
/// use claude::PermissionResult;
///
/// // Full signature
/// let callback = permission_callback!(|tool, input, ctx| {
///     if tool == "Bash" {
///         Ok(PermissionResult::deny("Bash not allowed"))
///     } else {
///         Ok(PermissionResult::allow())
///     }
/// });
///
/// // Without context
/// let callback = permission_callback!(|tool, input| {
///     Ok(PermissionResult::allow())
/// });
///
/// // Just tool name
/// let callback = permission_callback!(|tool| {
///     if tool.starts_with("Write") {
///         Ok(PermissionResult::deny("Write operations blocked"))
///     } else {
///         Ok(PermissionResult::allow())
///     }
/// });
/// ```
#[macro_export]
macro_rules! permission_callback {
    (|$tool:ident, $input:ident, $ctx:ident| $body:block) => {
        std::sync::Arc::new(
            |$tool: String,
             $input: std::collections::HashMap<String, serde_json::Value>,
             $ctx: $crate::ToolPermissionContext| {
                Box::pin(async move { $body })
            },
        )
    };
    (|$tool:ident, $input:ident| $body:block) => {
        std::sync::Arc::new(
            |$tool: String,
             $input: std::collections::HashMap<String, serde_json::Value>,
             _ctx: $crate::ToolPermissionContext| {
                Box::pin(async move { $body })
            },
        )
    };
    (|$tool:ident| $body:block) => {
        std::sync::Arc::new(
            |$tool: String,
             _input: std::collections::HashMap<String, serde_json::Value>,
             _ctx: $crate::ToolPermissionContext| {
                Box::pin(async move { $body })
            },
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::types::{HookJSONOutput, PermissionResult};

    #[test]
    fn test_hook_macro_compiles() {
        use crate::Result;

        let _callback = hook!(|input| {
            let _ = input;
            Ok::<HookJSONOutput, crate::ClaudeSDKError>(HookJSONOutput::default())
        });

        let _callback = hook!(|input, tool_id| {
            let _ = (input, tool_id);
            Ok::<HookJSONOutput, crate::ClaudeSDKError>(HookJSONOutput::default())
        });
    }

    #[test]
    fn test_permission_callback_macro_compiles() {
        use crate::Result;

        let _callback = permission_callback!(|tool| {
            if tool == "Bash" {
                Ok::<PermissionResult, crate::ClaudeSDKError>(PermissionResult::deny("no bash".to_string()))
            } else {
                Ok(PermissionResult::allow())
            }
        });

        let _callback = permission_callback!(|tool, input| {
            let _ = (tool, input);
            Ok::<PermissionResult, crate::ClaudeSDKError>(PermissionResult::allow())
        });
    }
}
