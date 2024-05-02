// still testing

use anchor_lang::prelude::*;

declare_id!("Cj6VD8qqwDj2fMk8G1EAFNV1v3w6AWJ5Sks6oRAkuspr");

#[program]
pub mod battle_royale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bonk_mint: Pubkey, wif_mint: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.bonk_mint = bonk_mint;
        state.wif_mint = wif_mint;
        state.bonk_deposits = 0;
        state.wif_deposits = 0;
        state.start_time = Clock::get()?.unix_timestamp;
        state.end_time = state.start_time + 300; // 24 hours
        Ok(())
    }

    pub fn deposit_bonk(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        require!(Clock::get()?.unix_timestamp <= state.end_time, ErrorCode::GameClosed);
        state.bonk_deposits += amount;
        // Logic to transfer BONK tokens to the contract's address
        Ok(())
    }

    pub fn deposit_wif(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        require!(Clock::get()?.unix_timestamp <= state.end_time, ErrorCode::GameClosed);
        state.wif_deposits += amount;
        // Logic to transfer WIF tokens to the contract's address
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let state = &ctx.accounts.state;
        require!(Clock::get()?.unix_timestamp > state.end_time, ErrorCode::GameNotClosed);
        // Logic to return tokens based on game result
        Ok(())
    }
}

#[account]
pub struct GameState {
    pub bonk_mint: Pubkey,
    pub wif_mint: Pubkey,
    pub bonk_deposits: u64,
    pub wif_deposits: u64,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 8 + 8 + 8 + 8)]
    pub state: Account<'info, GameState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub state: Account<'info, GameState>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub state: Account<'info, GameState>,
}