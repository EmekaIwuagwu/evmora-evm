use anchor_lang::prelude::*;

declare_id!("Quorlin11111111111111111111111111111111111");

#[program]
pub mod quorlin_token {
    use super::*;

    pub fn initialize_token(ctx: Context<InitializeToken>, decimals: u8) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.decimals = decimals;
        mint.mint_authority = ctx.accounts.authority.key();
        mint.supply = 0;
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let user = &mut ctx.accounts.user;

        mint.supply += amount;
        user.balance += amount;

        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;

        require!(from.balance >= amount, CustomError::InsufficientFunds);

        from.balance -= amount;
        to.balance += amount;

        Ok(())
    }
}

#[account]
pub struct Mint {
    pub decimals: u8,
    pub supply: u64,
    pub mint_authority: Pubkey,
}

#[account]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(init, payer = authority, space = 8 + 64)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
}

#[error_code]
pub enum CustomError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
