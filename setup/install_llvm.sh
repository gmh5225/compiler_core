#!/bin/bash

install_llvm_mac() {
    echo "Installing LLVM 17 on macOS..."
    brew install llvm@17
    echo "Exporting LLVM 17 to PATH..."
    echo 'export PATH="/usr/local/opt/llvm@17/bin:$PATH"' >> ~/.bash_profile
    source ~/.bash_profile
}

install_llvm_linux() {
    echo "Installing LLVM 17 on Linux..."
    sudo apt-get update
    sudo apt-get install -y software-properties-common
    wget https://apt.llvm.org/llvm.sh
    chmod +x llvm.sh
    sudo ./llvm.sh 17
}

OS="$(uname)"

case "$OS" in
    "Darwin")
        install_llvm_mac
        ;;
    "Linux")
        install_llvm_linux
        ;;
    *)
        echo "Unsupported operating system."
        ;;
esac
