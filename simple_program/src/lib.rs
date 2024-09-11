use borsh::BorshDeserialize;
use solana_contract::handler::InstrFn;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg};
use solana_contract::FromAccounts;

use solana_contract::solana_program;

solana_program::declare_id!("4msBmu3U2sTA2i5CKjdrzwEMq3YFNBSwSKhNB5hbhWwk");

solana_contract::index_dispatch![
    simple_one,
    simple_ctx,
    simple_ctx_data
];

fn simple_one<'a>(account: &AccountInfo<'a>) -> ProgramResult {
    msg!("Call simple_one with account {:?}", account.key);
    Ok(())
}

#[derive(Debug, FromAccounts)]
struct SimpleContext<'a> {
    one: AccountInfo<'a>,
    two: AccountInfo<'a>,
    three: AccountInfo<'a>,
}

fn simple_ctx(context: SimpleContext) -> ProgramResult {
    msg!("Call simple_ctx with context {:#?}", context);
    Ok(())
}

#[derive(Debug, BorshDeserialize)]
struct SimpleParams {
    amount: u64,
}

fn simple_ctx_data(context: SimpleContext, params: SimpleParams) -> ProgramResult {
    msg!(
        "Call simple_ctx_data with context {:#?} and params {:#?}",
        context,
        params
    );
    Ok(())
}
