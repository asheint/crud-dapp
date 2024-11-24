#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod cruddapp {
    use super::*;

  pub fn close(_ctx: Context<CloseCruddapp>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.cruddapp.count = ctx.accounts.cruddapp.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.cruddapp.count = ctx.accounts.cruddapp.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeCruddapp>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.cruddapp.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeCruddapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Cruddapp::INIT_SPACE,
  payer = payer
  )]
  pub cruddapp: Account<'info, Cruddapp>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseCruddapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub cruddapp: Account<'info, Cruddapp>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub cruddapp: Account<'info, Cruddapp>,
}

#[account]
#[derive(InitSpace)]
pub struct Cruddapp {
  count: u8,
}
