# Ayush's Web3 Portfolio

Hey 👋 there, Welcome 🙂

Ayush here, a software developer building full stack products using React, Next.js, Node, and Express.


Here are some of the web3 projects I build in my web3 journey and things I learned along the way : -


## Program (Solana)
<hr/>

<details close>
<summary>  Vault Program</summary>

- User can open a vault, which will be system account unique to user
- User can deposit amount into vault
- User can withdraw amount from vault

Tech Stack : Anchor, Rust
- [Repo Link](https://github.com/ayushagarwal27/anchor_vault_solana)

</details>



<hr/>

<details close>
<summary>  Escrow Program</summary>

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

</details>

<hr/>

<details close>
<summary> NFT Staking Program</summary>

#### Initialize User Instruction
- Creates user account PDA
- User account contains 
  - points: reward points
  - amounts_staked: number of nft staked

#### Initialize Config Instruction
- Creates Config PDA
- Config PDA includes
  - points_per_stake: reward points per stake
  - freeze_period: period till which nft needs to be staked
  - max_stake: max number of nft that can be staked
  - rewards_bump: bump of rewards_mint
- Initialize Rewards Mint
- Only Admin can create config and reward_mint

#### Stake Instruction
- Creates Stake PDA
- Stake PDA includes
  - owner: owner of nft
  - mint: mint address of nft
  - stake_at: Unix time stamp when nft was staked
- Delegate Authority of Mint ATA to Stake Account
- Freezes Nft
- Increment user account staked nft by one

#### UnStake Instruction
- Checks elapsed time
- Increases the user reward points
- Unfreezes NFT
- Revokes delegation to Stake Account
- Decreases ft staked number by one

#### Claim Instruction
- Mint reward tokens to User Rewards ATA
- Makes user reward points to zero


Tech Stack : Anchor, Rust
- [Repo Link](https://github.com/ayushagarwal27/anchor-nft-staking-program/tree/main)
</details>

## SolFlix  (Capstone Project)
> On-Chain Video Rental App

### Project Details

- **Website**: [Solflix](https://solflix-v.vercel.app/)  
- **Devnet Address**: [BFpFTmDwDNygSW9iL1UErkpDGTDiJFcjZh73DJ1vRu47](https://explorer.solana.com/address/BFpFTmDwDNygSW9iL1UErkpDGTDiJFcjZh73DJ1vRu47?cluster=devnet)
- **Program Repository**: [Program](https://github.com/ayushagarwal27/Solflix_Anchor_Rust)
- **Frontend Repository**: [Frontend](https://github.com/ayushagarwal27/solflix-frontend)
---

### Project Description
- SolFlix is a video rental dApp, where creator have autonomy to share videos anonymously, decide on validity of the video and price they would like to charge per video. (paid is SOL)


- Consumer have autonomy to have access to video in anonymous fashion and rent a video by paying direct to creator.


- This idea can be extended to share files and other media formats, without intervention of any intermediary.
