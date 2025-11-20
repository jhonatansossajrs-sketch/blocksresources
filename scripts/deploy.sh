#!/bin/bash

# Configuración de la red
NETWORK="testnet"
SOROBAN_RPC_URL="https://soroban-testnet.stellar.org:443"

# Direcciones proporcionadas
ADMIN_ADDRESS="GBMY6UIHGFIPM2ZWXBB5U7AJTSASWL7BB4PVYVNAZZQYEUQU3UYJOWYH"
SIGNER1_ADDRESS="GBMY6UIHGFIPM2ZWXBB5U7AJTSASWL7BB4PVYVNAZZQYEUQU3UYJOWYH"
SIGNER2_ADDRESS="GDIK733AD4V2CDQMMZGXLFGNM7W3234IORXIGVT2BCHSUDAWI42BRD3J"

# Construir el contrato
echo "Building the contract..."
cargo build --target wasm32-unknown-unknown --release

# Optimizar el contrato
echo "Optimizing the contract..."
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/blocks_resources.wasm

# Desplegar el contrato
echo "Deploying the contract..."
CONTRACT_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/blocks_resources.wasm --source $ADMIN_ADDRESS --network $NETWORK --rpc-url $SOROBAN_RPC_URL)

echo "Contract deployed with ID: $CONTRACT_ID"

# Inicializar el contrato
echo "Initializing the contract..."
soroban contract invoke \
  --id $CONTRACT_ID \
  --source $ADMIN_ADDRESS \
  --network $NETWORK \
  --rpc-url $SOROBAN_RPC_URL \
  -- \
  initialize \
  --admin $ADMIN_ADDRESS \
  --signers "[$SIGNER1_ADDRESS, $SIGNER2_ADDRESS]"

echo "Contract initialized with admin and signers"

# Guardar el ID del contrato en un archivo para uso futuro
echo $CONTRACT_ID > contract_id.txt
echo "Contract ID saved to contract_id.txt"
