use crate::handler::InstrFn;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub struct Service<'a, 'b> {
    instructions: Vec<Box<dyn Fn(&[AccountInfo<'a>], &'b [u8]) -> ProgramResult>>,
}

impl<'a, 'b> Service<'a, 'b> {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
        }
    }

    pub fn instruction<H, A>(mut self, handler: H) -> Self
    where
        H: InstrFn<'a, 'b, A> + 'static,
        A: Sized,
    {
        let inner_f = Box::new(move |accounts: &[AccountInfo<'a>], data: &'b [u8]| {
            handler.handle_request(accounts, data)
        });
        self.instructions.push(inner_f);
        self
    }

    pub fn process_instruction(
        &self,
        _program_id: &Pubkey,
        accounts: &[AccountInfo<'a>],
        data: &'b [u8],
    ) -> ProgramResult {
        let intr_idx = data[0] as usize;
        self.instructions[intr_idx](accounts, data)
    }
}
