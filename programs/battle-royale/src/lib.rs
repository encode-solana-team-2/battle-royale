use anchor_lang::prelude::*;

declare_id!("Cj6VD8qqwDj2fMk8G1EAFNV1v3w6AWJ5Sks6oRAkuspr");

#[program]
pub mod battle_royale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
