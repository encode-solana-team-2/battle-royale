use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};

declare_id!("Cj6VD8qqwDj2fMk8G1EAFNV1v3w6AWJ5Sks6oRAkuspr");

#[program]
pub mod battle_royale {
    use super::*;

    pub fn initialize_token_vault(ctx: Context<InitializeTokenVault>) -> Result<()> {
        let token_vault = &mut ctx.accounts.token_vault;
        token_vault.bonk_price = 0.00002308;
        token_vault.wif_price = 2.58;
        Ok(())
    }

    pub fn transfer_spl_tokens(ctx: Context<TransferSpl>, amount: u64) -> Result<()> {
        let destination = &ctx.accounts.token_vault;
        let source = &ctx.accounts.from_ata;
        let token_program = &ctx.accounts.token_program;
        let authority = &ctx.accounts.from;

        // Transfer tokens from user to token vault
        let cpi_accounts = SplTransfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };
        let cpi_program = token_program.to_account_info();

        token::transfer(
            CpiContext::new(cpi_program, cpi_accounts),
            amount)?;
        Ok(())
    }
}

// Contexts
#[derive(Accounts)]
pub struct InitializeTokenVault<'info> {
    #[account(
        init,
        seeds = [
            signer.key().as_ref(),
        ],
        payer = signer,
        space = 8 + 32 + 8 + 32 + 8,
        bump
    )]
    pub token_vault: Account<'info, TokenVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSpl<'info> {
    pub from: Signer<'info>,
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// Accounts
#[account]
#[derive(Default)]
pub struct TokenVault {
    bonk_price: f64,
    wif_price: f64,
}