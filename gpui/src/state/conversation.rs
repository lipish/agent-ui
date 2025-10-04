// Conversation state management

use crate::agent::{types::ChatMessage, AgentService};
use crate::models::Message;
use std::sync::Arc;

/// Conversation state - manages messages and input
pub struct ConversationState {
    messages: Vec<Message>,
    input: String,
    agent_service: Option<Arc<AgentService>>,
    is_loading: bool,
}

impl ConversationState {
    /// Create a new conversation with a welcome message
    pub fn new() -> Self {
        Self {
            messages: vec![Message::assistant(
                "你好！我是 AI 代码助手，我可以帮你：\n\n\
                 • 阅读和分析代码\n\
                 • 搜索代码库\n\
                 • 执行命令\n\
                 • 编写和修改文件\n\n\
                 有什么可以帮你的吗？",
            )],
            input: String::new(),
            agent_service: None,
            is_loading: false,
        }
    }

    /// Create with agent service
    pub fn with_agent_service(agent_service: Arc<AgentService>) -> Self {
        let mut state = Self::new();
        state.agent_service = Some(agent_service);
        state
    }

    /// Set agent service
    pub fn set_agent_service(&mut self, agent_service: Arc<AgentService>) {
        self.agent_service = Some(agent_service);
    }

    /// Check if loading
    pub fn is_loading(&self) -> bool {
        self.is_loading
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

    /// Send the current input as a user message
    /// Returns true if message was sent (will get async response)
    pub fn send_message(&mut self) -> bool {
        if self.input.trim().is_empty() || self.is_loading {
            return false;
        }

        // Add user message
        let user_message = self.input.clone();
        self.add_user_message(user_message.clone());
        self.clear_input();
        self.is_loading = true;

        // If no agent service, use mock response
        if self.agent_service.is_none() {
            let responses = [
                "我理解了你的问题。",
                "这是一个很好的想法！",
                "让我帮你分析一下。",
                "我会尽力帮助你。",
            ];

            let response = responses[self.messages.len() % responses.len()];
            self.add_assistant_message(response.to_string());
            self.is_loading = false;
        }

        true
    }

    /// Send message with agent service (async)
    pub async fn send_message_async(&mut self, user_message: String) -> anyhow::Result<()> {
        if let Some(agent_service) = &self.agent_service {
            self.is_loading = true;

            // Convert messages to ChatMessage format
            let chat_messages: Vec<ChatMessage> = self
                .messages
                .iter()
                .map(|msg| ChatMessage {
                    role: msg.role.as_str().to_string(),
                    content: msg.content.clone(),
                })
                .collect();

            // Add new user message
            let mut messages = chat_messages;
            messages.push(ChatMessage::user(user_message.clone()));

            // Get response from agent
            let response = agent_service.run_agent_loop(user_message).await?;

            // Add assistant response
            self.add_assistant_message(response);
            self.is_loading = false;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Agent service not configured"))
        }
    }
}

impl Default for ConversationState {
    fn default() -> Self {
        Self::new()
    }
}
