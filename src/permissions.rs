use crate::errors::Result;
use crate::types::{PermissionResult, ToolPermissionContext};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias for async can_use_tool callback functions
///
/// A permission callback receives:
/// - tool_name: The name of the tool being requested
/// - input: The input parameters for the tool
/// - context: Permission context with suggestions
///
/// Returns a PermissionResult (Allow or Deny)
pub type CanUseToolCallback = Arc<
    dyn Fn(
            String,
            HashMap<String, serde_json::Value>,
            ToolPermissionContext,
        ) -> Pin<Box<dyn Future<Output = Result<PermissionResult>> + Send>>
        + Send
        + Sync,
>;

/// Helper functions for creating permission results
impl PermissionResult {
    /// Create an Allow result
    pub fn allow() -> Self {
        PermissionResult::Allow {
            updated_input: None,
            updated_permissions: None,
        }
    }

    /// Create an Allow result with updated input
    pub fn allow_with_input(input: HashMap<String, serde_json::Value>) -> Self {
        PermissionResult::Allow {
            updated_input: Some(input),
            updated_permissions: None,
        }
    }

    /// Create a Deny result
    pub fn deny(message: String) -> Self {
        PermissionResult::Deny {
            message,
            interrupt: false,
        }
    }

    /// Create a Deny result with interrupt
    pub fn deny_with_interrupt(message: String) -> Self {
        PermissionResult::Deny {
            message,
            interrupt: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_result_allow() {
        let result = PermissionResult::allow();
        match result {
            PermissionResult::Allow { updated_input, updated_permissions } => {
                assert!(updated_input.is_none());
                assert!(updated_permissions.is_none());
            }
            _ => panic!("Expected Allow variant"),
        }
    }

    #[test]
    fn test_permission_result_allow_with_input() {
        let mut input = HashMap::new();
        input.insert("key".to_string(), serde_json::json!("value"));

        let result = PermissionResult::allow_with_input(input.clone());
        match result {
            PermissionResult::Allow { updated_input, .. } => {
                assert!(updated_input.is_some());
                assert_eq!(updated_input.unwrap().get("key"), input.get("key"));
            }
            _ => panic!("Expected Allow variant"),
        }
    }

    #[test]
    fn test_permission_result_deny() {
        let result = PermissionResult::deny("Test denial".to_string());
        match result {
            PermissionResult::Deny { message, interrupt } => {
                assert_eq!(message, "Test denial");
                assert!(!interrupt);
            }
            _ => panic!("Expected Deny variant"),
        }
    }

    #[test]
    fn test_permission_result_deny_with_interrupt() {
        let result = PermissionResult::deny_with_interrupt("Test interrupt".to_string());
        match result {
            PermissionResult::Deny { message, interrupt } => {
                assert_eq!(message, "Test interrupt");
                assert!(interrupt);
            }
            _ => panic!("Expected Deny variant"),
        }
    }

    #[tokio::test]
    async fn test_can_use_tool_callback() {
        let callback: CanUseToolCallback = Arc::new(|tool_name, _input, _context| {
            Box::pin(async move {
                if tool_name == "Bash" {
                    Ok(PermissionResult::deny("Bash not allowed".to_string()))
                } else {
                    Ok(PermissionResult::allow())
                }
            })
        });

        let mut input = HashMap::new();
        input.insert("command".to_string(), serde_json::json!("ls"));

        let context = ToolPermissionContext {
            suggestions: vec![],
        };

        // Test denying Bash
        let result = callback("Bash".to_string(), input.clone(), context.clone()).await;
        assert!(result.is_ok());
        match result.unwrap() {
            PermissionResult::Deny { message, .. } => {
                assert_eq!(message, "Bash not allowed");
            }
            _ => panic!("Expected Deny"),
        }

        // Test allowing Read
        let result2 = callback("Read".to_string(), input, context).await;
        assert!(result2.is_ok());
        match result2.unwrap() {
            PermissionResult::Allow { .. } => {}
            _ => panic!("Expected Allow"),
        }
    }
}
