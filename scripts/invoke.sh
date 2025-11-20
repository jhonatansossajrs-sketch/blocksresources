#!/bin/bash

# Configuración de la red
NETWORK="testnet"
SOROBAN_RPC_URL="https://soroban-testnet.stellar.org:443"

# Direcciones proporcionadas
ADMIN_ADDRESS="GBMY6UIHGFIPM2ZWXBB5U7AJTSASWL7BB4PVYVNAZZQYEUQU3UYJOWYH"
SIGNER1_ADDRESS="GBMY6UIHGFIPM2ZWXBB5U7AJTSASWL7BB4PVYVNAZZQYEUQU3UYJOWYH"
SIGNER2_ADDRESS="GDIK733AD4V2CDQMMZGXLFGNM7W3234IORXIGVT2BCHSUDAWI42BRD3J"
RECIPIENT_ADDRESS="GDIK733AD4V2CDQMMZGXLFGNM7W3234IORXIGVT2BCHSUDAWI42BRD3J"

# Obtener el ID del contrato
CONTRACT_ID=$(cat contract_id.txt)

# Menú de opciones
echo "Block's Resources - Environmental Resource Tracking"
echo "1. Register a new environmental resource"
echo "2. Create a transfer proposal"
echo "3. Sign a proposal"
echo "4. Get resource information"
echo "5. Get proposal information"
echo "6. Get list of authorized signers"
echo "7. Add a new signer (admin only)"
echo "Choose an option:"

read OPTION

case $OPTION in
  1)
    echo "Enter resource name:"
    read NAME
    echo "Enter resource origin:"
    read ORIGIN
    echo "Enter resource certification:"
    read CERTIFICATION
    
    echo "Registering resource..."
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source $ADMIN_ADDRESS \
      --network $NETWORK \
      --rpc-url $SOROBAN_RPC_URL \
      -- \
      register_resource \
      --name "$NAME" \
      --origin "$ORIGIN" \
      --certification "$CERTIFICATION"
    ;;
    
  2)
    echo "Enter resource ID:"
    read RESOURCE_ID
    echo "Enter proposer address (1 for $SIGNER1_ADDRESS, 2 for $SIGNER2_ADDRESS):"
    read PROPOSER_CHOICE
    
    if [ $PROPOSER_CHOICE -eq 1 ]; then
      PROPOSER=$SIGNER1_ADDRESS
    else
      PROPOSER=$SIGNER2_ADDRESS
    fi
    
    echo "Enter recipient address (default: $RECIPIENT_ADDRESS):"
    read RECIPIENT
    if [ -z "$RECIPIENT" ]; then
      RECIPIENT=$RECIPIENT_ADDRESS
    fi
    
    echo "Enter transfer description:"
    read DESCRIPTION
    
    echo "Creating transfer proposal..."
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source $PROPOSER \
      --network $NETWORK \
      --rpc-url $SOROBAN_RPC_URL \
      -- \
      create_transfer_proposal \
      --proposer $PROPOSER \
      --resource-id $RESOURCE_ID \
      --new-owner $RECIPIENT \
      --description "$DESCRIPTION"
    ;;
    
  3)
    echo "Enter proposal ID:"
    read PROPOSAL_ID
    echo "Enter signer address (1 for $SIGNER1_ADDRESS, 2 for $SIGNER2_ADDRESS):"
    read SIGNER_CHOICE
    
    if [ $SIGNER_CHOICE -eq 1 ]; then
      SIGNER=$SIGNER1_ADDRESS
    else
      SIGNER=$SIGNER2_ADDRESS
    fi
    
    echo "Signing proposal..."
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source $SIGNER \
      --network $NETWORK \
      --rpc-url $SOROBAN_RPC_URL \
      -- \
      sign_proposal \
      --signer $SIGNER \
      --proposal-id $PROPOSAL_ID
    ;;
    
  4)
    echo "Enter resource ID:"
    read RESOURCE_ID
    
    echo "Getting resource information..."
    soroban contract invoke \
      --id $CONTRACT_ID \
      --network $NETWORK \
      --rpc-url $SOROBAN_RPC_URL \
      -- \
      get_resource \
      --resource-id $RESOURCE_ID
    ;;
    
  5)
    echo "Enter proposal ID:"
    read PROPOSAL_ID
    
    echo "Getting proposal information..."
    soroban contract invoke \
      --id $CONTRACT_ID \
      --network $NETWORK \
      --rpc-url $SOROBAN_RPC_URL \
      -- \
      get_proposal \
      --proposal-id $PROPOSAL_ID
    ;;
    
  6)
    echo "Getting list of authorized signers..."
    soroban contract invoke \
      --id $CONTRACT_ID \
      --network $NETWORK \
      --rpc-url $SOROBAN_RPC_URL \
      -- \
      get_signers
    ;;
    
  7)
    echo "Enter new signer address:"
    read NEW_SIGNER
    
    echo "Adding new signer..."
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source $ADMIN_ADDRESS \
      --network $NETWORK \
      --rpc-url $SOROBAN_RPC_URL \
      -- \
      add_signer \
      --admin $ADMIN_ADDRESS \
      --new-signer $NEW_SIGNER
    ;;
    
  *)
    echo "Invalid option"
    ;;
esac
