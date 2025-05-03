#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, Mint, Token, TokenAccount,
};

declare_id!("upgpSaYnMQRqiQYKc2LVw5EvD9JJBj5UtmpUFJGjaHM");

#[program]
pub mod multi_currency_stablecoin {
    // use anchor_spl::token::burn;

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        // 1. Transfer collateral from user to vault
        token::transfer(ctx.accounts.transfer_collateral_ctx(), amount)?;

        // 2. Update user vault balance
        // let mut user_vault_balance = ctx.accounts.user_vault_balance.load_init()?;
        // let mut user_vault_balance = ctx.accounts.user_vault_balance.init()?;
        let user_vault_balance = &mut ctx.accounts.user_vault_balance;
        user_vault_balance.deposit_amount += amount;

        Ok(())
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
        token::transfer(ctx.accounts.transfer_collateral_ctx(), amount)?;

        // let mut user_vault_balance = ctx.accounts.user_vault_balance.load_mut()?;
        let user_vault_balance = &mut ctx.accounts.user_vault_balance;
        user_vault_balance.deposit_amount -= amount;
        
        Ok(())
    }

    pub fn mint_stablecoin(ctx: Context<MintStablecoin>, amount: u64) -> Result<()> {
        // Assuming 1:1 reserve-backed for now
        token::mint_to(ctx.accounts.mint_stablecoin_ctx(), amount)?;

        // TODO: Update user's stablecoin balance in a custom account if needed
        Ok(())
    }

    pub fn burn_stablecoin(ctx: Context<BurnStablecoin>, amount:  u64) -> Result<()> {
        // Assuming 1:1 reserve-backed for now
        token::burn(ctx.accounts.burn_stablecoin_ctx(), amount)?;

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
        init_if_needed,
        // init,
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
        // let cpi_context = CpiContext::new(
        CpiContext::new(
            self.token_program.to_account_info(),
            token::Transfer {
                from: self.user_collateral_ata.to_account_info(),
                to: self.vault_collateral_ata.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }
}

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>, // Account holding deposited collateral
    
    #[account(mut)]
    pub user_collateral_ata: Account<'info, TokenAccount>, // User's ATA for the collateral token
    #[account(mut)]
    pub vault_collateral_ata: Account<'info, TokenAccount>, // Vault's ATA for the collateral token

    #[account(
        mut, 
        seeds = [b"user_vault", user.key().as_ref(), vault.key().as_ref()], 
        bump
    )]
    pub user_vault_balance: Account<'info, UserVaultBalance>,

    pub token_program: Program<'info, Token>, 

}

impl<'info> WithdrawCollateral<'info> {
    fn transfer_collateral_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            token::Transfer {
                from: self.vault_collateral_ata.to_account_info(),
                to: self.user_collateral_ata.to_account_info(),
                authority: self.vault.to_account_info(), // Vault needs to be the signer (requires PDA)
            },
        )
    }
}

#[derive(Accounts)]
pub struct MintStablecoin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub stablecoin_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub user_stablecoin_ata: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub vault: AccountInfo<'info>, // Associated vault for the collateral

    // TODO: Maybe add checks in case of user has deposited enough collateral 
    pub token_program: Program<'info, Token>
}

impl<'info> MintStablecoin<'info> {
    fn mint_stablecoin_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            token::MintTo {
                mint: self.stablecoin_mint.to_account_info(),
                to: self.user_stablecoin_ata.to_account_info(),
                authority: self.vault.to_account_info(), // Assuming vault needs to be the mint authority (requires PDA)
            },
        ) 
    }
}

#[derive(Accounts)]
pub struct BurnStablecoin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>, // Associated vault for the collateral

    #[account(mut)]
    pub user_stablecoin_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_collateral_ata: Account<'info, TokenAccount>,
    
    
    #[account(mut)]
    pub vault_collateral_ata: Account<'info, TokenAccount>, // To transfer collateral back

    #[account(mut)]
    pub stablecoin_mint: Account<'info, Mint>, 

    #[account(
        mut, 
        seeds = [b"user_vault", user.key().as_ref(), vault.key().as_ref()],
        bump,
    )]
    pub user_vault_balance: Account<'info, UserVaultBalance>,

    pub token_program: Program<'info, Token>
}

impl<'info> BurnStablecoin<'info> {
    fn burn_stablecoin_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            token::Burn {
                mint: self.stablecoin_mint.to_account_info(),
                from: self.user_stablecoin_ata.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn transfer_collateral_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            token::Transfer {
                from: self.vault_collateral_ata.to_account_info(),
                to: self.user_collateral_ata.to_account_info(),
                authority: self.vault.to_account_info(),
            },
        )
    }
}

#[account]
#[derive(Default, Debug)]
pub struct UserVaultBalance {
    pub deposit_amount: u64,
}

impl UserVaultBalance {
    pub const LEN: usize = 8 + 8; // Discriminator + deposit amount
}