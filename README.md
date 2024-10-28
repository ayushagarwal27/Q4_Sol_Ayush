# Ayush's Web3 Portfolio

Hey 👋 there, Welcome 🙂

Ayush here, a software developer with over 3+ years of experience in building full stack products using React, Next.js, Node, and Express.


Here are some of the web3 projects I build in my web3 journey and things I learned along the way : -


## Smart Contract (Solana)

###  Voting dApp (devnet)

An Application where users can create new poll, add candidates, and vote by interacting via smart contract

 - Tech Stack : Anchor, Rust
 - [Repo Link](https://github.com/ayushagarwal27/solana_voting_dapp/tree/main/anchor/programs/voting)
 - [Explorer Link](https://explorer.solana.com/address/3DyHcuKbC9swQYJS6zDSRYTwDFoVaeEfkqgohS2GKg8M?cluster=devnet)

<hr/>

###  Vault Smart Contract

- User can open a vault, which will be system account unique to user
- User can deposit amount into vault
- User can withdraw amount from vault

Tech Stack : Anchor, Rust
- [Repo Link](https://github.com/ayushagarwal27/anchor_vault_solana)

<hr/>

###  Escrow Smart Contract

#### Make Instruction
- Maker initializes escrow PDA, 
- Maker creates vault PDA, whose authority lies with escrow
- Escrow contains information of token mint addresses and amount that needs to be exchanged

#### Refund Instruction
- Maker calls refund instruction for closing escrow and get a refund

#### Take Instruction
- Taker creates associated_token_account (ATA) for maker
- Taker transfers tokens to maker ATA
- Escrow transfers tokens from vault to taker ATA
- Escrow PDA is closed


Tech Stack : Anchor, Rust
- [Repo Link](https://github.com/ayushagarwal27/anchor_escrow_sol/tree/main)
