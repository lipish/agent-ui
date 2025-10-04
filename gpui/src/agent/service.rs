// Agent service - handles AI agent interactions

use anyhow::Result;
use futures::stream::Stream;
use std::pin::Pin;
use super::types::{AgentRequest, AgentResponse, ChatMessage, StreamEvent, ToolCall};
use super::tools;

/// Agent service configuration
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub api_key: String,
    pub api_base: String,
    pub model: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            api_base: std::env::var("OPENAI_API_BASE")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            model: std::env::var("OPENAI_MODEL")
                .unwrap_or_else(|_| "gpt-4".to_string()),
        }
    }
}

/// Agent service
pub struct AgentService {
    config: AgentConfig,
    client: reqwest::Client,
}

impl AgentService {
    /// Create a new agent service
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Create with default configuration
    pub fn with_default() -> Self {
        Self::new(AgentConfig::default())
    }

    /// Send a message and get a response
    pub async fn send_message(&self, messages: Vec<ChatMessage>) -> Result<AgentResponse> {
        let request = AgentRequest {
            messages,
            tools: Some(tools::get_available_tools()),
            stream: false,
        };

        self.send_request(request).await
    }

    /// Send a message and get a streaming response
    pub async fn send_message_stream(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<Pin<Box<dyn Stream<Item = StreamEvent> + Send>>> {
        let request = AgentRequest {
            messages,
            tools: Some(tools::get_available_tools()),
            stream: true,
        };

        self.send_request_stream(request).await
    }

    /// Send a request to the API
    async fn send_request(&self, request: AgentRequest) -> Result<AgentResponse> {
        // Build the API request
        let api_request = self.build_api_request(&request)?;

        // Send the request
        let response = self
            .client
            .post(format!("{}/chat/completions", self.config.api_base))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&api_request)
            .send()
            .await?;

        // Parse the response
        let response_json: serde_json::Value = response.json().await?;

        // Extract the content and tool calls
        let choice = &response_json["choices"][0];
        let message = &choice["message"];
        
        let content = message["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tool_calls = if let Some(calls) = message["tool_calls"].as_array() {
            calls
                .iter()
                .filter_map(|call| {
                    Some(ToolCall {
                        id: call["id"].as_str()?.to_string(),
                        name: call["function"]["name"].as_str()?.to_string(),
                        arguments: serde_json::from_str(
                            call["function"]["arguments"].as_str()?
                        ).ok()?,
                    })
                })
                .collect()
        } else {
            Vec::new()
        };

        let finish_reason = choice["finish_reason"]
            .as_str()
            .unwrap_or("stop")
            .to_string();

        Ok(AgentResponse {
            content,
            tool_calls,
            finish_reason,
        })
    }

    /// Send a streaming request to the API
    async fn send_request_stream(
        &self,
        _request: AgentRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = StreamEvent> + Send>>> {
        // TODO: Implement streaming
        // For now, return a simple stream
        use async_stream::stream;
        
        let s = stream! {
            yield StreamEvent::TextDelta("Hello".to_string());
            yield StreamEvent::TextDelta(" from".to_string());
            yield StreamEvent::TextDelta(" agent!".to_string());
            yield StreamEvent::Done;
        };

        Ok(Box::pin(s))
    }

    /// Build the API request JSON
    fn build_api_request(&self, request: &AgentRequest) -> Result<serde_json::Value> {
        let mut api_request = serde_json::json!({
            "model": self.config.model,
            "messages": request.messages,
            "stream": request.stream,
        });

        if let Some(tools) = &request.tools {
            let tools_json: Vec<serde_json::Value> = tools
                .iter()
                .map(|tool| {
                    serde_json::json!({
                        "type": "function",
                        "function": {
                            "name": tool.name,
                            "description": tool.description,
                            "parameters": tool.parameters,
                        }
                    })
                })
                .collect();

            api_request["tools"] = serde_json::json!(tools_json);
            api_request["tool_choice"] = serde_json::json!("auto");
        }

        Ok(api_request)
    }

    /// Execute tool calls and get results
    pub async fn execute_tools(&self, tool_calls: Vec<ToolCall>) -> Vec<(String, String)> {
        let mut results = Vec::new();

        for call in tool_calls {
            let result = match tools::execute_tool(&call.name, call.arguments) {
                Ok(output) => output,
                Err(e) => format!("Error: {}", e),
            };

            results.push((call.id, result));
        }

        results
    }

    /// Run a complete agent loop with tool execution
    pub async fn run_agent_loop(&self, user_message: String) -> Result<String> {
        let mut messages = vec![ChatMessage::user(user_message)];
        let mut final_response = String::new();

        // Agent loop: up to 10 iterations
        for _ in 0..10 {
            let response = self.send_message(messages.clone()).await?;

            // Add assistant response to messages
            if !response.content.is_empty() {
                final_response = response.content.clone();
                messages.push(ChatMessage::assistant(response.content));
            }

            // If no tool calls, we're done
            if response.tool_calls.is_empty() {
                break;
            }

            // Execute tool calls
            let tool_results = self.execute_tools(response.tool_calls.clone()).await;

            // Add tool results to messages
            for (tool_call_id, result) in tool_results {
                messages.push(ChatMessage {
                    role: "tool".to_string(),
                    content: result,
                });
                
                // Store tool call ID for reference
                // (In a real implementation, this would be part of the message structure)
                let _ = tool_call_id;
            }
        }

        Ok(final_response)
    }
}

