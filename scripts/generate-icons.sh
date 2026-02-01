#!/bin/bash
# Generate icon files from SVG
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$PROJECT_ROOT/assets"

# Check for required tools
check_tool() {
    if ! command -v "$1" &> /dev/null; then
        echo "Warning: $1 not found, some icons may not be generated"
        return 1
    fi
    return 0
}

# Generate PNG from SVG using various methods
generate_png() {
    local size=$1
    local output=$2

    if check_tool rsvg-convert; then
        rsvg-convert -w "$size" -h "$size" "$ASSETS_DIR/icon.svg" -o "$output"
    elif check_tool convert; then
        convert -background none -resize "${size}x${size}" "$ASSETS_DIR/icon.svg" "$output"
    elif check_tool inkscape; then
        inkscape -w "$size" -h "$size" "$ASSETS_DIR/icon.svg" -o "$output"
    else
        echo "No SVG converter found (rsvg-convert, convert, or inkscape)"
        return 1
    fi
}

# Create temporary directory for icon generation
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "Generating icons..."

# Generate main PNG (256x256)
if generate_png 256 "$ASSETS_DIR/icon.png"; then
    echo "✓ Generated icon.png (256x256)"
else
    echo "✗ Failed to generate icon.png"
fi

# Generate various sizes for macOS iconset
if [[ "$OSTYPE" == "darwin"* ]]; then
    ICONSET_DIR="$TEMP_DIR/icon.iconset"
    mkdir -p "$ICONSET_DIR"

    sizes=(16 32 64 128 256 512 1024)
    for size in "${sizes[@]}"; do
        if generate_png "$size" "$ICONSET_DIR/icon_${size}x${size}.png"; then
            echo "✓ Generated icon_${size}x${size}.png"
        fi
        # Generate @2x versions
        if [ $size -le 512 ]; then
            size2x=$((size * 2))
            if generate_png "$size2x" "$ICONSET_DIR/icon_${size}x${size}@2x.png"; then
                echo "✓ Generated icon_${size}x${size}@2x.png"
            fi
        fi
    done

    # Create icns file
    if iconutil -c icns "$ICONSET_DIR" -o "$ASSETS_DIR/icon.icns"; then
        echo "✓ Generated icon.icns"
    else
        echo "✗ Failed to generate icon.icns"
    fi
fi

# Generate sizes for Linux
for size in 16 24 32 48 64 128 256 512; do
    mkdir -p "$ASSETS_DIR/icons/${size}x${size}"
    if generate_png "$size" "$ASSETS_DIR/icons/${size}x${size}/claude-visual.png"; then
        echo "✓ Generated ${size}x${size}/claude-visual.png"
    fi
done

echo ""
echo "Icon generation complete!"
echo "Generated files:"
ls -la "$ASSETS_DIR/icon."* 2>/dev/null || true
