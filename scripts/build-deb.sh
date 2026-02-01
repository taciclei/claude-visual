#!/bin/bash
set -e

# Build Debian package for Claude Visual
# Requires: debhelper, cargo

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
PACKAGE_NAME="claude-visual"
ARCH="amd64"

echo "==> Building ${PACKAGE_NAME} ${VERSION} for ${ARCH}..."

# Create build directory
BUILD_DIR="build/deb"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/${PACKAGE_NAME}_${VERSION}_${ARCH}"
cd "$BUILD_DIR/${PACKAGE_NAME}_${VERSION}_${ARCH}"

echo "==> Building release binary..."
cd "$PROJECT_ROOT"
cargo build --release

cd "$BUILD_DIR/${PACKAGE_NAME}_${VERSION}_${ARCH}"

echo "==> Creating package structure..."
mkdir -p DEBIAN
mkdir -p usr/bin
mkdir -p usr/share/applications
mkdir -p usr/share/icons/hicolor/256x256/apps
mkdir -p usr/share/doc/${PACKAGE_NAME}

echo "==> Copying files..."
cp "$PROJECT_ROOT/target/release/claude-visual" usr/bin/
cp "$PROJECT_ROOT/packaging/appimage/claude-visual.desktop" usr/share/applications/
[ -f "$PROJECT_ROOT/assets/icon.png" ] && cp "$PROJECT_ROOT/assets/icon.png" usr/share/icons/hicolor/256x256/apps/claude-visual.png
cp "$PROJECT_ROOT/packaging/debian/copyright" usr/share/doc/${PACKAGE_NAME}/

# Create control file
cat > DEBIAN/control << EOF
Package: ${PACKAGE_NAME}
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: ${ARCH}
Depends: libc6 (>= 2.31), libgtk-3-0, libvulkan1, libxkbcommon0
Recommends: git
Maintainer: Claude Visual Team <claude-visual@example.com>
Description: Visual client for Claude Code
 Claude Visual is a GPU-accelerated visual interface for Claude Code CLI,
 inspired by Warp terminal. Built with GPUI for fast, native performance.
 .
 Features include:
  * Native GPU-accelerated UI
  * Syntax highlighting for multiple languages
  * Git integration with worktree support
  * Multi-tab conversations
  * Vim mode support
  * MCP protocol support
  * Multi-model AI support
EOF

# Set permissions
chmod 755 usr/bin/claude-visual
chmod 644 DEBIAN/control
chmod 644 usr/share/applications/claude-visual.desktop
[ -f usr/share/icons/hicolor/256x256/apps/claude-visual.png ] && chmod 644 usr/share/icons/hicolor/256x256/apps/claude-visual.png

echo "==> Building package..."
cd "$PROJECT_ROOT/$BUILD_DIR"
dpkg-deb --build "${PACKAGE_NAME}_${VERSION}_${ARCH}"

echo "==> Moving package to project root..."
mv "${PACKAGE_NAME}_${VERSION}_${ARCH}.deb" "$PROJECT_ROOT/"

echo "==> Cleaning up..."
cd "$PROJECT_ROOT"
rm -rf "$BUILD_DIR"

echo "==> Done!"
ls -la "${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
