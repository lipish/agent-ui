// Conversation state management

use crate::models::Message;

/// Conversation state - manages messages and input
pub struct ConversationState {
    messages: Vec<Message>,
    input: String,
}

impl ConversationState {
    /// Create a new conversation with a welcome message
    pub fn new() -> Self {
        Self {
            messages: vec![Message::assistant(
                "你好！我是 AI 助手，有什么可以帮你的吗？",
            )],
            input: String::new(),
        }
    }

    /// Get all messages
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Get current input text
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Set input text
    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    /// Clear input text
    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    /// Add a user message
    pub fn add_user_message(&mut self, content: String) {
        self.messages.push(Message::user(content));
    }

    /// Add an assistant message
    pub fn add_assistant_message(&mut self, content: String) {
        self.messages.push(Message::assistant(content));
    }

    /// Send the current input as a user message and get a mock response
    pub fn send_message(&mut self) -> bool {
        if self.input.trim().is_empty() {
            return false;
        }

        // Add user message
        self.add_user_message(self.input.clone());
        self.clear_input();

        // Mock AI response
        let responses = [
            "我理解了你的问题。",
            "这是一个很好的想法！",
            "让我帮你分析一下。",
            "我会尽力帮助你。",
        ];

        let response = responses[self.messages.len() % responses.len()];
        self.add_assistant_message(response.to_string());

        true
    }
}

impl Default for ConversationState {
    fn default() -> Self {
        Self::new()
    }
}

