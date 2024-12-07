use anchor_lang::prelude::*;

////////////////////////////////////////////////////////////////////////////////
// Constant
////////////////////////////////////////////////////////////////////////////////
pub(crate) const NUM_REWARDS: usize = 3;
pub(crate) const TICK_ARRAY_SIZE_USIZE: usize = 88;
pub(crate) const POSITION_BITMAP_USIZE: usize = 32;

////////////////////////////////////////////////////////////////////////////////
// Account
////////////////////////////////////////////////////////////////////////////////
#[account]
pub struct WhirlpoolsConfig {
  pub fee_authority: Pubkey,
  pub collect_protocol_fees_authority: Pubkey,
  pub reward_emissions_super_authority: Pubkey,
  pub default_protocol_fee_rate: u16,
}

impl WhirlpoolsConfig {
  pub const LEN: usize = 8 + 96 + 4;
}

#[account]
pub struct FeeTier {
  pub whirlpools_config: Pubkey,
  pub tick_spacing: u16,
  pub default_fee_rate: u16,
}

impl FeeTier {
  pub const LEN: usize = 8 + 32 + 4;
}

#[account]
pub struct Whirlpool {
  pub whirlpools_config: Pubkey, 
  pub whirlpool_bump: [u8; 1],
  pub tick_spacing: u16,
  pub tick_spacing_seed: [u8; 2],
  pub fee_rate: u16,
  pub protocol_fee_rate: u16,
  pub liquidity: u128,
  pub sqrt_price: u128,
  pub tick_current_index: i32,
  pub protocol_fee_owed_a: u64,
  pub protocol_fee_owed_b: u64,
  pub token_mint_a: Pubkey,
  pub token_vault_a: Pubkey,
  pub fee_growth_global_a: u128,
  pub token_mint_b: Pubkey,
  pub token_vault_b: Pubkey,
  pub fee_growth_global_b: u128,
  pub reward_last_updated_timestamp: u64,
  pub reward_infos: [WhirlpoolRewardInfo; NUM_REWARDS],
}

impl Whirlpool {
  pub const LEN: usize = 8 + 261 + 384;
}

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
pub struct TickArray {
  pub start_tick_index: i32,
  pub ticks: [Tick; TICK_ARRAY_SIZE_USIZE],
  pub whirlpool: Pubkey,
}

impl TickArray {
  pub const LEN: usize = 8 + 36 + (Tick::LEN * TICK_ARRAY_SIZE_USIZE);
}

#[account]
pub struct Position {
  pub whirlpool: Pubkey,
  pub position_mint: Pubkey,
  pub liquidity: u128,
  pub tick_lower_index: i32,
  pub tick_upper_index: i32,
  pub fee_growth_checkpoint_a: u128,
  pub fee_owed_a: u64,
  pub fee_growth_checkpoint_b: u128,
  pub fee_owed_b: u64,
  pub reward_infos: [PositionRewardInfo; NUM_REWARDS],
}

impl Position {
  pub const LEN: usize = 8 + 136 + 72;
}

#[account]
#[derive(Default)]
pub struct PositionBundle {
  pub position_bundle_mint: Pubkey,
  pub position_bitmap: [u8; POSITION_BITMAP_USIZE],
}

impl PositionBundle {
  pub const LEN: usize = 8 + 32 + 32 + 64;
}

#[account]
pub struct WhirlpoolsConfigExtension {
    pub whirlpools_config: Pubkey,
    pub config_extension_authority: Pubkey,
    pub token_badge_authority: Pubkey,
}

impl WhirlpoolsConfigExtension {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 512;
}

#[account]
#[derive(Default)]
pub struct TokenBadge {
    pub whirlpools_config: Pubkey,
    pub token_mint: Pubkey,
}

impl TokenBadge {
    pub const LEN: usize = 8 + 32 + 32 + 128;
}

////////////////////////////////////////////////////////////////////////////////
// Inner Struct
////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, Default, Debug, PartialEq)]
pub struct WhirlpoolRewardInfo {
  pub mint: Pubkey,
  pub vault: Pubkey,
  pub authority: Pubkey,
  pub emissions_per_second_x64: u128,
  pub growth_global_x64: u128,
}

#[zero_copy(unsafe)]
#[repr(C, packed)]
#[derive(Default, Debug, PartialEq)]
pub struct Tick {
  pub initialized: bool,
  pub liquidity_net: i128,
  pub liquidity_gross: u128,
  pub fee_growth_outside_a: u128,
  pub fee_growth_outside_b: u128,
  pub reward_growths_outside: [u128; NUM_REWARDS],
}

impl Tick {
  pub const LEN: usize = 113;
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, Default, Debug, PartialEq)]
pub struct PositionRewardInfo {
  pub growth_inside_checkpoint: u128,
  pub amount_owed: u64,
}

////////////////////////////////////////////////////////////////////////////////
// Bump
////////////////////////////////////////////////////////////////////////////////
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct WhirlpoolBumps {
  pub whirlpool_bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct OpenPositionBumps {
  pub position_bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct OpenPositionWithMetadataBumps {
  pub position_bump: u8,
  pub metadata_bump: u8,
}

////////////////////////////////////////////////////////////////////////////////
// Remaining Accounts
////////////////////////////////////////////////////////////////////////////////
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}
