import SwiftUI
import AppKit

struct AssistantPanelView: View {
    @State private var messages: [MessageModel] = []
    @State private var inputText: String = ""
    @State private var streamingText: String = ""
    @State private var isStreaming: Bool = false
    // Minimal toolbar states
    @State private var models: [String] = ["openai/gpt-4", "openai/gpt-4o", "anthropic/claude-3.5", "custom/my-model"]
    @State private var selectedModel: String = "openai/gpt-4"
    @State private var deepThinkEnabled: Bool = false
    @State private var internetEnabled: Bool = true

    private let service = AgentService()

    var body: some View {
        VStack(spacing: 0) {
            MessageListView(messages: messages, streamingText: isStreaming ? streamingText : nil)
            MessageInputView(text: $inputText, onSend: sendMessage)
        }
        .frame(minWidth: 720, minHeight: 480)
    }

    @ViewBuilder
    private func renderToolbar() -> some View {
        HStack(spacing: 8) {
            // Model menu
            Menu {
                ForEach(models, id: \.self) { m in
                    Button(action: { selectedModel = m }) {
                        Text(m)
                    }
                }
            } label: {
                HStack(spacing: 4) {
                    Image(systemName: "slider.horizontal.3")
                    Text(selectedModel).lineLimit(1)
                }
            }
            .help("Select Model")

            // Deep thinking toggle
            Button(action: { deepThinkEnabled.toggle() }) {
                Image(systemName: deepThinkEnabled ? "lightbulb.fill" : "lightbulb")
            }
            .buttonStyle(.plain)
            .help("Deep Thinking")

            // Internet access toggle
            Button(action: { internetEnabled.toggle() }) {
                Image(systemName: internetEnabled ? "globe" : "globe.badge.exclamationmark")
            }
            .buttonStyle(.plain)
            .help("Internet Access")

            Spacer()

            // Upload/attach
            Button(action: { openFilePicker() }) {
                Image(systemName: "doc.badge.plus")
            }
            .buttonStyle(.plain)
            .help("Upload Image or Document")
        }
    }

    private func sendMessage() {
        let trimmed = inputText.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }

        messages.append(MessageModel(role: .user, content: trimmed))
        inputText = ""

        // Start streaming placeholder
        isStreaming = true
        streamingText = ""
        let stream = service.streamMessage(messages: messages)
        Task {
            for await chunk in stream {
                streamingText += chunk
            }
            isStreaming = false
            let final = streamingText
            streamingText = ""

            // Append assistant message
            let blocks = MessageParser.parseBlocks(from: final)
            messages.append(MessageModel(role: .assistant, content: final, blocks: blocks))

            // Fetch full response to include tool calls when available
            do {
                // Use selected model for the request (hint; service reads env; we could set via env or extend service)
                let resp = try await service.sendMessage(messages: messages)
                if !resp.toolCalls.isEmpty {
                    let toolBlocks = resp.toolCalls.map { ContentBlock.tool($0) }
                    messages.append(MessageModel(role: .assistant, content: "", blocks: toolBlocks))
                }
            } catch {
                messages.append(MessageModel(role: .assistant, content: "Error: \(error.localizedDescription)"))
            }
        }
    }

    private func openFilePicker() {
        let panel = NSOpenPanel()
        panel.allowsMultipleSelection = false
        panel.canChooseFiles = true
        panel.canChooseDirectories = false
        panel.allowedFileTypes = ["png", "jpg", "jpeg", "gif", "pdf", "txt", "md"]
        panel.begin { response in
            if response == .OK, let url = panel.url {
                messages.append(MessageModel(role: .user, content: "Attached: \(url.lastPathComponent)"))
            }
        }
    }
}

struct MessageParser {
    static func parseBlocks(from content: String) -> [ContentBlock] {
        var blocks: [ContentBlock] = []
        let parts = content.components(separatedBy: "```")
        for (idx, part) in parts.enumerated() {
            if idx % 2 == 0 {
                if !part.isEmpty {
                    blocks.append(.text(part))
                }
            } else {
                // Code block: optional "lang\ncode"
                if let newlineIdx = part.firstIndex(of: "\n") {
                    let lang = String(part[..<newlineIdx]).trimmingCharacters(in: .whitespacesAndNewlines)
                    let code = String(part[part.index(after: newlineIdx)...])
                    blocks.append(.code(language: lang.isEmpty ? "text" : lang, code: code))
                } else {
                    blocks.append(.code(language: "text", code: part))
                }
            }
        }
        return blocks
    }
}
