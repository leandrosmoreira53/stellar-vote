# 🗳️ Soroban Voting Smart Contract (Stellar)

A multiple-choice voting system implemented in Rust for the Stellar network using Soroban.

## 🚀 Features

- ✅ **Multiple-choice voting**
- ✅ **Admin control** for managing elections
- ✅ **Unique vote verification** per address
- ✅ **Real-time results**
- ✅ **Simple and intuitive interface**

## 📋 Prerequisites

- Rust (version 1.70+)
- Cargo
- Soroban CLI
- Stellar Testnet account

## 🔧 Installation

### 1. Install Soroban CLI
```bash
cargo install --locked soroban-cli
```

### 2. Verify installation
```bash
soroban --version
```

## 🚀 Automated Deployment

### Option 1: Automated Script (Recommended)
```bash
# Make executable
chmod +x deploy.sh

# Execute deployment
./deploy.sh
```

### Option 2: Manual Deployment

#### Step 1: Compile
```bash
cargo build --target wasm32-unknown-unknown --release
```

#### Step 2: Optimize WASM
```bash
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/voting.wasm
```

#### Step 3: Configure Identity
```bash
soroban keys generate alice
```

#### Step 4: Configure Testnet Network
```bash
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

#### Step 5: Deploy
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/voting.wasm \
  --source alice \
  --network testnet
```

#### Step 6: Initialize
```bash
# Replace [CONTRACT_ID] with the ID returned from deployment
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source alice \
  --network testnet \
  -- initialize \
  --admin [ADMIN_ADDRESS]
```

## 📖 How to Use

### 1. Create Election
```bash
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source alice \
  --network testnet \
  -- create_election \
  --title "Presidential Election 2024" \
  --description "Presidential voting" \
  --end_time 1735689600
```

### 2. Add Candidates
```bash
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source alice \
  --network testnet \
  -- add_party \
  --name "Candidate A" \
  --description "Liberal Party"
```

### 3. Vote
```bash
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source [VOTER_ADDRESS] \
  --network testnet \
  -- vote \
  --election_id 0 \
  --party_id 0
```

### 4. View Results
```bash
soroban contract invoke \
  --id [CONTRACT_ID] \
  --source alice \
  --network testnet \
  -- get_results \
  --election_id 0
```

## 🔍 Contract Structure

### Main Functions
- `initialize(admin: Address)` - Initialize the contract
- `create_election(title: String, description: String, end_time: u64)` - Create new election
- `add_party(name: String, description: String)` - Add candidate/party
- `vote(election_id: u32, party_id: u32)` - Cast vote
- `get_results(election_id: u32)` - Get election results

### Data Structures
- `Election` - Election information
- `Party` - Candidate/party information
- `Vote` - Individual vote record

## 🌐 Available Networks

### Testnet
- **RPC URL:** https://soroban-testnet.stellar.org:443
- **Network Passphrase:** "Test SDF Network ; September 2015"
- **Explorer:** https://stellar.expert/explorer/testnet/

### Mainnet (Production)
- **RPC URL:** https://soroban-mainnet.stellar.org:443
- **Network Passphrase:** "Public Global Stellar Network ; September 2015"

## 💰 Funding (Testnet)

To obtain XLM on testnet:
- **Friendbot:** https://friendbot.stellar.org/
- **Stellar Laboratory:** https://laboratory.stellar.org/

## 🐛 Troubleshooting

### Error: "stellar contract optimize"
- **Problem:** Incorrect command
- **Solution:** Use `soroban contract optimize`

### Error: "No such file or directory"
- **Problem:** Incorrect WASM path
- **Solution:** Verify the file is at `target/wasm32-unknown-unknown/release/voting.wasm`

### Error: "Soroban CLI not found"
- **Problem:** CLI not installed
- **Solution:** Run `cargo install --locked soroban-cli`

## 📚 Additional Resources

- [Soroban Documentation](https://soroban.stellar.org/)
- [Stellar Developer Portal](https://developers.stellar.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Soroban Examples](https://github.com/stellar/soroban-examples)

## 🤝 Contributing

1. Fork the project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

## 🆘 Support

If you encounter problems:
1. Check error logs
2. Consult the documentation
3. Open an issue on GitHub
4. Contact the development team

---

**Developed with ❤️ for the Stellar community**