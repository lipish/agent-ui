import SwiftUI
import AppKit

struct MessageInputView: View {
    @Binding var text: String
    var onSend: () -> Void
    @State private var selectedModel: String = "openai/gpt-4"
    private let models: [String] = ["openai/gpt-4", "openai/gpt-4o", "anthropic/claude-3.5", "custom/my-model"]
    @State private var showModelMenu: Bool = false

    var body: some View {
        VStack(spacing: 0) {
            // Unified pill container following minimalist style
            VStack(spacing: 10) {
                ZStack(alignment: .leading) {
                    NoScrollTextEditor(text: $text)
                        .frame(minHeight: 48, maxHeight: 120)
                        .padding(.horizontal, 4)
                }
                
                HStack(spacing: 12) {

                    // Model selector: text pill + tight arrow capsule
                    HStack(spacing: 2) {
                        Text(selectedModel)
                            .foregroundColor(.primary)
                            .padding(.horizontal, 8)
                            .padding(.vertical, 6)
                            .background(Color(NSColor.controlBackgroundColor).opacity(0.85))
                            .clipShape(RoundedRectangle(cornerRadius: 12, style: .continuous))

                        Button(action: { showModelMenu.toggle() }) {
                            Image(systemName: "chevron.down")
                                .font(.system(size: 11, weight: .semibold))
                                .foregroundColor(.white)
                                .frame(width: 18, height: 18)
                                .background(Color.black)
                                .clipShape(RoundedRectangle(cornerRadius: 6, style: .continuous))
                        }
                        .buttonStyle(.plain)
                        .overlay(alignment: .top) {
                            if showModelMenu {
                                VStack(alignment: .leading, spacing: 6) {
                                    ForEach(models, id: \.self) { m in
                                        Button(action: {
                                            selectedModel = m
                                            showModelMenu = false
                                        }) {
                                            Text(m)
                                                .foregroundColor(.primary)
                                                .padding(.horizontal, 8)
                                                .padding(.vertical, 6)
                                        }
                                        .buttonStyle(.plain)
                                    }
                                }
                                .padding(8)
                                .background(Color(NSColor.controlBackgroundColor))
                                .clipShape(RoundedRectangle(cornerRadius: 10, style: .continuous))
                                .shadow(color: Color.black.opacity(0.08), radius: 8, x: 0, y: 3)
                                .offset(y: -5)
                                .zIndex(1000)
                            }
                        }
                    }
                    .frame(minWidth: 90)
                    .help("Select Model")
                    
                    Spacer()
                    
                    Button {
                        let panel = NSOpenPanel()
                        panel.allowsMultipleSelection = false
                        panel.canChooseFiles = true
                        panel.canChooseDirectories = false
                        panel.allowedFileTypes = ["png", "jpg", "jpeg", "gif", "pdf", "txt", "md"]
                        panel.begin { response in
                            if response == .OK, let url = panel.url {
                                text = (text.isEmpty ? "" : text + "\n") + "附件: \(url.lastPathComponent)"
                            }
                        }
                    } label: { Image(systemName: "paperclip").font(.system(size: 14)) }
                    .buttonStyle(.plain)
                    
                    Button { onSend() } label: { Image(systemName: "arrow.up.circle.fill").font(.system(size: 20)) }
                    .buttonStyle(.plain)
                    .foregroundColor(.accentColor)
                    .disabled(text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                    .keyboardShortcut(.return, modifiers: [.command])
                }
            }
            .padding(12)
            .background(Color.white)
            .clipShape(RoundedRectangle(cornerRadius: 24, style: .continuous))
            .shadow(color: Color.black.opacity(0.03), radius: 6, x: 0, y: 1)
            .padding(.horizontal, 16)
            .padding(.vertical, 12)
        }
    }
}

