use anchor_lang::prelude::*;

declare_id!("upgpSaYnMQRqiQYKc2LVw5EvD9JJBj5UtmpUFJGjaHM");

#[program]
pub mod globecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
