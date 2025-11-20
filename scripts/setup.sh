#!/bin/bash

# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Instalar Stellar CLI
curl -sSf https://raw.githubusercontent.com/stellar/stellar-sdk/master/install.sh | sh
export PATH="$HOME/.stellar/bin:$PATH"

# Instalar Soroban CLI
cargo install --locked soroban-cli

# Verificar instalación
rustc --version
cargo --version
stellar --version
soroban --version
