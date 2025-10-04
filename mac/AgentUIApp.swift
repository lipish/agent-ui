import AppKit
import SwiftUI

final class AppDelegate: NSObject, NSApplicationDelegate {
    var window: NSWindow?
    private var statusItem: NSStatusItem?

    func applicationDidFinishLaunching(_ notification: Notification) {
        setupStatusBar()
        showMainWindow()
        NSApp.activate(ignoringOtherApps: true)
    }

    private func setupStatusBar() {
        let item = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        item.button?.title = "Agent UI"
        let menu = NSMenu()
        let openItem = NSMenuItem(title: "Open Agent UI", action: #selector(openWindow), keyEquivalent: "o")
        openItem.target = self
        menu.addItem(openItem)
        let quitItem = NSMenuItem(title: "Quit", action: #selector(quitApp), keyEquivalent: "q")
        quitItem.target = self
        menu.addItem(quitItem)
        item.menu = menu
        self.statusItem = item
    }

    @objc private func openWindow() {
        showMainWindow()
        NSApp.activate(ignoringOtherApps: true)
    }

    private func targetFrame(for screen: NSScreen?, style: NSWindow.StyleMask, margin: CGFloat) -> NSRect {
        let visible = screen?.visibleFrame ?? NSScreen.main?.visibleFrame ?? NSRect(x: 0, y: 0, width: 900, height: 1600)
        let maxContentHeight = max(visible.height - 2 * margin, 200)
        let maxContentWidth = max(visible.width - 2 * margin, 200)

        // Desired 9:16 aspect ratio
        var contentHeight = min(maxContentHeight, 1200)
        var contentWidth = contentHeight * 9.0 / 16.0

        // If width exceeds available, shrink to fit and recompute height
        if contentWidth > maxContentWidth {
            contentWidth = maxContentWidth
            contentHeight = contentWidth * 16.0 / 9.0
            if contentHeight > maxContentHeight {
                contentHeight = maxContentHeight
                contentWidth = contentHeight * 9.0 / 16.0
            }
        }

        let frameSize = NSWindow.frameRect(forContentRect: NSRect(x: 0, y: 0, width: contentWidth, height: contentHeight), styleMask: style).size
        var x = visible.minX + margin
        var y = visible.midY - frameSize.height / 2.0
        if x + frameSize.width > visible.maxX - margin { x = visible.maxX - margin - frameSize.width }
        if y < visible.minY + margin { y = visible.minY + margin }
        if y + frameSize.height > visible.maxY - margin { y = visible.maxY - margin - frameSize.height }
        return NSRect(x: x, y: y, width: frameSize.width, height: frameSize.height)
    }

    private func positionWindow(_ win: NSWindow) {
        let margin: CGFloat = 20
        let style: NSWindow.StyleMask = [.titled, .closable, .miniaturizable, .resizable]
        let target = targetFrame(for: win.screen, style: style, margin: margin)
        win.setFrame(target, display: true, animate: true)
    }

    private func showMainWindow() {
        let margin: CGFloat = 20
        let style: NSWindow.StyleMask = [.titled, .closable, .miniaturizable, .resizable]

        if let win = window {
            positionWindow(win)
            win.makeKeyAndOrderFront(nil)
            return
        }

        let contentView = AssistantPanelView()
        // Create with a temporary rect; will be positioned immediately after
        let tempRect = NSRect(x: 0, y: 0, width: 600, height: 800)
        let win = NSWindow(
            contentRect: tempRect,
            styleMask: style,
            backing: .buffered,
            defer: false
        )
        win.title = "Agent UI (mac)"
        win.contentView = NSHostingView(rootView: contentView)
        let target = targetFrame(for: win.screen, style: style, margin: margin)
        win.setFrame(target, display: true)

        self.window = win
        window?.makeKeyAndOrderFront(nil)
    }

    @objc private func quitApp() {
        NSApp.terminate(nil)
    }
}

@main
struct AppMain {
    static func main() {
        let app = NSApplication.shared
        let delegate = AppDelegate()
        app.delegate = delegate
        app.setActivationPolicy(.regular)
        app.run()
    }
}
