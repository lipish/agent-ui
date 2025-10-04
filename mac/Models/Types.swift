import Foundation

enum Role: String, Codable {
    case user
    case assistant
    case tool
}

enum ToolStatus: Codable {
    case pending
    case running
    case success
    case error(String)
}

struct ToolCall: Identifiable, Codable {
    var id: String
    var name: String
    var arguments: [String: AnyCodable]
    var status: ToolStatus = .pending
    var result: String?
    var startedAt: Date?
    var completedAt: Date?
}

enum ContentBlock: Codable, Identifiable {
    case text(String)
    case code(language: String, code: String)
    case tool(ToolCall)

    var id: UUID { UUID() }
}

struct MessageModel: Identifiable, Codable {
    var id: UUID = UUID()
    var role: Role
    var content: String
    var blocks: [ContentBlock] = []
    var timestamp: Date = Date()
}

struct AgentResponse: Codable {
    var content: String
    var toolCalls: [ToolCall]
    var finishReason: String
}

/// Codable wrapper to allow heterogenous parameters dictionaries
struct AnyCodable: Codable {
    let value: Any

    init(_ value: Any) { self.value = value }

    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if let v = try? container.decode(Bool.self) { value = v; return }
        if let v = try? container.decode(Int.self) { value = v; return }
        if let v = try? container.decode(Double.self) { value = v; return }
        if let v = try? container.decode(String.self) { value = v; return }
        if let v = try? container.decode([String: AnyCodable].self) {
            value = v.mapValues { $0.value }; return
        }
        if let v = try? container.decode([AnyCodable].self) {
            value = v.map { $0.value }; return
        }
        value = NSNull()
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch value {
        case let v as Bool: try container.encode(v)
        case let v as Int: try container.encode(v)
        case let v as Double: try container.encode(v)
        case let v as String: try container.encode(v)
        case let v as [String: Any]:
            try container.encode(v.mapValues(AnyCodable.init))
        case let v as [Any]:
            try container.encode(v.map(AnyCodable.init))
        default:
            try container.encodeNil()
        }
    }
}
