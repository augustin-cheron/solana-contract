struct ImutableAccountId {
    tx: [u8;32],
    block_id:u64,
}

#[derive(Seriliaze, Deserialize)]
struct Mint {
    decimals: u64,
    freeze_authority: Option<Pubkey>,
    mint_authoriyt: Option<PubKey>,
    metadata: ImutableAccountId,
}

// imutable metadata imutable account should be less than 1kb
#[derive(Seriliaze, Deserialize)]
struct MintMeta {
    ticker: &str,
    name: &str,
    picture_hash:&str, // or valid image ?
    offchain_hash: &str, // url Vs Cid Vs Arweave ?
}

#[derive(Seriliaze, Deserialize)]
struct TokenAccount {
    mint: AccountID<Mint>,
    amount: u64
}

fn create_token() {
    // create_token_meta
    // check metadata
    // send metadata to proxy contract for imutable data
}

fn save_meta_data() {
    // check call from contract
    // check program id is caller / signer
}

fn transfer(Account(from) : Account<TokenAccount>, Account(to) : Account<TokenAccount>, amount : u64) {
    if from.mint != to.mint {
        return Err(error::Mintmismatch);
    }

    if from.amount < amount {
        return Err(error::NotEnoughtFund);
    }

    // should use non overflowing sub and add
    from.amount -= amount;
    to.amount += amount;

    Ok(())
}

// add DAO to avoid name squating
// SOL based vote
// send a dispute
// SOL holder can vote to switch metadata owner
// If metadata is transfer token as no metada but holder stay the same
// Protocol take fees on transaction
// User can sell metadata ?

// NFT are store on chain
// give nice client for rust / js / android / ios / go ?
// provide a price tacker ?

// provide pools for token
// Fixed pool against an other token (wrapped token)
// auto wrap + transfert in client

// by default have single token adress per wallet
// single token account per wallet ?
// support for anonymous transfer using tornado cash like system ?
// encrypt


fn create_token_mint(mint_authority: Signer, mint_account: Writable<AccountInfo>) {

}

fn mint(mint_authority: Signer, to: TokenAccount, amount:u64) {

}

fn send_token(
    // accounts
    from: Signer,
    mint : MintAccount,
    from_token_account : Writable<TokenAccount>,
    to_token_account : Writable<TokenAccount>,
    token_program : ProgramId,
    // data
    amount : u64
) {
    // assert program id
    // assert token mint match token accounts mint
    // assert amount ok

    // call program token with given rust sdk ?
    // build instruction
}


// surcouche avec le derive auto ?
struct SlclTokenAccount;

impl Pda for SlclTokenAccount {
    fn derive() {

    }
}

// we need to check that account is derived with the right info but name will be extracted from params.
// user need to impl the check


// no dervie
fn send_token_instr(mint : MintAccount, from : TokenAcount) {
    assert(from.mint == mint.id);
    assert()
    // how to check that Mint is USDC
    // how to check tokenProgram os token Program
    // will need dynamik parssing of accounts to find the mint one ?

}

struct UsdcMint {

}

impl UsdcMint {
    fn send_token(token_program: ProgramId) {}
    
}

// Can I get a mint type tha allow to send ?

impl FromAccount for UsdcMint {
    fn from_account(acc : AccountInfo) {
        let token_account = TokenAccount::fromAccount(acc);
        token.mint == USDC_MINT;
        token.token_program = TOKEN_PROGRAM_ID;
    }
}