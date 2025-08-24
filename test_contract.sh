#!/bin/bash

# Script de Teste para Smart Contract de Votação
# Testa todas as funcionalidades principais

set -e

echo "🧪 Iniciando testes do Smart Contract de Votação..."

# Verificar se o ID do contrato existe
if [ ! -f "contract_id.txt" ]; then
    echo "❌ Arquivo contract_id.txt não encontrado. Execute o deploy primeiro."
    exit 1
fi

CONTRACT_ID=$(cat contract_id.txt)
echo "✅ ID do contrato: $CONTRACT_ID"

# Verificar se a identidade alice existe
if ! soroban keys list | grep -q "alice"; then
    echo "❌ Identidade 'alice' não encontrada. Execute o deploy primeiro."
    exit 1
fi

echo "✅ Identidade 'alice' encontrada"

# Função para executar comando e mostrar resultado
run_command() {
    local description="$1"
    local command="$2"
    
    echo "🔍 $description"
    echo "   Comando: $command"
    
    if eval "$command"; then
        echo "   ✅ Sucesso"
    else
        echo "   ❌ Falha"
        return 1
    fi
    echo ""
}

# Teste 1: Verificar se o contrato está inicializado
echo "📋 Teste 1: Verificar inicialização do contrato"
run_command "Verificando admin do contrato" \
    "soroban contract invoke --id $CONTRACT_ID --source alice --network testnet -- get_admin"

# Teste 2: Criar uma eleição
echo "📋 Teste 2: Criar eleição"
ELECTION_TITLE="Eleição Teste 2024"
ELECTION_DESC="Eleição para teste do contrato"
END_TIME=$(($(date +%s) + 86400)) # 24 horas a partir de agora

run_command "Criando eleição: $ELECTION_TITLE" \
    "soroban contract invoke --id $CONTRACT_ID --source alice --network testnet -- create_election --title '$ELECTION_TITLE' --description '$ELECTION_DESC' --end_time $END_TIME"

# Teste 3: Adicionar candidatos
echo "📋 Teste 3: Adicionar candidatos"
run_command "Adicionando candidato: João Silva" \
    "soroban contract invoke --id $CONTRACT_ID --source alice --network testnet -- add_party --name 'João Silva' --description 'Partido Liberal'"

run_command "Adicionando candidato: Maria Santos" \
    "soroban contract invoke --id $CONTRACT_ID --source alice --network testnet -- add_party --name 'Maria Santos' --description 'Partido Conservador'"

run_command "Adicionando candidato: Pedro Costa" \
    "soroban contract invoke --id $CONTRACT_ID --source alice --network testnet -- add_party --name 'Pedro Costa' --description 'Partido Verde'"

# Teste 4: Verificar candidatos
echo "📋 Teste 4: Verificar candidatos registrados"
run_command "Listando candidatos" \
    "soroban contract invoke --id $CONTRACT_ID --source alice --network testnet -- get_parties"

# Teste 5: Simular votos
echo "📋 Teste 5: Simular votos"
echo "   Nota: Este teste requer endereços de votantes válidos"
echo "   Para testar com votos reais, você precisará de endereços Stellar válidos"

# Teste 6: Verificar resultados
echo "📋 Teste 6: Verificar resultados da eleição"
run_command "Obtendo resultados da eleição 0" \
    "soroban contract invoke --id $CONTRACT_ID --source alice --network testnet -- get_results --election_id 0"

# Teste 7: Verificar informações da eleição
echo "📋 Teste 7: Verificar informações da eleição"
run_command "Obtendo informações da eleição 0" \
    "soroban contract invoke --id $CONTRACT_ID --source alice --network testnet -- get_election --election_id 0"

echo ""
echo "🎉 Testes básicos concluídos!"
echo ""
echo "📝 Resumo dos testes:"
echo "   ✅ Verificação de inicialização"
echo "   ✅ Criação de eleição"
echo "   ✅ Adição de candidatos"
echo "   ✅ Listagem de candidatos"
echo "   ✅ Verificação de resultados"
echo "   ✅ Informações da eleição"
echo ""
echo "💡 Para testar votos reais:"
echo "   1. Obtenha endereços Stellar válidos"
echo "   2. Use o comando: soroban contract invoke --id $CONTRACT_ID --source [VOTER_ADDRESS] --network testnet -- vote --election_id 0 --party_id [PARTY_ID]"
echo ""
echo "🔗 Verificar no Explorer:"
echo "   https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
