# Mintbase Puzzletask Session NFT Smart Contract

## Intro

This is the repository for the user session connected NFT smart contract that is also being used in our [demo repository](https://github.com/pztask/mintbase-pzt-session-nft-demo). This was forked from the [repository](https://github.com/pztask/mintbase-pzt-session-nft-contract#:~:text=forked%20from-,near%2Dexamples/nft%2Dtutorial,-Watch) used on the [Near NFT Zero to Hero](https://docs.near.org/tutorials/nfts/introduction) tutorial.

Not all of the functionalities of the original repo where kept. We opted for a trimmed version of the original smart contract because we only need a small set of the functionalities implemented there. Basically all the functionalites related to external contracts where removed.


## Building and deploying

1.
    ```
    export NFT_CONTRACT_ID=<your-smart-contract-id>
    ```
2.
    ```
    yarn build && near deploy --wasmFile out/main.wasm --accountId $NFT_CONTRACT_ID
    ```