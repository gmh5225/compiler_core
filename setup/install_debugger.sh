#!/bin/bash

install_on_mac() {
    echo "Checking for Xcode Command Line Tools on macOS..."

    if xcode-select -p &>/dev/null; then
        echo "Xcode Command Line Tools are already installed."
    else
        echo "Xcode Command Line Tools not found. Installing..."
        xcode-select --install
    fi

    if lldb --version &>/dev/null; then
        echo "LLDB installed."
    else
        echo "LLDB not successfuly installed. Try installing Xcode from the App Store."
    fi
}

install_on_linux() {
    echo "Installing GDB on Linux..."
    sudo apt-get update
    sudo apt-get install gdb
}

OS="$(uname)"

case "$OS" in
    "Darwin")
        install_on_mac
        ;;
    "Linux")
        install_on_linux
        ;;
    *)
        echo "Unsupported operating system."
        ;;
esac
