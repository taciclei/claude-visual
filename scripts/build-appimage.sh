#!/bin/bash
set -e

# Build AppImage for Claude Visual
# Requires: cargo, appimage-builder

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "==> Building Claude Visual (release)..."
cargo build --release

echo "==> Creating AppDir structure..."
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/applications
mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps

echo "==> Copying files..."
cp target/release/claude-visual AppDir/usr/bin/
cp packaging/appimage/claude-visual.desktop AppDir/usr/share/applications/
cp packaging/appimage/claude-visual.desktop AppDir/

# Copy icon if it exists
if [ -f assets/icon.png ]; then
    cp assets/icon.png AppDir/usr/share/icons/hicolor/256x256/apps/claude-visual.png
    cp assets/icon.png AppDir/claude-visual.png
else
    echo "Warning: assets/icon.png not found, creating placeholder..."
    # Create a simple placeholder icon
    convert -size 256x256 xc:#1a1a2e -fill white -gravity center \
        -pointsize 48 -annotate 0 "CV" AppDir/claude-visual.png 2>/dev/null || \
    echo "ImageMagick not found, skipping icon creation"
fi

# Create AppRun script
cat > AppDir/AppRun << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
exec "${HERE}/usr/bin/claude-visual" "$@"
EOF
chmod +x AppDir/AppRun

echo "==> Building AppImage..."
if command -v appimage-builder &> /dev/null; then
    appimage-builder --recipe packaging/appimage/AppImageBuilder.yml --skip-tests
elif command -v appimagetool &> /dev/null; then
    appimagetool AppDir Claude_Visual-x86_64.AppImage
else
    echo "Error: Neither appimage-builder nor appimagetool found"
    echo "Install with: pip install appimage-builder"
    echo "Or download appimagetool from https://github.com/AppImage/AppImageKit/releases"
    exit 1
fi

echo "==> Cleaning up..."
rm -rf AppDir

echo "==> Done! AppImage created."
ls -la *.AppImage 2>/dev/null || echo "AppImage file not found in current directory"
