# Ayush's Web3 Portfolio

Hey 👋 there, Welcome 🙂

Ayush here, a software developer with over 3+ years of experience in building full stack products using React, Next.js, Node, and Express.


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