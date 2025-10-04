use crate::errors::{ClaudeSDKError, Result};
use crate::hooks::HookManager;
use crate::mcp_server::SdkMcpServer;
use crate::message_parser::parse_message;
use crate::permissions::CanUseToolCallback;
use crate::transport::{SubprocessTransport, Transport};
use crate::types::{ControlRequest, ControlResponseType, Message, PermissionResult, SDKControlRequest, SDKControlResponse, ToolPermissionContext};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};

/// Query handles control protocol for bidirectional communication
///
/// This manages:
/// - Control request/response routing
/// - Message streaming
/// - Initialization handshake
/// - Hook callbacks
/// - Permission callbacks (can_use_tool)
pub struct Query {
    transport: Arc<Mutex<SubprocessTransport>>,
    is_streaming_mode: bool,

    // Control protocol state
    pending_responses: Arc<RwLock<HashMap<String, tokio::sync::oneshot::Sender<Result<Value>>>>>,
    request_counter: Arc<Mutex<u64>>,

    // Message channels
    message_tx: mpsc::UnboundedSender<Result<Message>>,
    message_rx: Option<mpsc::UnboundedReceiver<Result<Message>>>,

    // Hooks support
    hook_manager: Option<Arc<Mutex<HookManager>>>,

    // Permission callback
    can_use_tool: Option<CanUseToolCallback>,

    // MCP servers
    mcp_servers: Arc<HashMap<String, SdkMcpServer>>,

    // Background task handles
    read_task: Option<tokio::task::JoinHandle<()>>,
    control_task: Option<tokio::task::JoinHandle<()>>,
}

impl Query {
    /// Create a new Query instance
    pub fn new(
        transport: SubprocessTransport,
        is_streaming_mode: bool,
    ) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Self {
            transport: Arc::new(Mutex::new(transport)),
            is_streaming_mode,
            pending_responses: Arc::new(RwLock::new(HashMap::new())),
            request_counter: Arc::new(Mutex::new(0)),
            message_tx,
            message_rx: Some(message_rx),
            hook_manager: None,
            can_use_tool: None,
            mcp_servers: Arc::new(HashMap::new()),
            read_task: None,
            control_task: None,
        }
    }

    /// Create a Query instance with hooks support
    pub fn with_hooks(
        transport: SubprocessTransport,
        is_streaming_mode: bool,
        hook_manager: HookManager,
    ) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Self {
            transport: Arc::new(Mutex::new(transport)),
            is_streaming_mode,
            pending_responses: Arc::new(RwLock::new(HashMap::new())),
            request_counter: Arc::new(Mutex::new(0)),
            message_tx,
            message_rx: Some(message_rx),
            hook_manager: Some(Arc::new(Mutex::new(hook_manager))),
            can_use_tool: None,
            mcp_servers: Arc::new(HashMap::new()),
            read_task: None,
            control_task: None,
        }
    }

    /// Create a Query instance with permission callback
    pub fn with_can_use_tool(
        transport: SubprocessTransport,
        is_streaming_mode: bool,
        can_use_tool: CanUseToolCallback,
    ) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Self {
            transport: Arc::new(Mutex::new(transport)),
            is_streaming_mode,
            pending_responses: Arc::new(RwLock::new(HashMap::new())),
            request_counter: Arc::new(Mutex::new(0)),
            message_tx,
            message_rx: Some(message_rx),
            hook_manager: None,
            can_use_tool: Some(can_use_tool),
            mcp_servers: Arc::new(HashMap::new()),
            read_task: None,
            control_task: None,
        }
    }

    /// Create a Query instance with MCP servers
    pub fn with_mcp_servers(
        transport: SubprocessTransport,
        is_streaming_mode: bool,
        mcp_servers: HashMap<String, SdkMcpServer>,
    ) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Self {
            transport: Arc::new(Mutex::new(transport)),
            is_streaming_mode,
            pending_responses: Arc::new(RwLock::new(HashMap::new())),
            request_counter: Arc::new(Mutex::new(0)),
            message_tx,
            message_rx: Some(message_rx),
            hook_manager: None,
            can_use_tool: None,
            mcp_servers: Arc::new(mcp_servers),
            read_task: None,
            control_task: None,
        }
    }

    /// Create a Query instance with all options
    pub fn with_options(
        transport: SubprocessTransport,
        is_streaming_mode: bool,
        can_use_tool: Option<CanUseToolCallback>,
        mcp_servers: Option<HashMap<String, SdkMcpServer>>,
    ) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Self {
            transport: Arc::new(Mutex::new(transport)),
            is_streaming_mode,
            pending_responses: Arc::new(RwLock::new(HashMap::new())),
            request_counter: Arc::new(Mutex::new(0)),
            message_tx,
            message_rx: Some(message_rx),
            hook_manager: None,
            can_use_tool,
            mcp_servers: Arc::new(mcp_servers.unwrap_or_default()),
            read_task: None,
            control_task: None,
        }
    }

    /// Start reading messages from transport
    pub async fn start(&mut self) -> Result<()> {
        let transport = Arc::clone(&self.transport);
        let message_tx = self.message_tx.clone();
        let pending_responses = Arc::clone(&self.pending_responses);
        let can_use_tool = self.can_use_tool.clone();

        let task = tokio::spawn(async move {
            let mut transport_guard = transport.lock().await;
            let stream = transport_guard.read_messages();

            futures::pin_mut!(stream);

            use futures::StreamExt;
            while let Some(result) = stream.next().await {
                match result {
                    Ok(json_value) => {
                        // Check message type
                        if let Some(msg_type) = json_value.get("type").and_then(|v| v.as_str()) {
                            match msg_type {
                                "control_response" => {
                                    // Handle control response (from CLI to SDK)
                                    if let Ok(ctrl_response) = serde_json::from_value::<SDKControlResponse>(json_value.clone()) {
                                        if let ControlResponseType::Success { request_id, response } = ctrl_response.response {
                                            let mut responses = pending_responses.write().await;
                                            if let Some(tx) = responses.remove(&request_id) {
                                                let result_value = response
                                                    .map(|r| json!(r))
                                                    .unwrap_or(json!({}));
                                                let _ = tx.send(Ok(result_value));
                                            }
                                        } else if let ControlResponseType::Error { request_id, error } = ctrl_response.response {
                                            let mut responses = pending_responses.write().await;
                                            if let Some(tx) = responses.remove(&request_id) {
                                                let _ = tx.send(Err(ClaudeSDKError::cli_connection_error(error)));
                                            }
                                        }
                                    }
                                    continue;
                                }
                                "control_request" => {
                                    // Handle control request (from CLI asking SDK)
                                    if let Ok(ctrl_request) = serde_json::from_value::<SDKControlRequest>(json_value.clone()) {
                                        // Handle can_use_tool requests
                                        if let ControlRequest::CanUseTool { tool_name, input, .. } = ctrl_request.request {
                                            if let Some(ref callback) = can_use_tool {
                                                let context = ToolPermissionContext {
                                                    suggestions: vec![], // TODO: Parse permission_suggestions properly
                                                };

                                                let transport_clone = Arc::clone(&transport);
                                                let request_id = ctrl_request.request_id.clone();
                                                let callback_clone = Arc::clone(callback);

                                                tokio::spawn(async move {
                                                    match callback_clone(tool_name.clone(), input.clone(), context).await {
                                                        Ok(perm_result) => {
                                                            // Convert PermissionResult to response
                                                            let mut response_data = HashMap::new();
                                                            match perm_result {
                                                                PermissionResult::Allow { updated_input, .. } => {
                                                                    response_data.insert("allow".to_string(), json!(true));
                                                                    if let Some(input) = updated_input {
                                                                        response_data.insert("input".to_string(), json!(input));
                                                                    }
                                                                }
                                                                PermissionResult::Deny { message, .. } => {
                                                                    response_data.insert("allow".to_string(), json!(false));
                                                                    response_data.insert("reason".to_string(), json!(message));
                                                                }
                                                            };

                                                            // Send response
                                                            let response = SDKControlResponse {
                                                                r#type: "control_response".to_string(),
                                                                response: ControlResponseType::Success {
                                                                    request_id,
                                                                    response: Some(response_data),
                                                                },
                                                            };

                                                            if let Ok(response_str) = serde_json::to_string(&response) {
                                                                let _ = transport_clone.lock().await.write(&format!("{}\n", response_str)).await;
                                                            }
                                                        }
                                                        Err(e) => {
                                                            // Send error response
                                                            let response = SDKControlResponse {
                                                                r#type: "control_response".to_string(),
                                                                response: ControlResponseType::Error {
                                                                    request_id,
                                                                    error: e.to_string(),
                                                                },
                                                            };

                                                            if let Ok(response_str) = serde_json::to_string(&response) {
                                                                let _ = transport_clone.lock().await.write(&format!("{}\n", response_str)).await;
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    }
                                    continue;
                                }
                                _ => {}
                            }
                        }

                        // Regular message - parse and send
                        match parse_message(&json_value) {
                            Ok(message) => {
                                if message_tx.send(Ok(message)).is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                let _ = message_tx.send(Err(e));
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = message_tx.send(Err(e));
                        break;
                    }
                }
            }
        });

        self.read_task = Some(task);
        Ok(())
    }

    /// Initialize the control protocol
    pub async fn initialize(&self) -> Result<Value> {
        if !self.is_streaming_mode {
            return Ok(json!(null));
        }

        // Get hooks configuration if available
        let hooks_config = if let Some(ref hook_manager) = self.hook_manager {
            let manager = hook_manager.lock().await;
            let config = manager.get_initialization_config();
            serde_json::to_value(config).ok()
        } else {
            None
        };

        let request = json!({
            "subtype": "initialize",
            "hooks": hooks_config
        });

        self.send_control_request(request).await
    }

    /// Send a control request and wait for response
    async fn send_control_request(&self, request: Value) -> Result<Value> {
        let mut counter = self.request_counter.lock().await;
        *counter += 1;
        let request_id = format!("req_{}", *counter);
        drop(counter);

        // Create oneshot channel for response
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Register pending response
        {
            let mut responses = self.pending_responses.write().await;
            responses.insert(request_id.clone(), tx);
        }

        // Build and send control request
        let control_msg = json!({
            "type": "control_request",
            "request_id": request_id,
            "request": request
        });

        let msg_str = serde_json::to_string(&control_msg)
            .map_err(|e| ClaudeSDKError::json_decode_error(String::new(), e.to_string()))?;

        {
            let mut transport = self.transport.lock().await;
            transport.write(&format!("{}\n", msg_str)).await?;
        }

        // Wait for response with timeout
        tokio::time::timeout(std::time::Duration::from_secs(30), rx)
            .await
            .map_err(|_| ClaudeSDKError::cli_connection_error("Control request timeout".to_string()))?
            .map_err(|_| ClaudeSDKError::cli_connection_error("Response channel closed".to_string()))?
    }

    /// Receive messages from the message stream
    pub fn receive_messages(&mut self) -> mpsc::UnboundedReceiver<Result<Message>> {
        self.message_rx.take().expect("Messages already taken")
    }

    /// Send a message through the transport
    pub async fn send_message(&mut self, message: Value) -> Result<()> {
        let msg_str = serde_json::to_string(&message)
            .map_err(|e| ClaudeSDKError::json_decode_error(String::new(), e.to_string()))?;

        let mut transport = self.transport.lock().await;
        transport.write(&format!("{}\n", msg_str)).await
    }

    /// Send an interrupt signal
    pub async fn interrupt(&mut self) -> Result<()> {
        let request = json!({
            "subtype": "interrupt"
        });

        self.send_control_request(request).await?;
        Ok(())
    }

    /// Set the permission mode
    pub async fn set_permission_mode(&mut self, mode: &str) -> Result<()> {
        let request = json!({
            "subtype": "set_permission_mode",
            "mode": mode
        });

        self.send_control_request(request).await?;
        Ok(())
    }

    /// Set the AI model
    pub async fn set_model(&mut self, model: Option<&str>) -> Result<()> {
        let request = json!({
            "subtype": "set_model",
            "model": model
        });

        self.send_control_request(request).await?;
        Ok(())
    }

    /// Close the query and clean up
    pub async fn close(mut self) -> Result<()> {
        if let Some(task) = self.read_task.take() {
            task.abort();
        }

        let mut transport = self.transport.lock().await;
        transport.close().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::SubprocessTransport;
    use crate::types::ClaudeAgentOptions;

    #[tokio::test]
    async fn test_query_creation() {
        let opts = ClaudeAgentOptions::default();
        let transport = SubprocessTransport::new("test".to_string(), opts);
        let query = Query::new(transport, true);

        assert!(query.is_streaming_mode);
    }

    #[tokio::test]
    async fn test_query_not_streaming() {
        let opts = ClaudeAgentOptions::default();
        let transport = SubprocessTransport::new("test".to_string(), opts);
        let query = Query::new(transport, false);

        assert!(!query.is_streaming_mode);
    }
}
