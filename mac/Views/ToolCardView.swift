import SwiftUI

struct ToolCardView: View {
    var toolCall: ToolCall
    @State private var expanded: Bool = false

    var statusText: String {
        switch toolCall.status {
        case .pending: return "Pending"
        case .running: return "Running"
        case .success: return "Success"
        case .error(let msg): return "Error: \(msg)"
        }
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack(spacing: 8) {
                Image(systemName: "wrench.and.screwdriver")
                Text(toolCall.name).fontWeight(.semibold)
                Spacer()
                Text(statusText).font(.caption).foregroundColor(.secondary)
                Button(action: { expanded.toggle() }) {
                    Image(systemName: expanded ? "chevron.up" : "chevron.down")
                }.buttonStyle(.plain)
            }
            .padding(8)
            .background(Color.secondary.opacity(0.12))
            .cornerRadius(6)

            if expanded {
                VStack(alignment: .leading, spacing: 6) {
                    Text("Parameters").font(.caption).bold()
                    Text(renderArguments(toolCall.arguments))
                        .font(.system(size: 13, weight: .regular, design: .monospaced))
                        .foregroundColor(.primary)
                        .padding(8)
                        .background(Color(NSColor.textBackgroundColor))
                        .cornerRadius(6)

                    if let result = toolCall.result {
                        Text("Result").font(.caption).bold()
                        Text(result)
                            .font(.system(size: 13, weight: .regular, design: .monospaced))
                            .foregroundColor(.primary)
                            .padding(8)
                            .background(Color(NSColor.textBackgroundColor))
                            .cornerRadius(6)
                    }
                }
                .padding(.horizontal, 4)
            }
        }
        .padding(8)
        .background(Color.secondary.opacity(0.08))
        .cornerRadius(8)
    }

    private func renderArguments(_ args: [String: AnyCodable]) -> String {
        let dict = args.mapValues { $0.value }
        if let data = try? JSONSerialization.data(withJSONObject: dict, options: [.prettyPrinted]),
           let s = String(data: data, encoding: .utf8) {
            return s
        }
        return "\(dict)"
    }
}
