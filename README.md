# Stellar Soroban Voting Smart Contract

Um contrato inteligente de votação simples de múltipla escolha construído em Rust para a plataforma Stellar Soroban.

## Características

- **Votação de Múltipla Escolha**: Suporte para múltiplos partidos/opções de voto
- **Registro de Votantes**: Apenas votantes registrados podem votar
- **Prevenção de Voto Duplo**: Cada votante pode votar apenas uma vez
- **Delegação de Votos**: Votantes podem delegar seus votos para outros
- **Controle de Acesso**: Apenas o administrador pode registrar partidos e votantes
- **Transparência**: Contagem de votos em tempo real e estatísticas
- **Limite de Tempo**: Suporte para definir prazo limite para votação
- **Proteção Avançada**: Prevenção de delegação circular e cadeia longa demais

## Estrutura do Projeto

```
.
├── Cargo.toml              # Configuração do workspace
├── README.md               # Este arquivo
└── contracts/
    └── voting/
        ├── Cargo.toml      # Configuração do contrato
        └── src/
            ├── lib.rs      # Implementação principal do contrato
            └── test.rs     # Suite de testes
```

## Pré-requisitos

- [Rust](https://rustup.rs/) (versão mais recente)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)

## Instalação

1. Clone o repositório:
```bash
git clone <seu-repo>
cd vote
```

2. Instale as dependências:
```bash
cargo check
```

## Compilação

Para compilar o contrato:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Para compilar com otimizações para Soroban:

```bash
stellar contract build
```

## Testes

Execute todos os testes:

```bash
cargo test
```

Execute testes específicos:

```bash
cargo test test_voting
cargo test test_delegation
```

## Uso do Contrato

### Funções Principais

#### `initialize(admin: Address)`
Inicializa o contrato com um endereço de administrador.

```rust
client.initialize(&admin_address);
```

#### `add_party(party_name: Symbol)`
Adiciona um novo partido/opção de voto (apenas admin).

```rust
client.add_party(&symbol_short!("PartyA"));
```

#### `add_voter(voter: Address)`
Registra um novo votante (apenas admin).

```rust
client.add_voter(&voter_address);
```

#### `vote(voter: Address, party_name: Symbol)`
Permite que um votante registrado vote em um partido.

```rust
client.vote(&voter_address, &symbol_short!("PartyA"));
```

#### `delegate(delegator: Address, delegate_to: Address)`
Permite que um votante delegue seu voto para outro votante.

```rust
client.delegate(&delegator_address, &delegate_address);
```

#### `set_voting_deadline(deadline: u64)`
Define um prazo limite para votação (apenas admin).

```rust
let deadline = env.ledger().timestamp() + 86400; // 24 horas
client.set_voting_deadline(&deadline);
```

### Funções de Consulta

#### `get_vote_count(party_name: Symbol) -> u32`
Retorna o número de votos para um partido específico.

#### `get_parties() -> Vec<Symbol>`
Retorna a lista de todos os partidos registrados.

#### `get_voter_status(voter: Address) -> VoterStatus`
Retorna o status de um votante específico.

#### `get_voting_stats() -> VotingStats`
Retorna estatísticas gerais da votação.

#### `get_all_results() -> Map<Symbol, u32>`
Retorna os resultados completos da votação.

#### `get_voting_deadline() -> Option<u64>`
Retorna o prazo limite para votação, se definido.

## Estados do Votante

- `NotRegistered`: Votante não registrado
- `Registered`: Votante registrado, pode votar
- `Voted`: Votante já votou
- `Delegated(Address)`: Votante delegou seu voto

## Regras de Votação

1. **Registro**: Apenas o administrador pode registrar partidos e votantes
2. **Voto Único**: Cada votante pode votar apenas uma vez
3. **Partidos Válidos**: Só é possível votar em partidos registrados
4. **Delegação**: 
   - Votantes podem delegar apenas se ainda não votaram
   - Não é possível delegar para si mesmo
   - Não é possível delegar para alguém que já votou ou delegou
5. **Poder de Voto**: Votantes acumulam votos delegados ao votar
6. **Prazo Limite**: Votação pode ter deadline definido pelo administrador
7. **Proteção Circular**: Sistema previne delegação circular e cadeias muito longas

## Exemplo de Uso Completo

```rust
// Inicializar contrato
client.initialize(&admin);

// Adicionar partidos
client.add_party(&symbol_short!("PartyA"));
client.add_party(&symbol_short!("PartyB"));
client.add_party(&symbol_short!("PartyC"));

// Registrar votantes
client.add_voter(&voter1);
client.add_voter(&voter2);
client.add_voter(&voter3);
client.add_voter(&voter4);

// Delegar votos
client.delegate(&voter1, &voter2); // voter1 delega para voter2

// Votar
client.vote(&voter2, &symbol_short!("PartyA")); // voter2 vota (2 votos: próprio + delegado)
client.vote(&voter3, &symbol_short!("PartyB")); // voter3 vota (1 voto)
client.vote(&voter4, &symbol_short!("PartyA")); // voter4 vota (1 voto)

// Verificar resultados
let results = client.get_all_results();
// PartyA: 3 votos, PartyB: 1 voto, PartyC: 0 votos
```

## Deploy

Para fazer deploy na testnet Stellar:

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/voting.wasm \
  --source alice \
  --network testnet
```

## Considerações de Segurança

- O contrato requer autenticação para todas as operações sensíveis
- Prevenção de overflow em contadores de votos
- Validação rigorosa de estados de votantes
- Proteção contra delegação circular (básica)

## Limitações

- Tamanho máximo do contrato: 64KB (conforme limites do Soroban)
- Máximo de 100 delegações em cadeia para evitar loops infinitos

## Licença

Este projeto é fornecido como exemplo educacional.