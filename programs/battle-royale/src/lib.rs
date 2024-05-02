use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};

declare_id!("Cj6VD8qqwDj2fMk8G1EAFNV1v3w6AWJ5Sks6oRAkuspr");

#[program]
pub mod battle_royale {
    use super::*;

    // Creates an account for the game
    pub fn new_battle(ctx: Context<NewBattle>, token_one: Pubkey, token_one_price: f64, token_two: Pubkey, token_two_price: f64) -> Result<()> {
        ctx.accounts
            .battle
            .new([ctx.accounts.signer.key(), token_one, token_one_price, token_two, token_two_price])
    }

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
pub struct NewBattle<'info> {
    #[account(
        init,
        payer = signer,
        space = 64 +  Battle::MAXIMUM_SIZE,
        seeds = [
            signer.key().as_ref(),
        ],
        bump
    )]
    pub battle: Account<'info, Battle>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

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
pub struct Battle {
    token_one: Pubkey,
    token_one_price: f64,
    token_one_balance: u64,
    token_two: Pubkey,
    token_two_price: f64,
    token_two_balance: u64,
    winner: String,
}

impl Battle {
    // Based on account varfiable sizes
    pub const MAXIMUM_SIZE: usize = (32 * 2) + (32 * 2) + 3 * 2;

    // Player that pays for account set up calls this with both pubkeys
    pub fn new(&mut self, token_one: Pubkey, token_one_price: f64, token_two: Pubkey, token_two_price: f64) -> Result<()> {
        self.token_one = token_one;
        self.token_one_price = token_one_price;
        self.token_one_balance = 0;
        self.token_two = token_two;
        self.token_two_price = token_two_price;
        self.token_two_balance = 0;
        Ok(())
    }

    pub fn get_player_index(&mut self, player: Pubkey) -> Result<usize> {
        let index_player: usize = self.players.iter().position(|&x| x == player).unwrap();

        match index_player {
            0 => Ok(index_player),
            1 => Ok(index_player),
            _ => Err(SErrors::MissingPlayer.into()),
        }
    }

    pub fn pick_winner(&mut self) -> HandResult {
        let (player1, player2) = (self.hand[0].beats(), self.hand[1].beats());

        msg!("player1 hand: {:?}", self.hand[0]);
        msg!("player2 hand: {:?}", self.hand[1]);

        match (player1, player2) {
            _ if player1 == self.hand[1] => Win,
            _ if player2 == self.hand[0] => Lose,
            _ => Draw,
        }
    }

    pub fn place_hash(&mut self, hashed_hand: [u8; 32], indx: usize) -> Result<()> {
        // Set hash
        self.hashed_hand[indx] = hashed_hand;

        // Mark submission
        self.hash_submitted[indx] = true;

        Ok(())
    }

    pub fn place_hand(&mut self, hand_string: String, indx: usize) -> Result<()> {
        // Extract the first word
        let words: Vec<&str> = hand_string.split(' ').collect();

        // Hash first word
        let new_hash = hash(hand_string.as_bytes());

        // Check if the same
        if new_hash == Hash::new_from_array(self.hashed_hand[indx]) {
            // Extract hand from the first char of the frist word
            let hand: Hand = Hand::new(words[0].chars().next().unwrap()).unwrap();

            // Place hand
            self.hand[indx] = hand;

            // Mark as loaded
            self.hand_submitted[indx] = true;

            // Check if the end ie final hand
            if self.hand_submitted[0] == true && self.hand_submitted[1] == true {
                let result_p1 = self.pick_winner();

                match result_p1 {
                    Win => self.winner = self.players[0].to_string(),
                    Lose => self.winner = self.players[1].to_string(),
                    Draw => self.winner = "DRAW".to_string(),
                }
            }

            return Ok(());
        }

        return Err(SErrors::WrongHash.into());
    }
}

#[account]
#[derive(Default)]
pub struct TokenVault {
    bonk_price: f64,
    wif_price: f64,
}