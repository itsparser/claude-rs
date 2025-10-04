# Claude SDK Examples (Rust)

This directory contains example programs demonstrating the Claude SDK for Rust. These examples showcase the core functionality that has been migrated from the Python SDK.

## Available Examples

### 1. Types Demo (`types_demo.rs`)

Demonstrates working with Claude SDK types:

- **ClaudeAgentOptions**: Configuration for Claude agent behavior
- **Message Types**: User, Assistant, System, and Result messages
- **Content Blocks**: Text, Thinking, ToolUse, and ToolResult blocks
- **Permission Types**: Permission modes, behaviors, and results
- **MCP Server Configs**: Stdio, SSE, and HTTP configurations
- **Serialization**: JSON serialization and deserialization

**Run:**
```bash
cargo run --example types_demo
```

**Key Features:**
- Creating and configuring agent options
- Building different message types
- Working with content blocks
- Managing permissions
- Configuring MCP servers
- JSON serialization/deserialization

### 2. Message Parser Demo (`message_parser_demo.rs`)

Demonstrates message parsing functionality:

- **User Messages**: Parsing text and content block formats
- **Assistant Messages**: Handling multiple content block types
- **System Messages**: Processing system notifications
- **Result Messages**: Extracting cost, usage, and session data
- **Stream Events**: Handling streaming message updates
- **Error Handling**: Graceful error handling for invalid messages

**Run:**
```bash
cargo run --example message_parser_demo
```

**Key Features:**
- Parse different message types from JSON
- Handle various content blocks
- Work with streaming events
- Robust error handling

### 3. Error Handling Demo (`error_handling_demo.rs`)

Demonstrates comprehensive error handling:

- **CLINotFoundError**: When Claude Code CLI is not installed
- **ProcessError**: When CLI process fails or exits with errors
- **CLIJSONDecodeError**: When JSON parsing fails
- **MessageParseError**: When message parsing fails
- **CLIConnectionError**: When connection to CLI fails
- **Result Type**: Using Rust's Result type for error handling
- **Error Propagation**: Using `?` operator for clean error handling

**Run:**
```bash
cargo run --example error_handling_demo
```

**Key Features:**
- Different error types and their use cases
- Error creation and formatting
- Result type usage
- Error propagation with `?` operator
- Error recovery patterns

## Running All Examples

To run all examples at once:

```bash
./examples/run_all.sh
```

Or individually:

```bash
# Types demo
cargo run --example types_demo

# Message parser demo
cargo run --example message_parser_demo

# Error handling demo
cargo run --example error_handling_demo
```

## Building Examples

Build all examples without running:

```bash
cargo build --examples
```

Build a specific example:

```bash
cargo build --example types_demo
```

## Example Output

Each example produces formatted output showing:
- ✓ Success indicators for successful operations
- ✗ Error indicators for error cases
- Detailed information about types, messages, and operations
- Demonstration of error handling and recovery

## Migration Status

These examples demonstrate the functionality that has been successfully migrated from the Python SDK:

### ✅ Completed
- **types.py → types.rs**: All type definitions (414 lines, 23 tests)
- **_errors.py → errors.rs**: Error types (110 lines, 13 tests)
- **message_parser.py → message_parser.rs**: Message parsing (450+ lines, 17 tests)

### 📋 Pending
- **subprocess_cli.py**: CLI subprocess management
- **query.py**: One-shot query function
- **client.py**: Interactive session client

## Comparison with Python Examples

The Python SDK examples in `claude-agent-sdk-python/examples/` include:

1. **quick_start.py**: Basic query, options, and tools
2. **mcp_calculator.py**: Custom MCP tools with SDK server
3. **streaming_mode.py**: ClaudeSDKClient usage
4. **hooks.py**: Hook implementations

The Rust examples currently demonstrate:
- **types_demo.rs**: Equivalent to Python type usage
- **message_parser_demo.rs**: Message parsing (internal to Python)
- **error_handling_demo.rs**: Error handling patterns

## Testing

All example code is also covered by the test suite:

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test types
cargo test errors
cargo test message_parser
```

Current test results: **58 tests passing**
- 23 types tests
- 13 error tests
- 17 message parser tests
- 5 inline tests

## Next Steps

Future examples will demonstrate:
- Full query() function usage (once transport layer is complete)
- ClaudeSDKClient for interactive sessions
- Custom MCP tools in Rust
- Hook implementations
- Real-world usage patterns

## Resources

- [Claude Code Documentation](https://docs.anthropic.com/en/docs/claude-code)
- [Python SDK Documentation](https://docs.anthropic.com/en/docs/claude-code/sdk/sdk-python)
- [Python SDK Examples](../claude-agent-sdk-python/examples/)

## License

MIT
