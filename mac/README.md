# Agent UI (mac)

This directory contains a macOS SwiftUI implementation of the Agent UI, aligned with the design in doc/system-design.md. It mirrors the single-panel vertical conversation flow with inline code blocks and tool execution cards.

## Structure

- AgentUIApp.swift — App entry point
- Models/Types.swift — Core models: roles, messages, content blocks, tool calls
- Services/AgentService.swift — Backend integration with code-agent (HTTP), stub streaming
- Views/
  - AssistantPanelView.swift — Main container with message list and input
  - MessageListView.swift — Virtualized list-like scrolling using LazyVStack
  - MessageBubbleView.swift — User/Assistant bubbles with Markdown-like blocks
  - CodeBlockView.swift — Simple code rendering with copy button (NSPasteboard)
  - ToolCardView.swift — Collapsible tool execution card
  - MessageInputView.swift — Bottom input area

## Backend Integration (code-agent)

AgentService reads environment variables:

- CODE_AGENT_BASE_URL — Base URL of code-agent backend (e.g., http://localhost:8080)
- CODE_AGENT_API_KEY — Optional API key for Authorization
- CODE_AGENT_MODEL — Model name (default: openai/gpt-4)

When CODE_AGENT_BASE_URL is not set, AgentService returns a stub response and a placeholder stream. To enable full functionality, run your code-agent service and set env variables accordingly.

## Notes

- This implementation operates only within the `mac/` directory and does not modify the Rust `src/` GPUI implementation.
- Streaming is currently a placeholder (AsyncStream). Replace with SSE/WebSocket from code-agent for real token streams.
- Syntax highlighting can be added later using third-party libraries if needed.
