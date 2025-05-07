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

    
    pub fn create_vault(
        ctx: Context<CreateVault>, 
        collateral_mint: Pubkey,
        stablecoin_mint: Pubkey,
        bump: u8, 
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.collateral_mint = collateral_mint;
        vault.stablecoin_mint = stablecoin_mint;
        vault.total_collateral = 0;
        vault.bump = bump;

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

#[account]
#[derive(Default, Debug)]
pub struct Vault {
    pub collateral_mint: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub total_collateral: u64,
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1; // Discriminator + Pubkeys + u64 + u8
} 


#[derive(Accounts)]
#[instruction(collateral_mint: Pubkey, stablecoin_mint: Pubkey, bump: u8)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // Or whoever has authority to create vaults

    #[account(init, payer = admin, space = 8 + Vault::LEN, 
        seeds = [b"vault", collateral_mint.as_ref(), stablecoin_mint.as_ref()], 
        bump,
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", collateral_mint.key().as_ref(), stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>, // Account to hold deposited collateral
    
    pub collateral_mint: Account<'info, Mint>, 
    pub stablecoin_mint: Account<'info, Mint>, 

    #[account(mut)]
    pub user_collateral_ata: Account<'info, TokenAccount>, // User's ATA for the current collateral 
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = vault,
    )]
    pub vault_collateral_ata: Account<'info, TokenAccount>, // Vault's ATA for the current collateral 

    #[account(
        init_if_needed,
        // init,
        payer = user,
        space = 8 + UserVaultBalance::LEN, // Temp figure
        seeds = [b"user_vault", user.key().as_ref(), vault.key().as_ref()],  // Unique key per user and its vault
        bump,
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
    #[account(
        mut,
        seeds = [b"vault", collateral_mint.key().as_ref(), stablecoin_mint.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>, // Account holding deposited collateral
    
    pub collateral_mint: Account<'info, Mint>,
    pub stablecoin_mint: Account<'info, Mint>,

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
    pub system_program: Program<'info, System>, 
}

impl<'info> WithdrawCollateral<'info> {
    fn transfer_collateral_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let seeds = &[
            b"vault",
            self.collateral_mint.key().as_ref(),
            self.stablecoin_mint.key().as_ref(),
            &[self.vault.bump],
        ];
        
        let signer_seeds = &[&seeds[..]];

        // CpiContext::new(
        //     self.token_program.to_account_info(),
        //     token::Transfer {
        //         from: self.vault_collateral_ata.to_account_info(),
        //         to: self.user_collateral_ata.to_account_info(),
        //         authority: self.vault.to_account_info(), // Vault needs to be the signer (requires PDA)
        //     },
        // )
        CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            token::Transfer {
                from: self.vault_collateral_ata.to_account_info(),
                to: self.user_collateral_ata.to_account_info(),
                authority: self.vault.to_account_info(), // Vault needs to be the signer (requires PDA)
            },
            signer_seeds,
        )
    }
}

#[derive(Accounts)]
pub struct MintStablecoin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", collateral_mint.key().as_ref(), stablecoin_mint.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    pub collateral_mint: Account<'info, Mint>,
    #[account(mut)]
    pub stablecoin_mint: Account<'info, Mint>,
    
    // #[account(mut)]
    // pub user_stablecoin_ata: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = user,
    )]
    pub user_stablecoin_ata: Account<'info, TokenAccount>,

    // #[account(mut)]
    // pub vault: AccountInfo<'info>, // Associated vault for the collateral



    // TODO: Maybe add checks in case of user has deposited enough collateral 
    pub token_program: Program<'info, Token>
}

impl<'info> MintStablecoin<'info> {
    fn mint_stablecoin_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let seeds = &[
            b"vault",
            self.collateral_mint.key().as_ref(),
            self.stablecoin_mint.key().as_ref(),
            &[self.vault.bump],
        ];
        
        let signer_seeds = &[&seeds[..]];

        // CpiContext::new(
        //     self.token_program.to_account_info(),
        //     token::MintTo {
        //         mint: self.stablecoin_mint.to_account_info(),
        //         to: self.user_stablecoin_ata.to_account_info(),
        //         authority: self.vault.to_account_info(), // Assuming vault needs to be the mint authority (requires PDA)
        //     },
        // ) 
        CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            token::MintTo {
                mint: self.stablecoin_mint.to_account_info(),
                to: self.user_stablecoin_ata.to_account_info(),
                authority: self.vault.to_account_info(), // Assuming vault needs to be the mint authority (requires PDA)
            },
            signer_seeds,
        ) 
    }
}

#[derive(Accounts)]
pub struct BurnStablecoin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // #[account(mut)]
    // pub vault: AccountInfo<'info>, // Associated vault for the collateral
    #[account(
        mut,
        seeds = [b"vault", collateral_mint.key().as_ref(), stablecoin_mint.key().as_ref()],
        bump,
    )]
    // pub vault: AccountInfo<'info>, // Associated vault for the collateral
    pub vault: Account<'info, Vault>, // Associated vault for the collateral

    pub collateral_mint: Account<'info, Mint>,
    #[account(mut)]
    pub stablecoin_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_stablecoin_ata: Account<'info, TokenAccount>,
    // #[account(mut)]
    // pub user_collateral_ata: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = vault,
    )]
    pub vault_collateral_ata: Account<'info, TokenAccount>, // To transfer collateral back
    // pub stablecoin_mint: Account<'info, Mint>, 

    // #[account(
    //     mut,
    //     seeds = [b"user_vault", user.key().as_ref(), vault.key().as_ref()],
    //     bump,
    // )]

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
        let seeds = &[
            b"vault",
            self.collateral_mint.key().as_ref(),
            self.stablecoin_mint.key().as_ref(),
            &[self.vault.bump],
        ];
        
        let signer_seeds = &[&seeds[..]];

        CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            token::Transfer {
                from: self.vault_collateral_ata.to_account_info(),
                to: self.user_collateral_ata.to_account_info(),
                authority: self.vault.to_account_info(),
            },
            signer_seeds,
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