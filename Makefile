.PHONY: build build-release run test check fmt lint clean bundle dmg install appimage deb flatpak linux-all

# Default target
all: build

# Development build
build:
	cargo build

# Release build with optimizations
build-release:
	cargo build --release

# Build with plugins feature
build-plugins:
	cargo build --features plugins

# Run development build
run:
	cargo run

# Run release build
run-release:
	cargo run --release

# Run tests
test:
	cargo test --all-features

# Check compilation without building
check:
	cargo check --all-features

# Format code
fmt:
	cargo fmt --all

# Run clippy linter
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
	cargo clean
	rm -rf "Claude Visual.app"
	rm -f claude-visual-*.dmg

# Create macOS app bundle
bundle: build-release
	@echo "Creating app bundle..."
	@mkdir -p "Claude Visual.app/Contents/MacOS"
	@mkdir -p "Claude Visual.app/Contents/Resources"
	@cp target/release/claude-visual "Claude Visual.app/Contents/MacOS/"
	@cp assets/Info.plist "Claude Visual.app/Contents/"
	@if [ -f assets/icon.icns ]; then cp assets/icon.icns "Claude Visual.app/Contents/Resources/"; fi
	@echo "App bundle created: Claude Visual.app"

# Create DMG installer
dmg: bundle
	@echo "Creating DMG..."
	@rm -f claude-visual.dmg
	@hdiutil create -volname "Claude Visual" -srcfolder "Claude Visual.app" -ov -format UDZO claude-visual.dmg
	@echo "DMG created: claude-visual.dmg"

# Install to /Applications (requires sudo)
install: bundle
	@echo "Installing to /Applications..."
	@rm -rf "/Applications/Claude Visual.app"
	@cp -R "Claude Visual.app" /Applications/
	@echo "Installed to /Applications/Claude Visual.app"

# Linux packaging targets
appimage: build-release
	@echo "Building AppImage..."
	@chmod +x scripts/build-appimage.sh
	@./scripts/build-appimage.sh

deb: build-release
	@echo "Building Debian package..."
	@chmod +x scripts/build-deb.sh
	@./scripts/build-deb.sh

flatpak:
	@echo "Building Flatpak..."
	@chmod +x scripts/build-flatpak.sh
	@./scripts/build-flatpak.sh

# Build all Linux packages
linux-all: appimage deb
	@echo "All Linux packages built!"

# Development workflow
dev: fmt lint check test
	@echo "All checks passed!"

# CI workflow
ci: fmt lint check test build-release
	@echo "CI checks passed!"

# Show help
help:
	@echo "Claude Visual - Build Commands"
	@echo ""
	@echo "Development:"
	@echo "  make build         - Debug build"
	@echo "  make run           - Run debug build"
	@echo "  make test          - Run tests"
	@echo "  make check         - Check compilation"
	@echo "  make fmt           - Format code"
	@echo "  make lint          - Run clippy"
	@echo "  make dev           - Run all dev checks"
	@echo ""
	@echo "macOS Release:"
	@echo "  make build-release - Optimized release build"
	@echo "  make run-release   - Run release build"
	@echo "  make bundle        - Create macOS .app bundle"
	@echo "  make dmg           - Create DMG installer"
	@echo "  make install       - Install to /Applications"
	@echo ""
	@echo "Linux Release:"
	@echo "  make appimage      - Create AppImage"
	@echo "  make deb           - Create Debian package"
	@echo "  make flatpak       - Create Flatpak bundle"
	@echo "  make linux-all     - Build all Linux packages"
	@echo ""
	@echo "Other:"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make ci            - Run CI checks"
	@echo "  make help          - Show this help"
