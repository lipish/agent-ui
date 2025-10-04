import SwiftUI

struct MessageBubbleView: View {
    var message: MessageModel

    var isUser: Bool { message.role == .user }

    var body: some View {
        HStack(alignment: .top, spacing: 8) {
            VStack(alignment: .leading, spacing: 8) {
                Text(isUser ? "You" : "Assistant")
                    .font(.caption).bold()
                VStack(alignment: .leading, spacing: 8) {
                    if message.blocks.isEmpty {
                        Text(message.content)
                            .font(.body)
                    } else {
                        ForEach(message.blocks) { block in
                            switch block {
                            case .text(let t):
                                Text(t)
                                    .font(.body)
                            case .code(let lang, let code):
                                CodeBlockView(language: lang, code: code)
                            case .tool(let call):
                                ToolCardView(toolCall: call)
                            }
                        }
                    }
                }
            }
            Spacer()
        }
        .padding(.horizontal, 8)
        .padding(.vertical, 6)
    }
}
