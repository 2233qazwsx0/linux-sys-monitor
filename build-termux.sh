#!/bin/bash

set -e

echo "========================================"
echo "  Termux System Monitor Build Script"
echo "========================================"
echo ""

TARGET="${1:-aarch64-linux-android}"
BUILD_TYPE="${2:-release}"

echo "Target: $TARGET"
echo "Build type: $BUILD_TYPE"
echo ""

check_toolchain() {
    if ! command -v rustup &> /dev/null; then
        echo "Error: rustup not found. Please install Rust first:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    if ! rustup target list --installed | grep -q "$TARGET"; then
        echo "Installing target: $TARGET..."
        rustup target add "$TARGET"
    fi
}

build_local() {
    echo ""
    echo "Building for local Termux environment..."
    echo "(x86_64-linux-android or aarch64-linux-android)"
    echo ""
    
    if [ "$(uname -m)" = "aarch64" ] || [ "$(uname -m)" = "arm64" ]; then
        LOCAL_TARGET="aarch64-linux-android"
    else
        LOCAL_TARGET="x86_64-linux-android"
    fi
    
    check_toolchain
    
    echo "Building for $LOCAL_TARGET..."
    cargo build --features termux --target "$LOCAL_TARGET" --"$BUILD_TYPE"
    
    echo ""
    echo "Build complete!"
    echo "Binary location: target/$LOCAL_TARGET/$BUILD_TYPE/termux-monitor"
    echo ""
    echo "To run in Termux:"
    echo "  cp target/$LOCAL_TARGET/$BUILD_TYPE/termux-monitor \$PREFIX/bin/"
    echo "  termux-monitor"
}

build_cross_compile() {
    echo ""
    echo "Cross-compiling for Android ARM64..."
    echo ""
    
    check_toolchain
    
    CROSS_TARGET="$TARGET"
    
    echo "Building for $CROSS_TARGET..."
    cargo build --features termux --target "$CROSS_TARGET" --"$BUILD_TYPE"
    
    echo ""
    echo "Build complete!"
    echo "Binary location: target/$CROSS_TARGET/$BUILD_TYPE/termux-monitor"
    echo ""
    echo "To install on Android device:"
    echo "  adb push target/$CROSS_TARGET/$BUILD_TYPE/termux-monitor /data/local/tmp/"
    echo "  adb shell chmod 755 /data/local/tmp/termux-monitor"
    echo "  adb shell /data/local/tmp/termux-monitor"
}

show_help() {
    echo "Usage: $0 [TARGET] [BUILD_TYPE]"
    echo ""
    echo "Targets:"
    echo "  aarch64-linux-android  - Android ARM64 (default)"
    echo "  armv7-linux-androideabi - Android ARMv7"
    echo "  x86_64-linux-android  - Android x86_64"
    echo "  local                  - Build for local Termux environment"
    echo ""
    echo "Build types:"
    echo "  release  - Optimized release build (default)"
    echo "  debug    - Debug build"
    echo ""
    echo "Examples:"
    echo "  $0                     # Build for ARM64 release"
    echo "  $0 aarch64-linux-android debug  # Debug build for ARM64"
    echo "  $0 local               # Build for local Termux"
}

case "$TARGET" in
    -h|--help)
        show_help
        exit 0
        ;;
    local)
        build_local
        ;;
    *)
        build_cross_compile
        ;;
esac

echo "========================================"
echo "  Build completed successfully!"
echo "========================================"
