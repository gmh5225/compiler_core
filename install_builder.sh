#!/bin/bash

# Build the project
cargo build --release
if [ $? -ne 0 ]; then
  echo "Build failed. Please check for any compilation errors."
  exit 1
fi

INSTALL_DIR="/usr/local/bin"

if [ ! -d "$INSTALL_DIR" ]; then
  sudo mkdir -p "$INSTALL_DIR"
  if [ $? -ne 0 ]; then
    echo "Failed to create the installation directory. Please check permissions."
    exit 1
  fi
fi

BINARY_PATH="$(pwd)/target/debug/compiler_core"

sudo ln -sf "$BINARY_PATH" "$INSTALL_DIR/builder"
if [ $? -ne 0 ]; then
  echo "Failed to create a symbolic link to $BINARY_PATH in $INSTALL_DIR. Please check permissions."
  exit 1
fi

echo "Installation complete. You can now use the 'builder' command."
