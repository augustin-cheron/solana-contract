use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
};

pub trait InstrFn<'a, 'b, Arg>: Sized + 'static {
    fn handle_request(&self, accounts: &[AccountInfo<'a>], data: &'b [u8]) -> ProgramResult;
}

impl<'a, 'b, F> InstrFn<'a, 'b, ((),)> for F
where
    F: Fn() -> ProgramResult + 'static,
{
    fn handle_request(&self, _accounts: &[AccountInfo<'a>], _data: &[u8]) -> ProgramResult {
        self()
    }
}

impl<'a, 'b, F, T> InstrFn<'a, 'b, ((), T)> for F
where
    F: Fn(T) -> ProgramResult + 'static,
    T: FromData<'b>,
{
    fn handle_request(&self, _accounts: &[AccountInfo<'a>], data: &'b [u8]) -> ProgramResult {
        let param = T::from_data(data)?;
        self(param)
    }
}

impl<'a, 'b, F, A> InstrFn<'a, 'b, ((), (), A)> for F
where
    F: Fn(A) -> ProgramResult + 'static,
    A: FromAccounts<'a>,
{
    fn handle_request(&self, accounts: &[AccountInfo<'a>], _data: &'b [u8]) -> ProgramResult {
        let acc = A::from_accounts(accounts)?;
        self(acc)
    }
}

impl<'a, 'b, F, A, P> InstrFn<'a, 'b, ((), A, P)> for F
where
    F: Fn(A, P) -> ProgramResult + 'static,
    A: FromAccounts<'a>,
    P: FromData<'b>,
{
    fn handle_request(&self, accounts: &[AccountInfo<'a>], data: &'b [u8]) -> ProgramResult {
        let acc = A::from_accounts(accounts)?;
        let param = P::from_data(data)?;
        self(acc, param)
    }
}

macro_rules! impl_account_only {
    (
        $($ty:ident),*
    ) => {
        #[allow(non_snake_case)]
        impl<'a,'b, F, $($ty,)*> InstrFn<'a,'b, ($($ty,)*)> for F
        where
            F: Fn($(&$ty,)*) -> ProgramResult + 'static,
            $($ty:FromAccount<'a>,)*
        {
            fn handle_request(&self, accounts: &[AccountInfo<'a>], _data: &'b [u8]) -> ProgramResult {
                let ai = &mut accounts.iter();
                $(let $ty = $ty::from_account(next_account_info(ai)?);)*
                self($(&$ty,)*)
            }
        }
    }
}

impl_account_only!(A0);
impl_account_only!(A0, A1);
impl_account_only!(A0, A1, A2, A3, A4);
impl_account_only!(A0, A1, A2, A3, A4, A5);
impl_account_only!(A0, A1, A2, A3, A4, A5, A6);
impl_account_only!(A0, A1, A2, A3, A4, A5, A6, A7);
impl_account_only!(A0, A1, A2, A3, A4, A5, A6, A7, A8);
impl_account_only!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_account_only!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

macro_rules! impl_with_data {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case)]
        impl<'a,'b, F, $($ty,)* $last> InstrFn<'a,'b, ($($ty,)* $last,())> for F
        where
            F: Fn($(&$ty,)* $last) -> ProgramResult + 'static,
            $($ty:FromAccount<'a>,)*
            $last:FromData<'b>,
        {
            fn handle_request(&self, accounts: &[AccountInfo<'a>], data: &'b [u8]) -> ProgramResult {
                let ai = &mut accounts.iter();
                $(let $ty = $ty::from_account(next_account_info(ai)?);)*
                let data = $last::from_data(data)?;
                self($(&$ty,)* data)
            }
        }
    }
}

impl_with_data!([A0], D);
impl_with_data!([A0, A1], D);
impl_with_data!([A0, A1, A2, A3, A4], D);
impl_with_data!([A0, A1, A2, A3, A4, A5], D);
impl_with_data!([A0, A1, A2, A3, A4, A5, A6], D);
impl_with_data!([A0, A1, A2, A3, A4, A5, A6, A7], D);
impl_with_data!([A0, A1, A2, A3, A4, A5, A6, A7, A8], D);
impl_with_data!([A0, A1, A2, A3, A4, A5, A6, A7, A8, A9], D);
impl_with_data!([A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10], D);

pub trait FromAccount<'a> {
    fn from_account(account: &AccountInfo<'a>) -> Self;
}

impl<'a> FromAccount<'a> for AccountInfo<'a> {
    fn from_account(account: &AccountInfo<'a>) -> Self {
        // TODO: avoid this clone ?
        account.clone()
    }
}

pub trait FromAccounts<'a> {
    fn from_accounts(accounts: &[AccountInfo<'a>]) -> Result<Self, ProgramError>
    where
        Self: Sized;
}


pub trait FromData<'a> {
    fn from_data(data: &'a [u8]) -> Result<Self, ProgramError>
    where
        Self: Sized;
}

/*
impl<'a, T> FromData<'a> for T where T : Deserialize<'a> {
    fn from_data(data: &'a [u8]) -> Result<T, ProgramError> where T: Sized {
        postcard::from_bytes(data)
        .map_err(move |_err| ProgramError::InvalidArgument)
    }
}
*/

impl<'a, T> FromData<'a> for T
where
    T: BorshDeserialize + std::fmt::Debug,
{
    fn from_data(data: &'a [u8]) -> Result<T, ProgramError>
    where
        T: Sized,
    {
        let res = T::try_from_slice(&data[1..]);
        msg!("from_data : {:?}, {:?}", data, res);
        res.map_err(move |err| {
            msg!("{:?}", err);
            ProgramError::InvalidArgument
        })
    }
}
