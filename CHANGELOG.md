# Changelog

All notable changes to this project will be documented in this file.

## [2.0.0] - 2025-10-02

### 🎉 Major Refactoring - Modular Architecture

#### Added
- **Modular code structure** with clear separation of concerns
  - `src/lib.rs` - Library entry point
  - `src/assets.rs` - Asset management (SVG icons)
  - `src/models/` - Data models (Message, MessageRole)
  - `src/state/` - State management (ConversationState)
  - `src/theme.rs` - Theme configuration (Yuanbao style)
  - `src/components/` - UI components (Panel, List, Input)
  - `src/utils.rs` - Utility functions

- **Documentation cleanup**
  - Removed 21 outdated/temporary documents
  - Kept 11 essential design documents
  - Added `doc/README.md` as documentation index
  - Added `README.md` for project overview

- **SVG icon support**
  - Proper AssetSource implementation
  - Heroicons paper-airplane icon
  - Icon path constants in `assets.rs`

#### Changed
- **main.rs** reduced from 266 lines to 50 lines
- **Code organization** follows SOLID principles
- **Window positioning** moved to `utils.rs`
- **Theme configuration** extracted to `theme.rs`

#### Removed
- All backup files (main_*.rs, simple_main.rs)
- Unused modules (old models/, state/, ui/)
- 21 outdated documentation files

### 🎨 Design

#### Features
- Yuanbao-inspired minimalist design
- Pure white background (#ffffff)
- Light gray message bubbles (#f5f5f5)
- Dark primary color (#1a1a1a)
- Fully rounded input box
- Right-aligned window with 20pt margin

#### UI Components
- AssistantPanel - Main container
- MessageList - Message display
- MessageInput - Input box and send button

### 📊 Statistics

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Source files | 1 | 11 | +10 |
| main.rs lines | 266 | 50 | -81% |
| Modules | 0 | 6 | +6 |
| Documentation | 32 | 13 | -59% |

### 🚀 Technical

#### Dependencies
- GPUI (from Zed repository)
- Tokio (async runtime)
- anyhow (error handling)

#### Architecture
- Single Responsibility Principle (SRP)
- Open/Closed Principle (OCP)
- Dependency Inversion Principle (DIP)
- Interface Segregation Principle (ISP)

---

## [1.0.0] - 2025-10-02

### Initial Release

#### Features
- Basic message display
- Mock AI responses
- Yuanbao style UI
- SVG icon support
- Window positioning

#### Known Limitations
- Input box is static (not editable)
- No keyboard events
- No async AI responses
- No tool call display

---

## Roadmap

### [2.1.0] - Planned
- Real text input
- Keyboard events (Enter to send)
- Scroll to bottom
- Loading states

### [3.0.0] - Future
- Async AI responses
- Tool call display
- Task management
- Markdown rendering
- Code highlighting
- Multi-conversation support

---

**Version Format:** [Major.Minor.Patch]
- **Major:** Breaking changes, major refactoring
- **Minor:** New features, non-breaking changes
- **Patch:** Bug fixes, documentation updates

