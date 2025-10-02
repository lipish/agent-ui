// Agent UI Library
// 模块化设计，遵循系统设计文档

pub mod assets;
pub mod components;
pub mod models;
pub mod state;
pub mod theme;
pub mod utils;

// Re-export commonly used types
pub use components::AssistantPanel;
pub use models::{Message, MessageRole};
pub use state::ConversationState;
pub use theme::Theme;

