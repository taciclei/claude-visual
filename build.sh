#!/bin/bash
# Build script for Claude Visual
#
# Prerequisites:
# 1. Install Metal Toolchain: xcodebuild -downloadComponent MetalToolchain
# 2. Ensure SDK 15.4 is available in /Library/Developer/CommandLineTools/SDKs/

set -e

# Use SDK 15.4 from Command Line Tools to avoid beta SDK issues
export SDKROOT="/Library/Developer/CommandLineTools/SDKs/MacOSX15.4.sdk"
export MACOSX_DEPLOYMENT_TARGET="15.4"

# Bindgen flags for proper header parsing
export BINDGEN_EXTRA_CLANG_ARGS="-isysroot $SDKROOT -I$SDKROOT/usr/include -DTARGET_OS_OSX=1"
export CPATH="$SDKROOT/usr/include"
export C_INCLUDE_PATH="$SDKROOT/usr/include"

# Build
cargo build "$@"
