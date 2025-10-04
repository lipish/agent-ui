# Agent UI

A modular AI assistant interface with multiple framework implementations.

**Version:** 2.1.0
**Status:** Active Development

---

## 📁 Project Structure

This repository contains different implementations of Agent UI:

```
agent-ui/
├── gpui/                        # GPUI framework implementation (Rust)
│   ├── src/                     # Source code
│   ├── scripts/                 # Development scripts
│   ├── Cargo.toml               # Rust project config
│   ├── dev                      # Quick start script
│   └── README.md                # GPUI documentation
├── web/                         # Web implementation (Svelte)
│   ├── src/                     # Source code
│   │   └── components/          # Svelte components
│   ├── package.json             # Dependencies
│   ├── dev.sh                   # Quick start script
│   └── README.md                # Web documentation
├── doc/                         # Documentation
├── images/                      # Design references
└── README.md                    # This file
```

---

## 🚀 Quick Start

### GPUI Version (Rust)

Native desktop application with high performance.

```bash
cd gpui
./dev
```

See [gpui/README.md](gpui/README.md) for details.

### Web Version (Svelte)

Modern web application with clean UI.

```bash
cd web
npm install
npm run dev
```

Open `http://localhost:5173`

See [web/README.md](web/README.md) for details.

---

## ✨ Features

### Common Features
- 🎨 Clean, minimalist design
- 💬 Message list with auto-scroll
- ✍️ Text input with send button
- ⌨️ Keyboard shortcuts (Enter to send)
- 📱 9:16 aspect ratio (450×800)

### GPUI Version
- ⚡ Native performance
- 🖥️ Desktop application
- 🦀 Built with Rust
- 🎯 Right-side positioning

### Web Version
- 🌐 Cross-platform
- 📱 Responsive design
- ⚡ Fast and lightweight
- 🎨 Smooth animations

---

## 🎨 Design

- **Window Size**: 450px × 800px
- **Aspect Ratio**: 9:16 (vertical/portrait)
- **Color Scheme**: Clean, modern
- **Typography**: System fonts

Design references in `images/` directory.

---

## 🛠️ Technology Stack

### GPUI Version
- **Language**: Rust
- **Framework**: GPUI (from Zed editor)
- **Platform**: macOS (currently)

### Web Version
- **Language**: JavaScript
- **Framework**: Svelte
- **Build Tool**: Vite
- **Platform**: Web browsers

---

## 📚 Documentation

- **GPUI Version**: [gpui/README.md](gpui/README.md)
- **Web Version**: [web/README.md](web/README.md)
- **Directory Setup**: [GPUI_DIRECTORY_SETUP.md](GPUI_DIRECTORY_SETUP.md)
- **Root Cleanup**: [ROOT_CLEANUP_COMPLETE.md](ROOT_CLEANUP_COMPLETE.md)

---

## 🎯 Roadmap

### Current
- ✅ GPUI desktop version
- ✅ Web version with Svelte

### Future
- [ ] Tauri version (desktop + web)
- [ ] Electron version
- [ ] Mobile apps (React Native / Flutter)
- [ ] Real AI backend integration
- [ ] Authentication
- [ ] Conversation history
- [ ] File upload support

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

---

## 📄 License

MIT License

---

## ✨ Get Started

Choose your preferred version:

**GPUI (Native Desktop)**:
```bash
cd gpui && ./dev
```

**Web (Browser)**:
```bash
cd web && npm install && npm run dev
```

Happy coding! 🚀

