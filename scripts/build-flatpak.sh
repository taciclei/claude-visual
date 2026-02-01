#!/bin/bash
set -e

# Build Flatpak for Claude Visual
# Requires: flatpak, flatpak-builder

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

APP_ID="com.claude-visual.app"

echo "==> Checking Flatpak SDK..."
if ! flatpak info org.freedesktop.Sdk//23.08 &> /dev/null; then
    echo "Installing Flatpak SDK..."
    flatpak install -y flathub org.freedesktop.Platform//23.08 org.freedesktop.Sdk//23.08
    flatpak install -y flathub org.freedesktop.Sdk.Extension.rust-stable//23.08
fi

echo "==> Generating cargo sources..."
# Generate cargo-sources.json for offline build
if command -v flatpak-cargo-generator &> /dev/null; then
    flatpak-cargo-generator Cargo.lock -o packaging/flatpak/cargo-sources.json
elif command -v python3 &> /dev/null && [ -f flatpak-cargo-generator.py ]; then
    python3 flatpak-cargo-generator.py Cargo.lock -o packaging/flatpak/cargo-sources.json
else
    echo "Warning: flatpak-cargo-generator not found"
    echo "Install with: pip install flatpak-cargo-generator"
    echo "Creating empty cargo-sources.json..."
    echo "[]" > packaging/flatpak/cargo-sources.json
fi

echo "==> Building Flatpak..."
flatpak-builder --force-clean --user --install-deps-from=flathub \
    build-dir packaging/flatpak/${APP_ID}.yml

echo "==> Creating Flatpak bundle..."
flatpak build-bundle ~/.local/share/flatpak/repo \
    claude-visual.flatpak ${APP_ID} --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo

echo "==> Done!"
ls -la claude-visual.flatpak
