#[macro_export]
macro_rules! index_dispatch {
    ($($func:ident),*) => {
        pub const INSTRUCTIONS: &[fn(&[AccountInfo], &[u8]) -> ProgramResult] = &[
            $(|accounts: &[AccountInfo], data: &[u8]| $func.handle_request(accounts, data),)*
        ];

        pub fn process_instruction(
            program_id: &solana_program::pubkey::Pubkey,
            accounts: &[solana_program::account_info::AccountInfo],
            data: &[u8],
        ) -> ProgramResult {
            if !check_id(program_id) {
                return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
            }
            let intr_idx = data[0] as usize;
            if intr_idx >= INSTRUCTIONS.len() {
                return Err(solana_program::program_error::ProgramError::InvalidArgument);
            }
            INSTRUCTIONS[intr_idx](accounts, data)
        }
        solana_program::entrypoint!(process_instruction);
    };
}
