use anchor_lang::prelude::*;

declare_id!("5bNzrjEhjWuDCgduYRvxdMagHDktfV5XWkeUh6k17zYa");

#[program]
pub mod vault {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>, name: String) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.admin = ctx.accounts.admin.key();
        vault.name = name;
        vault.total_deposited = 0;
        vault.bump = ctx.bumps.vault;

        msg!("Vault created {}", vault.name);

        Ok(())
    }

    pub fn lock_tokens(ctx: Context<LockTokens>, amount: u64, lock_days: u32) -> Result<()> {
        let user_deposit = &mut ctx.accounts.user_deposit;
        let vault = &mut ctx.accounts.vault;

        let clock = Clock::get()?;
        let unlock_time = clock.unix_timestamp + (lock_days as i64 * 24 * 60 * 60);

        let reward_rate = match lock_days {
            1..=7 => 5,
            8..=30 => 10,
            31..=365 => 20,
            _ => 0,
        };

        user_deposit.user = ctx.accounts.user.key();
        user_deposit.amount = amount;
        user_deposit.lock_time = clock.unix_timestamp;
        user_deposit.unlock_time = unlock_time;
        user_deposit.reward_rate = reward_rate;
        user_deposit.is_withdrawn = false;
        user_deposit.bump = ctx.bumps.user_deposit;

        vault.total_deposited += amount;

        msg!(
            "{} tokens locked for {} days at {}% APY",
            amount,
            lock_days,
            reward_rate
        );
        msg!("Unclock time: {}", unlock_time);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateVault<'info> {
    #[account(
        init,
        payer = admin,
        space = Vault::INIT_SPACE,
        seeds = [b"vault", admin.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LockTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"vault", vault.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = user,
        space = UserDeposit::INIT_SPACE,
        seeds = [b"deposit", vault.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_deposit: Account<'info, UserDeposit>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub admin: Pubkey,
    #[max_len(50)]
    pub name: String,
    pub total_deposited: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserDeposit {
    pub user: Pubkey,
    pub amount: u64,
    pub lock_time: i64,
    pub unlock_time: i64,
    pub reward_rate: u8,
    pub is_withdrawn: bool,
    pub bump: u8,
}
