use anchor_lang::prelude::*;

declare_id!("2qfykMYmSRc8nU94maJsyRYqYc6ZnUnZLmyDpj7pKifX");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said{:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>,arr: Vec<u64>) -> Result<()> {
        msg!("You array {:?}", arr);
        Ok(())
    }

    pub fn cube_root(ctx: Context<Initialize>, a: f32) -> Result<()> {
        msg!("You cube {} : {:?}",a, a.cbrt());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

