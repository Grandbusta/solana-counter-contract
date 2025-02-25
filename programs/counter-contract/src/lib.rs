use anchor_lang::prelude::*;

declare_id!("2frp6ukvEsqPsh6GaCjqxGf8CPnLDcijSmpgAfFGgVRE");

#[program]
pub mod counter_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;

        Ok(())
    }

    pub fn decrement(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count -= 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 1
    )]
    counter: Account<'info,Counter>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    counter: Account<'info, Counter>,

    #[account(mut)]
    payer: Signer<'info>,
}


#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    counter: Account<'info, Counter>,

    #[account(mut)]
    payer: Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u8,
}
