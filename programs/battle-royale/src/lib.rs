use anchor_lang::prelude::*;

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

#[account]
#[derive(Default)]
pub struct TokenVault {
    bonk_price: f64,
    wif_price: f64,
}