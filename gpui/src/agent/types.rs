// Agent types and data structures

use serde::{Deserialize, Serialize};

/// Agent request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRequest {
    pub messages: Vec<ChatMessage>,
    pub tools: Option<Vec<ToolDefinition>>,
    pub stream: bool,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Agent response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub finish_reason: String,
}

/// Tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Stream event
#[derive(Debug, Clone)]
pub enum StreamEvent {
    /// Text delta
    TextDelta(String),
    
    /// Tool call started
    ToolCallStart {
        id: String,
        name: String,
    },
    
    /// Tool call arguments delta
    ToolCallArgumentsDelta {
        id: String,
        delta: String,
    },
    
    /// Tool call completed
    ToolCallComplete {
        id: String,
        name: String,
        arguments: serde_json::Value,
    },
    
    /// Tool execution started
    ToolExecutionStart {
        id: String,
        name: String,
    },
    
    /// Tool execution completed
    ToolExecutionComplete {
        id: String,
        result: String,
    },
    
    /// Stream completed
    Done,
    
    /// Error occurred
    Error(String),
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

