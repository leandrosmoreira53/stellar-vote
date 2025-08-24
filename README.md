# 🗳️ Smart Contract de Votação Soroban (Stellar)

Um sistema de votação de múltipla escolha implementado em Rust para a rede Stellar usando Soroban.

## 🚀 Funcionalidades

- ✅ **Votação de múltipla escolha**
- ✅ **Controle de admin** para gerenciar eleições
- ✅ **Verificação de votos únicos** por endereço
- ✅ **Resultados em tempo real**
- ✅ **Interface simples e intuitiva**

## 📋 Pré-requisitos

- Rust (versão 1.70+)
- Cargo
- Soroban CLI
- Conta na rede Stellar Testnet

## 🔧 Instalação

### 1. Instalar Soroban CLI
```bash
cargo install --locked soroban-cli
```

### 2. Verificar instalação
```bash
soroban --version
```

## 🚀 Deploy Automatizado

### Opção 1: Script Automatizado (Recomendado)
```bash
# Tornar executável
chmod +x deploy.sh

# Executar deploy
./deploy.sh
```

### Opção 2: Deploy Manual

#### Passo 1: Compilar
```bash
cargo build --target wasm32-unknown-unknown --release
```

#### Passo 2: Otimizar WASM
```bash
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/voting.wasm
```

#### Passo 3: Configurar Identidade
```bash
soroban keys generate alice
```

#### Passo 4: Configurar Rede Testnet
```bash
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

#### Passo 5: Deploy
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/voting.wasm \
  --source alice \
  --network testnet
```

#### Passo 6: Inicializar
```bash
# Substitua [CONTRACT_ID] pelo ID retornado no deploy
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source alice \
  --network testnet \
  -- initialize \
  --admin [ADMIN_ADDRESS]
```

## 📖 Como Usar

### 1. Criar Eleição
```bash
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source alice \
  --network testnet \
  -- create_election \
  --title "Eleição Presidencial 2024" \
  --description "Votação para presidente" \
  --end_time 1735689600
```

### 2. Adicionar Candidatos
```bash
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source alice \
  --network testnet \
  -- add_party \
  --name "Candidato A" \
  --description "Partido Liberal"
```

### 3. Votar
```bash
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source [VOTER_ADDRESS] \
  --network testnet \
  -- vote \
  --election_id 0 \
  --party_id 0
```

### 4. Ver Resultados
```bash
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source alice \
  --network testnet \
  -- get_results \
  --election_id 0
```

## 🔍 Estrutura do Contrato

### Funções Principais
- `initialize(admin: Address)` - Inicializa o contrato
- `create_election(title: String, description: String, end_time: u64)` - Cria nova eleição
- `add_party(name: String, description: String)` - Adiciona candidato/partido
- `vote(election_id: u32, party_id: u32)` - Registra voto
- `get_results(election_id: u32)` - Obtém resultados da eleição

### Estruturas de Dados
- `Election` - Informações da eleição
- `Party` - Informações do candidato/partido
- `Vote` - Registro de voto individual

## 🌐 Redes Disponíveis

### Testnet
- **RPC URL:** https://soroban-testnet.stellar.org:443
- **Network Passphrase:** "Test SDF Network ; September 2015"
- **Explorer:** https://stellar.expert/explorer/testnet/

### Mainnet (Produção)
- **RPC URL:** https://soroban-mainnet.stellar.org:443
- **Network Passphrase:** "Public Global Stellar Network ; September 2015"

## 💰 Funding (Testnet)

Para obter XLM na testnet:
- **Friendbot:** https://friendbot.stellar.org/
- **Stellar Laboratory:** https://laboratory.stellar.org/

## 🐛 Solução de Problemas

### Erro: "stellar contract optimize"
- **Problema:** Comando incorreto
- **Solução:** Use `soroban contract optimize`

### Erro: "No such file or directory"
- **Problema:** Caminho incorreto do WASM
- **Solução:** Verifique se o arquivo está em `target/wasm32-unknown-unknown/release/voting.wasm`

### Erro: "Soroban CLI não encontrado"
- **Problema:** CLI não instalado
- **Solução:** Execute `cargo install --locked soroban-cli`

## 📚 Recursos Adicionais

- [Documentação Soroban](https://soroban.stellar.org/)
- [Stellar Developer Portal](https://developers.stellar.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Soroban Examples](https://github.com/stellar/soroban-examples)

## 🤝 Contribuição

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo `LICENSE` para mais detalhes.

## 🆘 Suporte

Se encontrar problemas:
1. Verifique os logs de erro
2. Consulte a documentação
3. Abra uma issue no GitHub
4. Entre em contato com a equipe de desenvolvimento

---

**Desenvolvido com ❤️ para a comunidade Stellar**