use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, Mint, Token, TokenAccount,
};

declare_id!("upgpSaYnMQRqiQYKc2LVw5EvD9JJBj5UtmpUFJGjaHM");

#[program]
pub mod multi_currency_stablecoin {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        // 1. Transfer collateral from user to vault
        token::Transfer(ctx.accounts.transfer_collateral_ctx(), amount)?;

        // 2. Update user vault balance
        let mut user_vault_balance= ctx.accounts.user_vault_balance.load_init()?;
        user_vault_balance.deposit_amount += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>, // Account to hold deposited collateral
    
    #[account(mut)]
    pub user_collateral_ata: Account<'info, TokenAccount>, // User's ATA for the current collateral 
    #[account(mut)]
    pub vault_collateral_ata: Account<'info, TokenAccount>, // Vault's ATA for the current collateral 

    #[account(
        // init_if_needed,
        init,
        payer = user,
        space = 8 + UserVaultBalance::LEN, // Temp figure
        seeds = [b"user_vault", user.key().as_ref(), vault.key().as_ref()],  // Unique key per user and its vault
        bump
    )]
    pub user_vault_balance: Account<'info, UserVaultBalance>,

    // pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositCollateral<'info> {
    fn transfer_collateral_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_context = CpiContext::new(
            self.token_program.to_account_info(),
            token::Transfer {
                from: self.user_collateral_ata.to_account_info(),
                to: self.vault_collateral_ata.to_account_info(),
                authority: self.user.to_account_info(),
            },
        );
    }
}

// #[derive(Accounts)]
// pub struct WithdrawCollateral<'info> {
    
// }

// #[derive(Accounts)]
// pub struct MintStablecoin<'info> {

// }

#[account]
#[derive(Default, Debug)]
pub struct UserVaultBalance {
    pub deposit_amount: u64,
}

impl UserVaultBalance {
    pub const LEN: usize = 8 + 8; // Discriminator + deposit amount
}