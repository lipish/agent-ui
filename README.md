# Agent UI

A modular AI assistant interface built with GPUI, featuring a clean Yuanbao-inspired design.

**Version:** 2.0.0  
**Status:** Active Development

---

## ✨ Features

- 🎨 **Yuanbao Style Design** - Minimalist gray and white color scheme
- 🏗️ **Modular Architecture** - Clean separation of concerns
- ✈️ **SVG Icons** - Heroicons integration with proper asset management
- 📱 **Smart Positioning** - Right-aligned window with screen adaptation
- 💬 **Message Flow** - User and assistant message display

---

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable)
- macOS (GPUI currently supports macOS)

### Build and Run

```bash
# Clone the repository
git clone <repository-url>
cd agent-ui

# Build
cargo build

# Run
cargo run
```

The window will appear on the right side of your screen with a 20pt margin.

---

## 📁 Project Structure

```
agent-ui/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # Application entry (50 lines)
│   ├── assets.rs           # Asset management (SVG icons)
│   ├── theme.rs            # Theme configuration
│   ├── utils.rs            # Utility functions
│   ├── models/             # Data models
│   │   └── message.rs      # Message, MessageRole
│   ├── state/              # State management
│   │   └── conversation.rs # ConversationState
│   └── components/         # UI components
│       ├── assistant_panel.rs
│       ├── message_list.rs
│       └── message_input.rs
├── assets/
│   └── icons/
│       └── paper-airplane.svg
├── doc/                    # Documentation
│   ├── README.md           # Documentation index
│   ├── system-design.md    # System architecture
│   └── ...
└── Cargo.toml
```

---

## 🏗️ Architecture

### Modular Design

The project follows a modular architecture with clear separation of concerns:

- **assets** - Resource loading (SVG icons, images)
- **models** - Data structures (Message, MessageRole)
- **state** - State management (ConversationState)
- **theme** - UI theming (Yuanbao style)
- **components** - UI components (Panel, List, Input)
- **utils** - Utility functions (window positioning)

### Design Principles

- ✅ Single Responsibility Principle (SRP)
- ✅ Open/Closed Principle (OCP)
- ✅ Dependency Inversion Principle (DIP)
- ✅ Interface Segregation Principle (ISP)

See [doc/MODULAR-REFACTORING.md](doc/MODULAR-REFACTORING.md) for details.

---

## 🎨 Design

### Yuanbao Style

Inspired by minimalist design, featuring:

- Pure white background (`#ffffff`)
- Light gray message bubbles (`#f5f5f5`)
- Dark primary color (`#1a1a1a`)
- Fully rounded input box
- Clean, professional appearance

### Window Positioning

- Right-aligned with 20pt margin
- Adapts to different screen sizes
- 16:9 aspect ratio (600x1067)

---

## 📚 Documentation

See [doc/README.md](doc/README.md) for complete documentation index.

### Key Documents

- [system-design.md](doc/system-design.md) - System architecture (English)
- [system-design_CN.md](doc/system-design_CN.md) - 系统架构 (中文)
- [MODULAR-REFACTORING.md](doc/MODULAR-REFACTORING.md) - Modular design guide
- [implementation-roadmap.md](doc/implementation-roadmap.md) - Development roadmap

---

## 🛠️ Technology Stack

- **UI Framework:** [GPUI](https://github.com/zed-industries/zed) - Zed's GPU-accelerated UI framework
- **Language:** Rust
- **Icons:** [Heroicons](https://heroicons.com/)
- **Async Runtime:** Tokio

---

## 🗺️ Roadmap

### Current (v2.0.0)
- ✅ Modular architecture
- ✅ Yuanbao style UI
- ✅ SVG icon support
- ✅ Basic message flow

### Next (v2.1.0)
- ⏳ Real text input
- ⏳ Keyboard events (Enter to send)
- ⏳ Scroll to bottom

### Future (v3.0.0)
- 📋 Async AI responses
- 📋 Tool call display
- 📋 Task management
- 📋 Markdown rendering
- 📋 Code highlighting

See [implementation-roadmap.md](doc/implementation-roadmap.md) for details.

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Read [system-design.md](doc/system-design.md) to understand the architecture
2. Follow the modular design principles
3. Keep documentation up to date
4. Write clear commit messages

---

## 📄 License

[Add your license here]

---

## 🙏 Acknowledgments

- [GPUI](https://github.com/zed-industries/zed) - Amazing GPU-accelerated UI framework
- [Heroicons](https://heroicons.com/) - Beautiful SVG icons
- Yuanbao - Design inspiration

---

**Built with ❤️ using Rust and GPUI**

