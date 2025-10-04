import Foundation

struct AgentConfig {
    var baseURL: URL?
    var apiKey: String?
    var model: String

    init() {
        let env = ProcessInfo.processInfo.environment
        if let base = env["CODE_AGENT_BASE_URL"], let url = URL(string: base) {
            baseURL = url
        } else {
            baseURL = nil
        }
        apiKey = env["CODE_AGENT_API_KEY"]
        model = env["CODE_AGENT_MODEL"] ?? "openai/gpt-4"
    }
}

final class AgentService {
    private let config = AgentConfig()
    private let session = URLSession(configuration: .default)

    func sendMessage(messages: [MessageModel]) async throws -> AgentResponse {
        // If backend not configured, return stub
        guard let base = config.baseURL else {
            let content = "Stub response: configure CODE_AGENT_BASE_URL to integrate with code-agent backend. Model: \(config.model)"
            return AgentResponse(content: content, toolCalls: [], finishReason: "stop")
        }

        let url = base.appendingPathComponent("chat")
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        if let apiKey = config.apiKey {
            request.setValue("Bearer \(apiKey)", forHTTPHeaderField: "Authorization")
        }

        let payload: [String: Any] = [
            "model": config.model,
            "messages": messages.map { ["role": $0.role.rawValue, "content": $0.content] },
            "stream": false
        ]
        request.httpBody = try JSONSerialization.data(withJSONObject: payload, options: [])

        let (data, _) = try await session.data(for: request)
        let decoded = try JSONDecoder().decode(AgentResponse.self, from: data)
        return decoded
    }

    // REPLACED: streamMessage placeholder with SSE implementation (fallback to stub)
    func streamMessage(messages: [MessageModel]) -> AsyncStream<String> {
        // If backend configured, try SSE streaming at /chat/stream
        if let base = config.baseURL {
            let url = base.appendingPathComponent("chat/stream")
            var request = URLRequest(url: url)
            request.httpMethod = "POST"
            request.setValue("text/event-stream", forHTTPHeaderField: "Accept")
            request.setValue("application/json", forHTTPHeaderField: "Content-Type")
            if let apiKey = config.apiKey {
                request.setValue("Bearer \(apiKey)", forHTTPHeaderField: "Authorization")
            }
            let payload: [String: Any] = [
                "model": config.model,
                "messages": messages.map { ["role": $0.role.rawValue, "content": $0.content] },
                "stream": true
            ]
            request.httpBody = try? JSONSerialization.data(withJSONObject: payload, options: [])

            return AsyncStream { continuation in
                Task {
                    do {
                        // bytes(for:) yields (URLSession.AsyncBytes, URLResponse)
                        let (bytes, _) = try await session.bytes(for: request)
                        var buffer = Data()
                        for try await chunk in bytes {
                            buffer.append(chunk)
                            // SSE frames are separated by \n\n; lines prefixed with "data:"
                            while let range = buffer.range(of: "\n\n".data(using: .utf8)!) {
                                let frame = buffer.subdata(in: 0..<range.lowerBound)
                                buffer.removeSubrange(0..<range.upperBound)
                                if let line = String(data: frame, encoding: .utf8) {
                                    for l in line.split(separator: "\n") {
                                        if l.hasPrefix("data:") {
                                            let dataPart = l.dropFirst(5).trimmingCharacters(in: .whitespaces)
                                            if !dataPart.isEmpty {
                                                continuation.yield(String(dataPart))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } catch {
                        // Fallback: yield error text then finish
                        continuation.yield("[stream-error] \(error.localizedDescription)")
                    }
                    continuation.finish()
                }
            }
        }

        // Fallback stub stream when backend not configured
        return AsyncStream { continuation in
            let parts = ["Analyzing", " your", " request", "..."]
            Task {
                for p in parts {
                    try await Task.sleep(nanoseconds: 300_000_000)
                    continuation.yield(p)
                }
                continuation.finish()
            }
        }
    }
}
