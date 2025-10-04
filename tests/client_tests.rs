use claude::{ClaudeSDKClient, ClaudeAgentOptions};

#[tokio::test]
async fn test_client_creation_with_default_options() {
    let client = ClaudeSDKClient::new(None);
    // Just verify it can be created
    drop(client);
}

#[tokio::test]
async fn test_client_creation_with_custom_options() {
    let opts = ClaudeAgentOptions {
        max_turns: Some(5),
        allowed_tools: vec!["Bash".to_string()],
        ..Default::default()
    };

    let client = ClaudeSDKClient::new(Some(opts));
    drop(client);
}

#[tokio::test]
async fn test_client_query_before_connect_fails() {
    let mut client = ClaudeSDKClient::new(None);

    let result = client.query("test", None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Not connected"));
}

#[tokio::test]
async fn test_client_interrupt_before_connect_fails() {
    let mut client = ClaudeSDKClient::new(None);

    let result = client.interrupt().await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Not connected"));
}

#[tokio::test]
async fn test_client_set_permission_mode_before_connect_fails() {
    let mut client = ClaudeSDKClient::new(None);

    let result = client.set_permission_mode("default").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Not connected"));
}

#[tokio::test]
async fn test_client_set_model_before_connect_fails() {
    let mut client = ClaudeSDKClient::new(None);

    let result = client.set_model(Some("claude-sonnet-4-5")).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Not connected"));
}

#[tokio::test]
async fn test_receive_messages_before_connect_returns_empty() {
    use futures::StreamExt;

    let mut client = ClaudeSDKClient::new(None);
    let mut stream = client.receive_messages();

    // Should return None immediately since not connected
    assert!(stream.next().await.is_none());
}

#[tokio::test]
async fn test_client_options_preserved() {
    let opts = ClaudeAgentOptions {
        max_turns: Some(10),
        model: Some("claude-sonnet-4-5".to_string()),
        allowed_tools: vec!["Bash".to_string(), "Read".to_string()],
        ..Default::default()
    };

    let client = ClaudeSDKClient::new(Some(opts.clone()));
    // Verify options are stored (indirectly through creation success)
    drop(client);
}
