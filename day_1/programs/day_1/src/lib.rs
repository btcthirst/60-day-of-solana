use anchor_lang::prelude::*;

declare_id!("GopXAkrJHyQRbo7wUTL3omCUrPcrSUQc5Hi7TecaN6Wx");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
