/// Extension traits for better ergonomics
use crate::types::{AssistantMessage, ContentBlock, Message, ResultMessage};

/// Extension methods for Vec<Message>
///
/// Provides convenient access to message content without verbose pattern matching.
pub trait MessageVecExt {
    /// Get all assistant messages
    fn assistant_messages(&self) -> Vec<&AssistantMessage>;

    /// Extract all text content from assistant messages
    fn text_content(&self) -> String;

    /// Get the last assistant message
    fn last_assistant(&self) -> Option<&AssistantMessage>;

    /// Get the result message
    fn result_message(&self) -> Option<&ResultMessage>;

    /// Check if there are any assistant messages
    fn has_assistant_messages(&self) -> bool;

    /// Get all text blocks from assistant messages
    fn text_blocks(&self) -> Vec<&str>;
}

impl MessageVecExt for Vec<Message> {
    fn assistant_messages(&self) -> Vec<&AssistantMessage> {
        self.iter().filter_map(|m| m.as_assistant()).collect()
    }

    fn text_content(&self) -> String {
        self.iter()
            .filter_map(|m| m.as_assistant())
            .flat_map(|msg| &msg.content)
            .filter_map(|block| match block {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn last_assistant(&self) -> Option<&AssistantMessage> {
        self.iter().rev().find_map(|m| m.as_assistant())
    }

    fn result_message(&self) -> Option<&ResultMessage> {
        self.iter().find_map(|m| m.as_result())
    }

    fn has_assistant_messages(&self) -> bool {
        self.iter().any(|m| m.is_assistant())
    }

    fn text_blocks(&self) -> Vec<&str> {
        self.iter()
            .filter_map(|m| m.as_assistant())
            .flat_map(|msg| &msg.content)
            .filter_map(|block| match block {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }
}

/// Extension methods for Message
impl Message {
    /// Check if this is an assistant message
    pub fn is_assistant(&self) -> bool {
        matches!(self, Message::Assistant(_))
    }

    /// Check if this is a user message
    pub fn is_user(&self) -> bool {
        matches!(self, Message::User(_))
    }

    /// Check if this is a result message
    pub fn is_result(&self) -> bool {
        matches!(self, Message::Result(_))
    }

    /// Get as assistant message if it is one
    pub fn as_assistant(&self) -> Option<&AssistantMessage> {
        match self {
            Message::Assistant(msg) => Some(msg),
            _ => None,
        }
    }

    /// Get as result message if it is one
    pub fn as_result(&self) -> Option<&ResultMessage> {
        match self {
            Message::Result(msg) => Some(msg),
            _ => None,
        }
    }

    /// Extract text content if this is an assistant message
    pub fn text_content(&self) -> Option<String> {
        self.as_assistant().map(|msg| {
            msg.content
                .iter()
                .filter_map(|block| match block {
                    ContentBlock::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_messages() -> Vec<Message> {
        vec![
            Message::Assistant(AssistantMessage {
                content: vec![
                    ContentBlock::Text {
                        text: "Hello".to_string(),
                    },
                    ContentBlock::Text {
                        text: "World".to_string(),
                    },
                ],
                model: "test-model".to_string(),
                parent_tool_use_id: None,
            }),
            Message::Assistant(AssistantMessage {
                content: vec![ContentBlock::Text {
                    text: "Goodbye".to_string(),
                }],
                model: "test-model".to_string(),
                parent_tool_use_id: None,
            }),
        ]
    }

    #[test]
    fn test_text_content() {
        let messages = create_test_messages();
        let text = messages.text_content();
        assert_eq!(text, "Hello\nWorld\nGoodbye");
    }

    #[test]
    fn test_assistant_messages() {
        let messages = create_test_messages();
        let assistants = messages.assistant_messages();
        assert_eq!(assistants.len(), 2);
    }

    #[test]
    fn test_last_assistant() {
        let messages = create_test_messages();
        let last = messages.last_assistant();
        assert!(last.is_some());
        let text = last
            .unwrap()
            .content
            .iter()
            .filter_map(|b| match b {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");
        assert_eq!(text, "Goodbye");
    }

    #[test]
    fn test_has_assistant_messages() {
        let messages = create_test_messages();
        assert!(messages.has_assistant_messages());

        let empty: Vec<Message> = vec![];
        assert!(!empty.has_assistant_messages());
    }

    #[test]
    fn test_message_methods() {
        let msg = Message::Assistant(AssistantMessage {
            content: vec![ContentBlock::Text {
                text: "Test".to_string(),
            }],
            model: "test-model".to_string(),
            parent_tool_use_id: None,
        });

        assert!(msg.is_assistant());
        assert!(!msg.is_user());
        assert!(!msg.is_result());
        assert!(msg.as_assistant().is_some());
        assert_eq!(msg.text_content(), Some("Test".to_string()));
    }

    #[test]
    fn test_text_blocks() {
        let messages = create_test_messages();
        let blocks = messages.text_blocks();
        assert_eq!(blocks, vec!["Hello", "World", "Goodbye"]);
    }
}
