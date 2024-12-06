use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Arbitrage program entrypoint");

    // Get account iterator
    let accounts_iter = &mut accounts.iter();

    // Get accounts
    let authority = next_account_info(accounts_iter)?;
    let source_token = next_account_info(accounts_iter)?;
    let destination_token = next_account_info(accounts_iter)?;
    let dex_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Verify authority
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Process the instruction based on the instruction data
    let instruction = ArbitrageInstruction::unpack(instruction_data)?;
    
    match instruction {
        ArbitrageInstruction::InitializeArbitrage { amount } => {
            process_initialize_arbitrage(
                program_id,
                authority,
                source_token,
                destination_token,
                dex_program,
                token_program,
                amount,
            )
        }
        ArbitrageInstruction::ExecuteArbitrage { route } => {
            process_execute_arbitrage(
                program_id,
                authority,
                source_token,
                destination_token,
                dex_program,
                token_program,
                route,
            )
        }
    }
} 