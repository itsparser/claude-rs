use crate::errors::Result;
use crate::types::{HookContext, HookJSONOutput};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias for async hook callback functions
///
/// A hook callback receives:
/// - input_data: The data from the hook event (e.g., tool use parameters)
/// - tool_use_id: Optional tool use identifier
/// - context: Hook execution context
///
/// Returns a HookJSONOutput with optional hook-specific output
pub type HookCallback = Arc<
    dyn Fn(
            HashMap<String, serde_json::Value>,
            Option<String>,
            HookContext,
        ) -> Pin<Box<dyn Future<Output = Result<HookJSONOutput>> + Send>>
        + Send
        + Sync,
>;

/// Stores registered hook callbacks with their IDs
pub struct HookRegistry {
    callbacks: HashMap<String, HookCallback>,
    next_id: u64,
}

impl HookRegistry {
    /// Create a new empty hook registry
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
            next_id: 0,
        }
    }

    /// Register a new hook callback and return its ID
    pub fn register(&mut self, callback: HookCallback) -> String {
        let id = format!("hook_{}", self.next_id);
        self.next_id += 1;
        self.callbacks.insert(id.clone(), callback);
        id
    }

    /// Get a callback by ID
    pub fn get(&self, id: &str) -> Option<&HookCallback> {
        self.callbacks.get(id)
    }

    /// Remove a callback by ID
    pub fn unregister(&mut self, id: &str) -> Option<HookCallback> {
        self.callbacks.remove(id)
    }

    /// Check if a callback exists
    pub fn contains(&self, id: &str) -> bool {
        self.callbacks.contains_key(id)
    }

    /// Get the number of registered callbacks
    pub fn len(&self) -> usize {
        self.callbacks.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.callbacks.is_empty()
    }
}

impl Default for HookRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for a hook matcher
///
/// Associates a tool name pattern with hook callbacks
#[derive(Clone)]
pub struct HookMatcherConfig {
    /// Pattern to match against tool names (e.g., "Bash", "Read", "*")
    pub matcher: String,
    /// List of callback IDs to execute when matched
    pub callback_ids: Vec<String>,
}

impl HookMatcherConfig {
    /// Create a new hook matcher configuration
    pub fn new(matcher: String, callback_ids: Vec<String>) -> Self {
        Self {
            matcher,
            callback_ids,
        }
    }

    /// Check if this matcher matches the given tool name
    pub fn matches(&self, tool_name: &str) -> bool {
        if self.matcher == "*" {
            return true;
        }
        self.matcher == tool_name
    }
}

/// Manages hook configurations for different events
pub struct HookManager {
    /// Registered callbacks
    registry: HookRegistry,
    /// Hook matchers organized by event type
    matchers: HashMap<String, Vec<HookMatcherConfig>>,
}

impl HookManager {
    /// Create a new hook manager
    pub fn new() -> Self {
        Self {
            registry: HookRegistry::new(),
            matchers: HashMap::new(),
        }
    }

    /// Register a callback and return its ID
    pub fn register_callback(&mut self, callback: HookCallback) -> String {
        self.registry.register(callback)
    }

    /// Add a hook matcher for a specific event
    pub fn add_matcher(&mut self, event: String, matcher: HookMatcherConfig) {
        self.matchers
            .entry(event)
            .or_insert_with(Vec::new)
            .push(matcher);
    }

    /// Get all matchers for a specific event
    pub fn get_matchers(&self, event: &str) -> Option<&Vec<HookMatcherConfig>> {
        self.matchers.get(event)
    }

    /// Get a callback by ID
    pub fn get_callback(&self, id: &str) -> Option<&HookCallback> {
        self.registry.get(id)
    }

    /// Find matching callback IDs for an event and tool name
    pub fn find_matching_callbacks(&self, event: &str, tool_name: &str) -> Vec<String> {
        let mut callback_ids = Vec::new();

        if let Some(matchers) = self.matchers.get(event) {
            for matcher in matchers {
                if matcher.matches(tool_name) {
                    callback_ids.extend(matcher.callback_ids.clone());
                }
            }
        }

        callback_ids
    }

    /// Execute all matching hooks for an event
    pub async fn execute_hooks(
        &self,
        event: &str,
        tool_name: &str,
        input_data: HashMap<String, serde_json::Value>,
        tool_use_id: Option<String>,
        context: HookContext,
    ) -> Result<Vec<HookJSONOutput>> {
        let callback_ids = self.find_matching_callbacks(event, tool_name);
        let mut results = Vec::new();

        for callback_id in callback_ids {
            if let Some(callback) = self.get_callback(&callback_id) {
                let output = callback(input_data.clone(), tool_use_id.clone(), context.clone()).await?;
                results.push(output);
            }
        }

        Ok(results)
    }

    /// Get the hook configuration for initialization
    ///
    /// Returns a JSON-serializable structure for the control protocol
    pub fn get_initialization_config(&self) -> HashMap<String, Vec<HashMap<String, serde_json::Value>>> {
        let mut config = HashMap::new();

        for (event, matchers) in &self.matchers {
            let matcher_configs: Vec<HashMap<String, serde_json::Value>> = matchers
                .iter()
                .map(|matcher| {
                    let mut m = HashMap::new();
                    m.insert(
                        "matcher".to_string(),
                        serde_json::Value::String(matcher.matcher.clone()),
                    );
                    m.insert(
                        "callback_ids".to_string(),
                        serde_json::to_value(&matcher.callback_ids).unwrap(),
                    );
                    m
                })
                .collect();

            config.insert(event.clone(), matcher_configs);
        }

        config
    }
}

impl Default for HookManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_registry_creation() {
        let registry = HookRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_hook_registry_register() {
        let mut registry = HookRegistry::new();

        let callback: HookCallback = Arc::new(|_, _, _| {
            Box::pin(async { Ok(HookJSONOutput::default()) })
        });

        let id = registry.register(callback);
        assert_eq!(id, "hook_0");
        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&id));
    }

    #[test]
    fn test_hook_registry_multiple_callbacks() {
        let mut registry = HookRegistry::new();

        let callback1: HookCallback = Arc::new(|_, _, _| {
            Box::pin(async { Ok(HookJSONOutput::default()) })
        });
        let callback2: HookCallback = Arc::new(|_, _, _| {
            Box::pin(async { Ok(HookJSONOutput::default()) })
        });

        let id1 = registry.register(callback1);
        let id2 = registry.register(callback2);

        assert_eq!(id1, "hook_0");
        assert_eq!(id2, "hook_1");
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn test_hook_registry_unregister() {
        let mut registry = HookRegistry::new();

        let callback: HookCallback = Arc::new(|_, _, _| {
            Box::pin(async { Ok(HookJSONOutput::default()) })
        });

        let id = registry.register(callback);
        assert_eq!(registry.len(), 1);

        let removed = registry.unregister(&id);
        assert!(removed.is_some());
        assert_eq!(registry.len(), 0);
        assert!(!registry.contains(&id));
    }

    #[test]
    fn test_hook_matcher_wildcard() {
        let matcher = HookMatcherConfig::new("*".to_string(), vec!["hook_0".to_string()]);

        assert!(matcher.matches("Bash"));
        assert!(matcher.matches("Read"));
        assert!(matcher.matches("Write"));
    }

    #[test]
    fn test_hook_matcher_specific() {
        let matcher = HookMatcherConfig::new("Bash".to_string(), vec!["hook_0".to_string()]);

        assert!(matcher.matches("Bash"));
        assert!(!matcher.matches("Read"));
        assert!(!matcher.matches("Write"));
    }

    #[test]
    fn test_hook_manager_creation() {
        let manager = HookManager::new();
        assert!(manager.registry.is_empty());
        assert!(manager.matchers.is_empty());
    }

    #[test]
    fn test_hook_manager_add_matcher() {
        let mut manager = HookManager::new();

        let callback: HookCallback = Arc::new(|_, _, _| {
            Box::pin(async { Ok(HookJSONOutput::default()) })
        });

        let callback_id = manager.register_callback(callback);
        let matcher = HookMatcherConfig::new("Bash".to_string(), vec![callback_id.clone()]);

        manager.add_matcher("PreToolUse".to_string(), matcher);

        let matchers = manager.get_matchers("PreToolUse");
        assert!(matchers.is_some());
        assert_eq!(matchers.unwrap().len(), 1);
    }

    #[test]
    fn test_hook_manager_find_matching_callbacks() {
        let mut manager = HookManager::new();

        let callback: HookCallback = Arc::new(|_, _, _| {
            Box::pin(async { Ok(HookJSONOutput::default()) })
        });

        let callback_id = manager.register_callback(callback);
        let matcher = HookMatcherConfig::new("Bash".to_string(), vec![callback_id.clone()]);

        manager.add_matcher("PreToolUse".to_string(), matcher);

        let matches = manager.find_matching_callbacks("PreToolUse", "Bash");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], callback_id);

        let no_matches = manager.find_matching_callbacks("PreToolUse", "Read");
        assert_eq!(no_matches.len(), 0);
    }

    #[tokio::test]
    async fn test_hook_manager_execute_hooks() {
        let mut manager = HookManager::new();

        let callback: HookCallback = Arc::new(|input, _tool_id, _ctx| {
            Box::pin(async move {
                let value = input.get("value").cloned().unwrap();
                let output = HookJSONOutput {
                    decision: Some("allow".to_string()),
                    system_message: Some(format!("Processed: {:?}", value)),
                    hook_specific_output: None,
                };
                Ok(output)
            })
        });

        let callback_id = manager.register_callback(callback);
        let matcher = HookMatcherConfig::new("Bash".to_string(), vec![callback_id]);

        manager.add_matcher("PreToolUse".to_string(), matcher);

        let mut input_data = HashMap::new();
        input_data.insert(
            "value".to_string(),
            serde_json::Value::String("test_value".to_string()),
        );

        let context = HookContext { signal: None };

        let results = manager
            .execute_hooks("PreToolUse", "Bash", input_data, None, context)
            .await;

        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].decision, Some("allow".to_string()));
    }
}
