use anchor_lang::prelude::*;

declare_id!("FUM4VGMhGoz5L19EB1XYopH6SY8mnTNJUohsFanRAahn");

#[program]
pub mod globecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
