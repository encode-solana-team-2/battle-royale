use anchor_lang::prelude::*;

//Declare program ID
declare_id!("Cj6VD8qqwDj2fMk8G1EAFNV1v3w6AWJ5Sks6oRAkuspr");

//Account structure for each players wallet (add owner pubkey)
#[Account]
pub struct Wallet {
    pub tokens: u64,
    pub owner: Pubkey,
    pub metadata: String,
    
}

//Program
#[program]
mod competition {
    use super::*;

    //Initialize competition
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        let wallet =&mut ctx,account.wallet;
        wallet.tokens = 0;
        wallet.owner = *ctx.accounts.authority.key;
        wallet.metadata = "Some metadata".to_string();


        ctx.accounts.parameters.token_value = token_value;


        Ok(())
    }
//End the competition

pub fn end(ctx: Context<End>) -> Result<()> {

    let winner = find_winner(&ctx.acount.wallets)?:

    for wallet in ctx.accounts.wallets.iter() {
        if wallet.key().is_some() && wallet.key() == Some (winner.key()) {
            msg!("Winner: {:?}", wallet.key().unwrap());
        } else{
            msg!("Participant: {:?}", wallet.key().unwrap_or_default());
        }

        
    }

    Ok(())

}

//Status of competition

pub fn view_status(ctx: Context<ViewStatus>) -> Result<()> {

    //Rereive wallet accounts 
    let wallet = ctx.accounts.wallets;

    //Value per token
    let token_value = 1;

    //Total value for each wallet

    for wallet in wallets.iter() {
        let total_vaule = wallet.tokens * token_value;
        msg!(
            "Wallet {} - Tokens: {}, Total Value: {}",
            wallet.key,
            wallet.tokens,
            total_vaule
        );
    }

    Ok(())
  }

  fn find_winner(wallets: &[Account<Wallet>]) -> Result<&Account<Wallet>, Error> {

    if let Some(winner)= wallets.iter().max_by_key(|w| w.tokens) {
        Ok(winner)
    }else {
        Err(Error::NoWinner)
    }
  }

  pub enum Error {
    #[msg("No winner found")]
    NoWinner,
  }

}

#[derive(Accounts)]
pub struct Initialize<'info> {}
#[account(init, payer = authority, space = 8 + 64)]
pub wallet: ProgramAccount<'info, Wallet>,
pub authority: Signer<'info>,
pub parameters: ProgramAccount<'info, Parameters>,


#[derive{Accounts}]
pub struct End {}

#[derive(Accounts)]
pub struct ViewStatus<'info> {
    #[account(mut)]
    pub wallets: Vec<Accounts<'info, Wallet>>,
    pub parameters: ProgramAccount<'info, Parameters>,
}    

#[account]
pub struct Parameters {
    pub token_value: u64,
}