use anchor_lang::prelude::*;

declare_id!("GZePYX85VtK5aXyowai3wxhTapbWNMeMjYqAQDmL7Uqc");

#[program]
pub mod day_4 {
    use super::*;

    pub fn limit_range(ctxThen : Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, Day4Error::AisTooSmall);
        require!(a <= 100, Day4Error::AisTooBig);
        msg!("Result = {}", a);
        Ok(())
    }

    // NEW FUNCTION
    pub fn func(ctx: Context<ReturnError>) -> Result<()> {
        msg!("Will this print?");
        return err!(Day4Error::AlwaysErrors);
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[derive(Accounts)]
pub struct ReturnError {}

#[error_code]
pub enum Day4Error {
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("a is too big")]
    AisTooBig,
    #[msg("Always errors")]  // NEW ERROR, what do you think the error code will be?
    AlwaysErrors,
}