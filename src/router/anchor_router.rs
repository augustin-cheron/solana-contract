#[macro_export]
macro_rules! anchor_dispatch {
    ($($func:ident),*) => {
        use crate::handler::InstrFn;

        const fn hashid(f_name:&'static str) -> [u8;8] {
            let b = f_name.as_bytes();
            [
                b[0],b[1],b[2],b[3],
                b[4],b[5],b[6],b[7]
            ]
        }

        enum Func {
            $($func),*
        }

        pub fn process_instruction(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            data: &[u8],
        ) -> ProgramResult {
            if !check_id(program_id) {
                return Err(ProgramError::IncorrectProgramId);
            }
            let intr_idx = data[0] as usize;
            if intr_idx >= INSTRUCTIONS.len() {
                return Err(ProgramError::InvalidArgument);
            }
            match(data[0..8]) {
                $(Func::$func => $func.handle_request(accounts, data)?,)*
                _=> return Err(ProgramError::InvalidArgument)
            };
            Ok(())
        }
        entrypoint!(process_instruction);
    };
}
