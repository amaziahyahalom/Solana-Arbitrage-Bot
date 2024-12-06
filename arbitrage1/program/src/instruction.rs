use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::convert::TryInto;

#[derive(Debug)]
pub enum ArbitrageInstruction {
    /// Initialize an arbitrage opportunity
    ///
    /// Accounts expected:
    /// 0. `[signer]` The account of the person initializing the arbitrage
    /// 1. `[writable]` The source token account
    /// 2. `[writable]` The destination token account
    /// 3. `[]` The DEX program
    /// 4. `[]` The token program
    InitializeArbitrage {
        amount: u64,
    },

    /// Execute an arbitrage trade
    ///
    /// Accounts expected:
    /// 0. `[signer]` The account of the person executing the arbitrage
    /// 1. `[writable]` The source token account
    /// 2. `[writable]` The destination token account
    /// 3. `[]` The DEX program
    /// 4. `[]` The token program
    ExecuteArbitrage {
        route: Vec<Pubkey>,
    },
}

impl ArbitrageInstruction {
    /// Unpacks a byte buffer into an ArbitrageInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                Self::InitializeArbitrage { amount }
            }
            1 => {
                let route_len = rest
                    .get(..4)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u32::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)? as usize;
                
                let mut route = Vec::with_capacity(route_len);
                let mut offset = 4;
                
                for _ in 0..route_len {
                    let pubkey = Pubkey::new(&rest[offset..offset + 32]);
                    route.push(pubkey);
                    offset += 32;
                }
                
                Self::ExecuteArbitrage { route }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
} 