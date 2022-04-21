use anchor_lang::prelude::*;
use anchor_spl::dex::serum_dex::instruction::{
    CancelOrderInstructionV2, NewOrderInstructionV3,
};
use anchor_spl::dex::{
    Context as ContextDex, Logger, MarketMiddleware, MarketProxy, OpenOrdersPda,
};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

declare_id!("2uVbokHLkdS1LyCQYasbJ917F52ekrvopadwp73bSWKN");

pub const CORE_STATE_SEED: &str = "core-state";

#[program]
pub mod short_serum_market {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> ProgramResult {
        ctx.accounts.core_state.admin = ctx.accounts.admin.key();
        ctx.accounts.core_state.core_state_nonce = args.core_state_nonce;
        let clock = Clock::get()?;
        ctx.accounts.core_state.expiry = 
            (clock.unix_timestamp as u64) + (args.expiry_period_days * 86400);
        
        Ok(())
    }

    pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        MarketProxy::new()
            .middleware(&mut Logger)
            .middleware(&mut Identity)
            .middleware(&mut OpenOrdersPda::new())
            .run(program_id, accounts, data)
    }
}

#[derive(Accounts)]
#[instruction(args: InitializeArgs)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        seeds = [CORE_STATE_SEED.as_bytes().as_ref(), admin.key().as_ref()],
        bump = args.core_state_nonce,
        payer = admin,
    )]
    pub core_state: Account<'info, CoreState>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeArgs {
    pub expiry_period_days: u64, // in day
    pub core_state_nonce: u8,
}

#[account]
#[derive(Default)]
pub struct CoreState {
    pub core_state_nonce: u8,
    pub expiry: u64,
    pub admin: Pubkey,
}

struct Identity;

impl Identity {

}

impl MarketMiddleware for Identity {
    fn init_open_orders(&self, ctx: &mut ContextDex) -> ProgramResult {
        check_expiry(ctx)
    }

    fn new_order_v3(&self, ctx: &mut ContextDex, _ix: &NewOrderInstructionV3) -> ProgramResult {
        check_expiry(ctx)
    }

    fn cancel_order_v2(&self, ctx: &mut ContextDex, _ix: &CancelOrderInstructionV2) -> ProgramResult {
        check_expiry(ctx)
    }

    fn cancel_order_by_client_id_v2(&self, ctx: &mut ContextDex, _client_id: u64) -> ProgramResult {
        check_expiry(ctx)
    }

    fn settle_funds(&self, ctx: &mut ContextDex) -> ProgramResult {
        check_expiry(ctx)
    }

    fn close_open_orders(&self, ctx: &mut ContextDex) -> ProgramResult {
        check_expiry(ctx)
    }

    fn prune(&self, ctx: &mut ContextDex, _limit: u16) -> ProgramResult {
        check_expiry(ctx)
    }

    fn fallback(&self, ctx: &mut ContextDex) -> ProgramResult {
        check_expiry(ctx)
    }
}

fn check_expiry(ctx: &mut ContextDex) -> ProgramResult {
    let state = CoreState::try_from_slice(&ctx.accounts[0].try_borrow_data()?)?;
    
    let clock = Clock::get()?;
    require!(clock.unix_timestamp as u64 <= state.expiry, ExpiredMarketError);

    ctx.accounts = (&ctx.accounts[1..]).to_vec();
    Ok(())
}

#[error]
pub enum ErrorCode {
    #[msg("Unknown Error")]
    UnknownError,
    #[msg("Expired Market Error")]
    ExpiredMarketError,
}
