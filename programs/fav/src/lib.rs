use anchor_lang::prelude::*;

declare_id!("76DNVL4cmnev3znFBkPj2HZUrAa9jQN54L1vqciYEcQs");

#[program]
pub mod fav {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
