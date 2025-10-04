import SwiftUI

struct MessageListView: View {
    var messages: [MessageModel]
    var streamingText: String?

    var body: some View {
        ScrollViewReader { proxy in
            ScrollView {
                LazyVStack(alignment: .leading, spacing: 12) {
                    ForEach(messages) { msg in
                        MessageBubbleView(message: msg)
                            .id(msg.id)
                    }
                    if let streaming = streamingText, !streaming.isEmpty {
                        StreamingBubbleView(text: streaming)
                    }
                }
                .padding(.horizontal, 16)
                .padding(.vertical, 12)
            }
            .onChange(of: messages.count) { _ in
                if let last = messages.last {
                    withAnimation { proxy.scrollTo(last.id, anchor: .bottom) }
                }
            }
        }
        .background(Color(NSColor.windowBackgroundColor))
    }
}

struct StreamingBubbleView: View {
    var text: String
    var body: some View {
        HStack(alignment: .top, spacing: 8) {
            Circle().fill(Color.accentColor.opacity(0.9)).frame(width: 20, height: 20)
            VStack(alignment: .leading, spacing: 8) {
                Text("Assistant")
                    .font(.caption).bold()
                Text(text)
                    .font(.body)
                    .foregroundColor(.secondary)
            }
            Spacer()
        }
        .padding(12)
        .background(Color.secondary.opacity(0.08))
        .cornerRadius(8)
    }
}
