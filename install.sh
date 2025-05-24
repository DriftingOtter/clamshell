#!/usr/bin/env bash

# === Clamshell Builder Script ===
# Author: DriftingOtter (Daksh Kaul)

set -e

# Handle Ctrl+C interrupt
trap 'echo -e "\nBuild cancelled by user. Exiting."; exit 1' SIGINT

echo "Welcome to the Clamshell Builder"
echo "--------------------------------"

# Prompt for TODO file path
read -rp "Enter the full path to your to-do file (e.g., /home/user/todo.org or ~/todo.org): " RAW_TODO_FILE_PATH
if [[ -z "$RAW_TODO_FILE_PATH" ]]; then
    echo "ERROR: TODO file path cannot be empty."
    exit 1
fi

# Expand ~ to full home path
TODO_FILE_PATH="${RAW_TODO_FILE_PATH/#\~/$HOME}"

if [[ ! -f "$TODO_FILE_PATH" ]]; then
    echo "WARNING: File does not exist. Proceeding anyway..."
fi

# Prompt for optional default editor
read -rp "Enter your preferred text editor (press Enter to use 'nano'): " DEFAULT_EDITOR
DEFAULT_EDITOR=${DEFAULT_EDITOR:-nano}

# Prompt for optional default viewer
read -rp "Enter your preferred file viewer (press Enter to use 'cat'): " DEFAULT_VIEWER
DEFAULT_VIEWER=${DEFAULT_VIEWER:-cat}

echo ""
echo "Starting build with:"
echo "TODO file: $TODO_FILE_PATH"
echo "Editor:    $DEFAULT_EDITOR"
echo "Viewer:    $DEFAULT_VIEWER"
echo ""

# Export environment variables
export TODO_FILE_PATH
export DEFAULT_EDITOR
export DEFAULT_VIEWER

# Build project
echo "Building clamshell..."
cargo build --release

# Post-build checks
BINARY_PATH="target/release/clamshell"
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "ERROR: Build failed. Binary not found at $BINARY_PATH"
    exit 1
fi

# Basic binary test
echo "Running binary to confirm functionality..."
if ! "$BINARY_PATH" --help > /dev/null 2>&1; then
    echo "ERROR: Binary failed to execute properly."
    exit 1
fi

echo "Build verified successfully."

# Prompt to move binary to ~/.local/bin
read -rp "Move binary to ~/.local/bin for easy access? (y/N): " MOVE_BIN
if [[ "${MOVE_BIN,,}" == "y" ]]; then
    mkdir -p ~/.local/bin
    cp "$BINARY_PATH" ~/.local/bin/
    echo "Binary moved to ~/.local/bin/clamshell"
    echo "Make sure ~/.local/bin is in your PATH:"
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
else
    echo "You can run the app from $BINARY_PATH"
fi

# Clean build artifacts
echo "Cleaning up temporary build files..."
cargo clean

echo ""
echo "Clamshell build completed successfully."

