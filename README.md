# Mintbase Puzzletask Session-NFT Contract

## Introduction

This smart contract is meant to implement our Mintbase Puzzletask Session-NFT use case.

The idea behind it is that an NFT will be bound to a puzzletask user and that use can manage it's NFTs without the need to be the NFT owner. We can check our NFTs with just the puzzletask user session and we can also transfer the NFTs between different wallets, even if we loose access to the wallet where the NFTs are stored.

The smart contract is based on the zero to hero tutorial from Near, with some capabilities removed in favor of our custom puzzletask user layer and because we thought about this use case as a closed environment.

To enable this functionality we came up with the concept of _permit_, an API and a smart-contract stored record that associates a puzzletask user with a Near ecosystem wallet. With this we can authorize an action reaching the smart-contract that comes from a user authorized wallet. All NFT related actions require the existence of a valid permit (user and wallet).

There are also custom views to support our use case, such as listing a puzzletask user's NFTs and its permit. These do not require a permit as to be publicly auditable.

## Getting started

### Requirements

- [Rust](https://docs.near.org/develop/prerequisites)
- [A NEAR Wallet](https://wallet.testnet.near.org/create)
- [NEAR-CLI](https://docs.near.org/tools/near-cli#setup)

### Build

```bash
yarn build
```

### Deploy

1.  ```bash
    export NFT_CONTRACT_ID="YOUR_ACCOUNT_NAME"
    ```
2.  ```bash
    near deploy --wasmFile out/main.wasm --accountId $NFT_CONTRACT_ID
    ```

### Initializing the contract

```bash
near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID
```

### Viewing the contract's metadata

```bash
near view $NFT_CONTRACT_ID nft_metadata
```
