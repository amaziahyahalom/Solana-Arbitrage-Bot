use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug)]
pub struct ArbitrageState {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub source_token: Pubkey,
    pub destination_token: Pubkey,
    pub min_profit_threshold: u64,
    pub last_execution_slot: u64,
}

impl Sealed for ArbitrageState {}

impl IsInitialized for ArbitrageState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for ArbitrageState {
    const LEN: usize = 82; // 1 + 32 + 32 + 32 + 8 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ArbitrageState::LEN];
        let (
            is_initialized,
            authority,
            source_token,
            destination_token,
            min_profit_threshold,
            last_execution_slot,
        ) = array_refs![src, 1, 32, 32, 32, 8, 8];

        Ok(ArbitrageState {
            is_initialized: is_initialized[0] != 0,
            authority: Pubkey::new_from_array(*authority),
            source_token: Pubkey::new_from_array(*source_token),
            destination_token: Pubkey::new_from_array(*destination_token),
            min_profit_threshold: u64::from_le_bytes(*min_profit_threshold),
            last_execution_slot: u64::from_le_bytes(*last_execution_slot),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ArbitrageState::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            source_token_dst,
            destination_token_dst,
            min_profit_threshold_dst,
            last_execution_slot_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        source_token_dst.copy_from_slice(self.source_token.as_ref());
        destination_token_dst.copy_from_slice(self.destination_token.as_ref());
        *min_profit_threshold_dst = self.min_profit_threshold.to_le_bytes();
        *last_execution_slot_dst = self.last_execution_slot.to_le_bytes();
    }
} 