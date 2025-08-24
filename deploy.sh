#!/bin/bash

# Script de Deploy Automatizado para Smart Contract de Votação Soroban
# Autor: Sistema de Votação Stellar

set -e  # Para o script se houver erro

echo "🚀 Iniciando deploy do Smart Contract de Votação Soroban..."

# Verificar se o Soroban CLI está instalado
if ! command -v soroban &> /dev/null; then
    echo "❌ Soroban CLI não encontrado. Instalando..."
    cargo install --locked soroban-cli
fi

echo "✅ Soroban CLI encontrado: $(soroban --version)"

# Verificar se o arquivo WASM existe
WASM_PATH="target/wasm32-unknown-unknown/release/voting.wasm"
if [ ! -f "$WASM_PATH" ]; then
    echo "❌ Arquivo WASM não encontrado em $WASM_PATH"
    echo "🔨 Compilando o contrato..."
    cargo build --target wasm32-unknown-unknown --release
fi

echo "✅ Arquivo WASM encontrado: $WASM_PATH"

# Otimizar o WASM para Soroban
echo "🔧 Otimizando WASM para Soroban..."
soroban contract optimize --wasm "$WASM_PATH"

# Verificar se a identidade 'alice' existe, se não, criar
if ! soroban keys list | grep -q "alice"; then
    echo "🔑 Criando identidade 'alice'..."
    soroban keys generate alice
fi

# Verificar se a rede testnet está configurada
if ! soroban network list | grep -q "testnet"; then
    echo "🌐 Configurando rede testnet..."
    soroban network add testnet \
        --rpc-url https://soroban-testnet.stellar.org:443 \
        --network-passphrase "Test SDF Network ; September 2015"
fi

echo "✅ Rede testnet configurada"

# Deploy do contrato
echo "🚀 Fazendo deploy do contrato..."
CONTRACT_ID=$(soroban contract deploy \
    --wasm "$WASM_PATH" \
    --source alice \
    --network testnet)

echo "✅ Contrato deployado com ID: $CONTRACT_ID"

# Salvar o ID do contrato em um arquivo
echo "$CONTRACT_ID" > contract_id.txt
echo "📝 ID do contrato salvo em contract_id.txt"

# Obter endereço da identidade alice
ADMIN_ADDRESS=$(soroban keys show alice --public-key)
echo "👤 Endereço admin: $ADMIN_ADDRESS"

# Inicializar o contrato
echo "🔧 Inicializando o contrato..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source alice \
    --network testnet \
    -- initialize \
    --admin "$ADMIN_ADDRESS"

echo "✅ Contrato inicializado com sucesso!"
echo ""
echo "🎉 DEPLOY COMPLETO!"
echo "📋 Resumo:"
echo "   - ID do Contrato: $CONTRACT_ID"
echo "   - Admin: $ADMIN_ADDRESS"
echo "   - Rede: Testnet"
echo "   - Arquivo WASM: $WASM_PATH"
echo ""
echo "🔗 Links úteis:"
echo "   - Explorer: https://stellar.expert/explorer/testnet/"
echo "   - RPC: https://soroban-testnet.stellar.org:443"
echo ""
echo "💡 Para interagir com o contrato:"
echo "   soroban contract invoke --id $CONTRACT_ID --source alice --network testnet -- [FUNÇÃO]"
