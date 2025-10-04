use claude::{Query, ClaudeAgentOptions};
use claude::transport::SubprocessTransport;

#[tokio::test]
async fn test_query_creation_streaming_mode() {
    let opts = ClaudeAgentOptions::default();
    let transport = SubprocessTransport::new("test".to_string(), opts);
    let query = Query::new(transport, true);

    // Verify it's created in streaming mode
    drop(query);
}

#[tokio::test]
async fn test_query_creation_non_streaming_mode() {
    let opts = ClaudeAgentOptions::default();
    let transport = SubprocessTransport::new("test".to_string(), opts);
    let query = Query::new(transport, false);

    // Verify it's created in non-streaming mode
    drop(query);
}

#[tokio::test]
async fn test_query_initialize_non_streaming_returns_null() {
    let opts = ClaudeAgentOptions::default();
    let transport = SubprocessTransport::new("test".to_string(), opts);
    let query = Query::new(transport, false);

    let result = query.initialize().await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_null());
}

#[tokio::test]
async fn test_query_with_different_options() {
    let opts = ClaudeAgentOptions {
        max_turns: Some(5),
        model: Some("claude-sonnet-4-5".to_string()),
        ..Default::default()
    };
    let transport = SubprocessTransport::new("test prompt".to_string(), opts);
    let query = Query::new(transport, true);

    drop(query);
}
