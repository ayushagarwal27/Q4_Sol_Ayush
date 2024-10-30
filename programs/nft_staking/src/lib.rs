use anchor_lang::prelude::*;

declare_id!("3moWTBUxJaV1CV9EQ4G62Ms8ppmcP41o1wuUUnaKYg13");

mod state;
mod instructions;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
