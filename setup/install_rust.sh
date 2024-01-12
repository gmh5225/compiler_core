#!/bin/bash

install_rust() {
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
}

install_rust

if cargo --version &>/dev/null; then
    echo "Cargo is successfully installed."
else
    echo "There was an error installing Cargo."
fi
