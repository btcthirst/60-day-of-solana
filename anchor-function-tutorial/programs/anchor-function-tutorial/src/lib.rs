use anchor_lang::prelude::*;

declare_id!("71B14sKRSwVXWxuxEgbYkecpZre2XLbZYWduZuZNxxLo");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<Empty>) -> Result<()> {        
        Ok(())
    }

    pub fn add(ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("Sum is {}", sum);
        Ok(())
    }

    pub fn sub(ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let difference = a - b;
        msg!("Difference is {}", difference);
        Ok(())
    }

    pub fn mul(ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let multiply = a * b;
        msg!("Result is {}", multiply);
        Ok(())
    }

    pub fn div(ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let res = a / b;
        msg!("Result is {}", res);
        Ok(())
    }

    pub fn function_a(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }

    pub fn function_b(ctx: Context<NonEmptyAccountExample>, firstArg: u64) -> Result<()> {
        Ok(())
    }

}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Empty {}
