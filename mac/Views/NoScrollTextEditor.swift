import SwiftUI
import AppKit

struct NoScrollTextEditor: NSViewRepresentable {
    @Binding var text: String

    class Coordinator: NSObject, NSTextViewDelegate {
        var parent: NoScrollTextEditor
        init(_ parent: NoScrollTextEditor) { self.parent = parent }
        func textDidChange(_ notification: Notification) {
            if let tv = notification.object as? NSTextView {
                parent.text = tv.string
            }
        }
    }

    func makeCoordinator() -> Coordinator { Coordinator(self) }

    func makeNSView(context: Context) -> NSScrollView {
        let textView = NSTextView()
        textView.isEditable = true
        textView.isRichText = false
        textView.font = NSFont.systemFont(ofSize: 14)
        textView.textColor = NSColor.labelColor
        textView.insertionPointColor = NSColor.labelColor
        textView.drawsBackground = false
        textView.delegate = context.coordinator
        textView.textContainerInset = NSSize(width: 0, height: 6)

        let scrollView = NSScrollView()
        scrollView.hasVerticalScroller = false
        scrollView.hasHorizontalScroller = false
        scrollView.borderType = .noBorder
        scrollView.drawsBackground = false
        scrollView.documentView = textView
        return scrollView
    }

    func updateNSView(_ nsView: NSScrollView, context: Context) {
        if let tv = nsView.documentView as? NSTextView {
            if tv.string != text { tv.string = text }
            if tv.textColor != NSColor.labelColor { tv.textColor = NSColor.labelColor }
            if tv.insertionPointColor != NSColor.labelColor { tv.insertionPointColor = NSColor.labelColor }
        }
    }
}
