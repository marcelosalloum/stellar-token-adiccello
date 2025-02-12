# Soroban OpenZeppelin Token

This repository contains a Soroban SEP-41 fungible token built using OpenZeppelin libraries. The contract is designed to run on Stellar's smart contract platform, Soroban.

Full tutorial and video demo is available here: https://jamesbachini.com/stellar-soroban-with-openzeppelin/

### Prerequisites

Ensure you have the following installed before proceeding:

- Rust (latest stable version)
- Cargo (Rust package manager)
- Soroban SDK
- Stellar CLI

### Installation

Clone this repository:

```bash
git clone https://github.com/jamesbachini/Soroban-OpenZeppelin-Token.git
cd Soroban-OpenZeppelin-Token
```

Compile the smart contract into WebAssembly (WASM):

```bash
cargo build --target wasm32-unknown-unknown --release
```

Generate a Stellar key pair if you haven’t already and fund them by adding your address to the end of the friendbot url:

```bash
stellar keys

curl https://friendbot.stellar.org/?addr=
```

Deploy the contract to the Stellar Testnet:

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/mycoin.wasm \
  --source james \
  --network testnet \
  --owner YOUR_PUBLIC_KEY \
  --cap 1000000
```

Minting Tokens

After deploying, mint tokens to the owner’s address:

```bash
stellar contract invoke \
  --id YOUR_CONTRACT_ID \
  --source james \
  --network testnet \
  -- mint --account YOUR_PUBLIC_KEY \
  --amount 1000
```

The full contract code is located in src/contract.rs. 
Resources

Stellar Dev Portal: https://bit.ly/stellardevhub 

Stellar Dev Docs: https://bit.ly/stellardevdocs

Stellar Dev Discord: https://bit.ly/stellar-discord-JB


License

This project is licensed under the MIT License.