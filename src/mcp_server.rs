use crate::errors::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias for async tool handler functions
///
/// A tool handler receives:
/// - args: The input parameters for the tool as a HashMap
///
/// Returns a ToolResult containing content blocks
pub type ToolHandler = Arc<
    dyn Fn(HashMap<String, Value>) -> Pin<Box<dyn Future<Output = Result<ToolResult>> + Send>>
        + Send
        + Sync,
>;

/// Tool definition for MCP servers
#[derive(Clone)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    handler: ToolHandler,
}

impl std::fmt::Debug for McpTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("McpTool")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("input_schema", &self.input_schema)
            .finish()
    }
}

impl McpTool {
    /// Create a new MCP tool
    pub fn new(
        name: String,
        description: String,
        input_schema: Value,
        handler: ToolHandler,
    ) -> Self {
        Self {
            name,
            description,
            input_schema,
            handler,
        }
    }

    /// Execute the tool with given arguments
    pub async fn execute(&self, args: HashMap<String, Value>) -> Result<ToolResult> {
        (self.handler)(args).await
    }
}

/// Result from a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub content: Vec<ToolResultContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

impl ToolResult {
    /// Create a successful text result
    pub fn text(text: String) -> Self {
        Self {
            content: vec![ToolResultContent::Text { text }],
            is_error: None,
        }
    }

    /// Create an error result
    pub fn error(text: String) -> Self {
        Self {
            content: vec![ToolResultContent::Text { text }],
            is_error: Some(true),
        }
    }

    /// Create a result with multiple content blocks
    pub fn with_content(content: Vec<ToolResultContent>) -> Self {
        Self {
            content,
            is_error: None,
        }
    }
}

/// Content block in a tool result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolResultContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image {
        #[serde(rename = "source")]
        source: ImageSource,
    },
}

/// Image source for tool results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

/// SDK MCP Server - in-process MCP server
///
/// SDK MCP servers run within the same process as your application,
/// eliminating IPC overhead and simplifying deployment.
#[derive(Clone)]
pub struct SdkMcpServer {
    pub name: String,
    pub version: String,
    tools: Arc<HashMap<String, McpTool>>,
}

impl SdkMcpServer {
    /// Create a new SDK MCP server
    ///
    /// # Example
    /// ```
    /// use claude::{SdkMcpServer, McpTool, ToolResult};
    /// use std::sync::Arc;
    /// use std::collections::HashMap;
    ///
    /// let greet_tool = McpTool::new(
    ///     "greet".to_string(),
    ///     "Greet a user".to_string(),
    ///     serde_json::json!({
    ///         "type": "object",
    ///         "properties": {
    ///             "name": {"type": "string"}
    ///         },
    ///         "required": ["name"]
    ///     }),
    ///     Arc::new(|args| {
    ///         Box::pin(async move {
    ///             let name = args.get("name")
    ///                 .and_then(|v| v.as_str())
    ///                 .unwrap_or("World");
    ///             Ok(ToolResult::text(format!("Hello, {}!", name)))
    ///         })
    ///     }),
    /// );
    ///
    /// let server = SdkMcpServer::new(
    ///     "my-tools".to_string(),
    ///     "1.0.0".to_string(),
    ///     vec![greet_tool],
    /// );
    /// ```
    pub fn new(name: String, version: String, tools: Vec<McpTool>) -> Self {
        let mut tool_map = HashMap::new();
        for tool in tools {
            tool_map.insert(tool.name.clone(), tool);
        }

        Self {
            name,
            version,
            tools: Arc::new(tool_map),
        }
    }

    /// Get a tool by name
    pub fn get_tool(&self, name: &str) -> Option<&McpTool> {
        self.tools.get(name)
    }

    /// List all available tools
    pub fn list_tools(&self) -> Vec<&McpTool> {
        self.tools.values().collect()
    }

    /// Execute a tool by name
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        args: HashMap<String, Value>,
    ) -> Result<ToolResult> {
        let tool = self
            .tools
            .get(tool_name)
            .ok_or_else(|| crate::errors::ClaudeSDKError::message_parse_error(
                format!("Tool not found: {}", tool_name),
                None
            ))?;

        tool.execute(args).await
    }

    /// Get the server configuration for ClaudeAgentOptions
    pub fn to_config(&self) -> HashMap<String, Value> {
        let mut config = HashMap::new();
        config.insert("name".to_string(), serde_json::json!(self.name));
        config.insert("version".to_string(), serde_json::json!(self.version));
        config.insert("type".to_string(), serde_json::json!("sdk"));

        // Convert tools to JSON schema
        let tools: Vec<Value> = self
            .tools
            .values()
            .map(|tool| {
                serde_json::json!({
                    "name": tool.name,
                    "description": tool.description,
                    "inputSchema": tool.input_schema,
                })
            })
            .collect();

        config.insert("tools".to_string(), serde_json::json!(tools));
        config
    }
}

impl std::fmt::Debug for SdkMcpServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SdkMcpServer")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("tools", &self.tools.keys().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_creation() {
        let tool = McpTool::new(
            "test".to_string(),
            "A test tool".to_string(),
            serde_json::json!({"type": "object"}),
            Arc::new(|_args| {
                Box::pin(async { Ok(ToolResult::text("test result".to_string())) })
            }),
        );

        assert_eq!(tool.name, "test");
        assert_eq!(tool.description, "A test tool");
    }

    #[tokio::test]
    async fn test_tool_execution() {
        let tool = McpTool::new(
            "echo".to_string(),
            "Echo input".to_string(),
            serde_json::json!({"type": "object"}),
            Arc::new(|args| {
                Box::pin(async move {
                    let text = args
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("no message");
                    Ok(ToolResult::text(text.to_string()))
                })
            }),
        );

        let mut args = HashMap::new();
        args.insert("message".to_string(), serde_json::json!("hello"));

        let result = tool.execute(args).await.unwrap();
        assert_eq!(result.content.len(), 1);
        if let ToolResultContent::Text { text } = &result.content[0] {
            assert_eq!(text, "hello");
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_server_creation() {
        let tool = McpTool::new(
            "greet".to_string(),
            "Greet a user".to_string(),
            serde_json::json!({"type": "object"}),
            Arc::new(|_args| {
                Box::pin(async { Ok(ToolResult::text("Hello!".to_string())) })
            }),
        );

        let server = SdkMcpServer::new(
            "test-server".to_string(),
            "1.0.0".to_string(),
            vec![tool],
        );

        assert_eq!(server.name, "test-server");
        assert_eq!(server.version, "1.0.0");
        assert_eq!(server.list_tools().len(), 1);
    }

    #[tokio::test]
    async fn test_server_get_tool() {
        let tool = McpTool::new(
            "test_tool".to_string(),
            "A test tool".to_string(),
            serde_json::json!({"type": "object"}),
            Arc::new(|_args| {
                Box::pin(async { Ok(ToolResult::text("result".to_string())) })
            }),
        );

        let server = SdkMcpServer::new("server".to_string(), "1.0.0".to_string(), vec![tool]);

        assert!(server.get_tool("test_tool").is_some());
        assert!(server.get_tool("nonexistent").is_none());
    }

    #[tokio::test]
    async fn test_server_execute_tool() {
        let tool = McpTool::new(
            "calculator".to_string(),
            "Add two numbers".to_string(),
            serde_json::json!({"type": "object"}),
            Arc::new(|args| {
                Box::pin(async move {
                    let a = args.get("a").and_then(|v| v.as_i64()).unwrap_or(0);
                    let b = args.get("b").and_then(|v| v.as_i64()).unwrap_or(0);
                    Ok(ToolResult::text(format!("{}", a + b)))
                })
            }),
        );

        let server = SdkMcpServer::new("calc".to_string(), "1.0.0".to_string(), vec![tool]);

        let mut args = HashMap::new();
        args.insert("a".to_string(), serde_json::json!(5));
        args.insert("b".to_string(), serde_json::json!(3));

        let result = server.execute_tool("calculator", args).await.unwrap();

        if let ToolResultContent::Text { text } = &result.content[0] {
            assert_eq!(text, "8");
        } else {
            panic!("Expected text content");
        }
    }

    #[test]
    fn test_tool_result_helpers() {
        let text_result = ToolResult::text("success".to_string());
        assert_eq!(text_result.content.len(), 1);
        assert!(text_result.is_error.is_none());

        let error_result = ToolResult::error("failed".to_string());
        assert_eq!(error_result.content.len(), 1);
        assert_eq!(error_result.is_error, Some(true));
    }

    #[test]
    fn test_server_to_config() {
        let tool = McpTool::new(
            "test".to_string(),
            "Test tool".to_string(),
            serde_json::json!({
                "type": "object",
                "properties": {
                    "arg": {"type": "string"}
                }
            }),
            Arc::new(|_args| {
                Box::pin(async { Ok(ToolResult::text("result".to_string())) })
            }),
        );

        let server = SdkMcpServer::new("my-server".to_string(), "1.0.0".to_string(), vec![tool]);

        let config = server.to_config();
        assert_eq!(config.get("name").unwrap(), "my-server");
        assert_eq!(config.get("version").unwrap(), "1.0.0");
        assert_eq!(config.get("type").unwrap(), "sdk");
        assert!(config.get("tools").is_some());
    }
}
