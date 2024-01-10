#!/bin/bash

install_on_linux() {
    echo "Installing perf on Linux..."
    sudo apt-get update
    sudo apt-get install -y linux-tools-$(uname -r) linux-tools-generic
    echo "Installed version:"
    perf --version
}

install_on_mac() {
    echo "Checking for Instruments on macOS..."
    if ! xcode-select -p &>/dev/null; then
        echo "Installing Xcode Command Line Tools..."
        xcode-select --install
    else
        echo "Xcode Command Line Tools already installed."
    fi
    echo "Instruments is part of Xcode and should be already installed."
    echo "Installed version:"
    xcodebuild -version
}

OS="$(uname -s)"

case "$OS" in
    Linux*)     
        install_on_linux
        ;;
    Darwin*)    
        install_on_mac
        ;;
    *)          
        echo "Unsupported operating system for this script."
        echo "For Windows, consider using Visual Studio's Performance Profiler."
        ;;
esac
