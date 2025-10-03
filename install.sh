#!/bin/bash

set -e

echo "🎯 Installing Klippy - Clipboard Manager for macOS"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed."
    echo ""
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "Then run this script again."
    exit 1
fi

echo "✅ Rust found"
echo ""

# Build the release binary
echo "📦 Building Klippy (this may take a minute)..."
cargo build --release

# Create app bundle
APP_NAME="Klippy"
APP_DIR="$APP_NAME.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

echo "📱 Creating macOS app bundle..."
rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Copy binary
cp target/release/klippy "$MACOS_DIR/$APP_NAME"
chmod +x "$MACOS_DIR/$APP_NAME"

# Create a simple icon using macOS built-in tools
echo "🎨 Creating app icon..."

# Create a simple SVG icon with paperclip emoji
cat > icon.svg << 'SVG'
<?xml version="1.0" encoding="UTF-8"?>
<svg width="1024" height="1024" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="grad" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" style="stop-color:#3B82F6;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#1D4ED8;stop-opacity:1" />
    </linearGradient>
  </defs>
  <rect width="1024" height="1024" rx="180" fill="url(#grad)"/>
  <text x="512" y="700" font-size="600" text-anchor="middle" fill="white">📎</text>
</svg>
SVG

# Convert SVG to PNG using qlmanage or sips
if command -v qlmanage &> /dev/null; then
    qlmanage -t -s 1024 -o . icon.svg >/dev/null 2>&1
    if [ -f "icon.svg.png" ]; then
        mv icon.svg.png icon.png
    fi
fi

# If PNG doesn't exist, create a simple colored square
if [ ! -f "icon.png" ]; then
    # Create a simple blue icon using sips
    sips -s format png --out icon.png icon.svg >/dev/null 2>&1 || {
        # Fallback: create minimal icon data
        echo "⚠️  Using simplified icon"
    }
fi

# Convert to ICNS if we have an icon
if [ -f "icon.png" ]; then
    mkdir -p icon.iconset
    sips -z 16 16     icon.png --out icon.iconset/icon_16x16.png >/dev/null 2>&1
    sips -z 32 32     icon.png --out icon.iconset/icon_16x16@2x.png >/dev/null 2>&1
    sips -z 32 32     icon.png --out icon.iconset/icon_32x32.png >/dev/null 2>&1
    sips -z 64 64     icon.png --out icon.iconset/icon_32x32@2x.png >/dev/null 2>&1
    sips -z 128 128   icon.png --out icon.iconset/icon_128x128.png >/dev/null 2>&1
    sips -z 256 256   icon.png --out icon.iconset/icon_128x128@2x.png >/dev/null 2>&1
    sips -z 256 256   icon.png --out icon.iconset/icon_256x256.png >/dev/null 2>&1
    sips -z 512 512   icon.png --out icon.iconset/icon_256x256@2x.png >/dev/null 2>&1
    sips -z 512 512   icon.png --out icon.iconset/icon_512x512.png >/dev/null 2>&1
    sips -z 1024 1024 icon.png --out icon.iconset/icon_512x512@2x.png >/dev/null 2>&1

    iconutil -c icns icon.iconset -o "$RESOURCES_DIR/AppIcon.icns" 2>/dev/null
fi

# Clean up
rm -rf icon.iconset icon.png icon.svg

# Create Info.plist
cat > "$CONTENTS_DIR/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>Klippy</string>
    <key>CFBundleIdentifier</key>
    <string>com.klippy.app</string>
    <key>CFBundleName</key>
    <string>Klippy</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>LSUIElement</key>
    <true/>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
</dict>
</plist>
EOF

# Remove old version if exists
if [ -d "/Applications/$APP_DIR" ]; then
    echo "🗑️  Removing old version..."
    rm -rf "/Applications/$APP_DIR"
fi

# Copy to Applications
echo "📂 Installing to /Applications..."
cp -r "$APP_DIR" /Applications/

echo ""
echo "✅ Installation complete!"
echo ""
echo "📋 Klippy has been installed to /Applications/Klippy.app"
echo ""
echo "🔐 IMPORTANT: Grant Accessibility permissions"
echo "   1. Open System Settings"
echo "   2. Go to Privacy & Security → Accessibility"
echo "   3. Click the + button and add Klippy"
echo "   4. Enable the checkbox next to Klippy"
echo ""
echo "🚀 To start Klippy:"
echo "   - Open Klippy from Applications folder"
echo "   - Or run: open /Applications/Klippy.app"
echo ""
echo "⌨️  Keyboard shortcuts:"
echo "   Cmd+0-9 = Select clipboard items"
echo "   Cmd+V = Paste selected item"
echo "   📎 icon = Click to see all 25 items"
echo ""
