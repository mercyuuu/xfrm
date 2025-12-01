#!/bin/bash
set -e

echo "🚀 Installing xfrm..."
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi
echo "📦 Building xfrm in release mode..."
cargo build --release

if [ ! -f "target/release/xfrm" ]; then
    echo "❌ Build failed"
    exit 1
fi

if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

echo "📁 Installing to $INSTALL_DIR..."
cp target/release/xfrm "$INSTALL_DIR/xfrm"
chmod +x "$INSTALL_DIR/xfrm"

echo -e "xfrm installed successfully \n"
echo "binary location: $INSTALL_DIR/xfrm"
echo -e "binary size: $(du -h $INSTALL_DIR/xfrm | cut -f1) \n"

# Check dependencies
echo "🔍 Checking dependencies..."
MISSING_DEPS=()

if ! command -v ffmpeg &> /dev/null; then
    MISSING_DEPS+=("ffmpeg")
fi

if ! command -v magick &> /dev/null && ! command -v convert &> /dev/null; then
    MISSING_DEPS+=("imagemagick")
fi

if ! command -v pandoc &> /dev/null; then
    MISSING_DEPS+=("pandoc")
fi

if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
    echo "⚠️  Missing dependencies: ${MISSING_DEPS[*]}"
    echo ""
    echo "Install them with:"
    
    if [ -f /etc/debian_version ]; then
        echo "  sudo apt update && sudo apt install ${MISSING_DEPS[*]}"
    elif [ -f /etc/arch-release ]; then
        echo "  sudo pacman -S ${MISSING_DEPS[*]}"
    elif [ "$(uname)" == "Darwin" ]; then
        echo "  brew install ${MISSING_DEPS[*]}"
    else
        echo "  Please install: ${MISSING_DEPS[*]}"
    fi
else
    echo "✅ All dependencies are installed!"
fi

echo ""
echo "Try it out:"
echo "  xfrm --help"
echo "  xfrm input.mp4 output.mp3"
