# Vault Program

A simple Solana program built with Anchor that allows users to create vaults and lock tokens for specified periods to earn rewards.

## Features

- **Create Vault**: Admins can create named vaults to manage token deposits
- **Lock Tokens**: Users can lock tokens for different periods and earn rewards based on lock duration
- **Tiered Rewards**: Reward rates vary based on lock period:
  - 1-7 days: 5% APY
  - 8-30 days: 10% APY  
  - 31-365 days: 20% APY

## How It Works

1. An admin creates a vault with a name
2. Users can lock tokens in the vault for a specified number of days (1-365)
3. The program calculates rewards based on the lock period
4. User deposits are tracked with lock/unlock times and reward rates

## Program Structure

- `Vault`: Stores vault information (admin, name, total deposited)
- `UserDeposit`: Tracks individual user deposits with lock periods and rewards
- `create_vault()`: Creates a new vault
- `lock_tokens()`: Allows users to deposit and lock tokens

## ⚠️ IMPORTANT CAUTION ⚠️

**THIS PROGRAM HAS NOT BEEN AUDITED**

This is a simple educational/demonstration program and should NOT be used in production or with real funds without proper security auditing. The code may contain:

- Security vulnerabilities
- Logic errors
- Economic exploits
- Missing safety checks

**DO NOT USE WITH REAL MONEY OR ON MAINNET WITHOUT A PROFESSIONAL AUDIT**

## Usage

This program is built using the Anchor framework for Solana. To build and deploy:

```bash
anchor build
anchor deploy
```

## Program ID

`5bNzrjEhjWuDCgduYRvxdMagHDktfV5XWkeUh6k17zYa`

---

*This is experimental code for learning purposes only.*
