# Klippy - macOS Clipboard Manager

A minimal, fast clipboard manager for macOS. Lives in your menu bar, remembers your last 25 clipboard items, and provides instant keyboard shortcuts.

## ✨ Features

- 📎 **25 clipboard items** stored in memory
- ⌨️ **Keyboard shortcuts** - Cmd+0-9 to select items, then Cmd+V to paste
- 🎯 **Menu bar app** - Clean, simple interface with paperclip icon
- 🚀 **Fast** - Built in Rust, lightweight and efficient
- 🔒 **Private** - Everything stays local, no cloud sync, no analytics
- 🎨 **Smart** - Handles text and images, no duplicates from pasting

## 📦 Installation

### Requirements

- **macOS 10.15+** (Catalina or later)
- **Rust toolchain** - Install via [rustup](https://rustup.rs):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Quick Install

```bash
git clone https://github.com/yourusername/klippy.git
cd klippy
chmod +x install.sh
./install.sh
```

The installer will:
1. ✅ Check for Rust (guide you if missing)
2. 🔨 Build the release binary
3. 🎨 Generate app icon
4. 📱 Create macOS app bundle
5. 📂 Install to `/Applications/Klippy.app`

### Grant Permissions (Required)

Klippy needs **Accessibility** permissions to register global hotkeys:

1. Open **System Settings**
2. Navigate to **Privacy & Security → Accessibility**
3. Click the **+** button
4. Select **Klippy** from `/Applications`
5. Enable the checkbox next to Klippy

### Launch

```bash
open /Applications/Klippy.app
```

Or open from Applications folder in Finder. Look for the 📎 icon in your menu bar!

## ⌨️ How It Works

Klippy uses a **two-step workflow** for pasting:

1. **Select** an item with **Cmd+0** through **Cmd+9**
2. **Paste** it with **Cmd+V**

### Keyboard Shortcuts

- **Cmd+0** - Select most recent clipboard item
- **Cmd+1** - Select 2nd most recent item
- **Cmd+2** - Select 3rd most recent item
- **...**
- **Cmd+9** - Select 10th most recent item
- **Cmd+V** - Paste the currently selected item

### Menu Bar

Click the **📎** icon to:
- See all 25 clipboard items
- Click any item to select it (same as Cmd+0-9)
- Clear all history
- Quit the app

## 🎮 Usage Example

1. Copy some text: "Hello World" → stored as item 0
2. Copy more text: "Goodbye" → stored as item 0, "Hello World" moves to item 1
3. Press **Cmd+1** → selects "Hello World"
4. Press **Cmd+V** → pastes "Hello World"
5. Press **Cmd+0** → selects "Goodbye"
6. Press **Cmd+V** → pastes "Goodbye"

**Note:** When you select an item with Cmd+0-9, it doesn't auto-paste. This lets you position your cursor first, then paste with Cmd+V.

## 🛠️ Development

### Build from Source

```bash
# Clone the repo
git clone https://github.com/yourusername/klippy.git
cd klippy

# Build release binary
cargo build --release

# Run the binary
./target/release/klippy
```

### Project Structure

```
klippy/
├── src/
│   ├── main.rs          # App entry point, UI, event handling
│   ├── clipboard.rs     # Clipboard monitoring and management
│   └── storage.rs       # In-memory clipboard history storage
├── install.sh           # Installation script
├── build-app.sh         # Manual app bundle builder
└── Cargo.toml           # Rust dependencies
```

### Architecture

- **Event Loop**: Uses `winit` for cross-platform event handling
- **Tray Icon**: Built with `tray-icon` for menu bar integration
- **Global Hotkeys**: `global-hotkey` crate for Cmd+0-9 shortcuts
- **Clipboard**: `arboard` for clipboard access (text + images)
- **Storage**: Simple in-memory Vec, FIFO with 25-item limit

## 🗑️ Uninstall

```bash
rm -rf /Applications/Klippy.app
```

## 📝 License

MIT License - Free to use, modify, and distribute.

**Attribution appreciated but not required.** If you fork or build upon this, a link back is nice but optional.

## 🤝 Contributing

Contributions welcome! Please:
- Keep it simple and lightweight
- Follow existing code style
- Test on macOS before submitting
- One feature per PR

## 🐛 Issues & Feature Requests

Open an issue on GitHub if you find bugs or have ideas for improvements.

## ⭐ Support

If you find Klippy useful, consider:
- ⭐ Starring the repo
- 🐦 Sharing it with others
- 🤝 Contributing improvements
