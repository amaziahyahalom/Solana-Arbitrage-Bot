use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;

use crate::state::*;

////////////////////////////////////////////////////////////////////////////////
// Context
////////////////////////////////////////////////////////////////////////////////
// to avoid depending on spl-token, we use a placeholder account
#[account]
pub struct AccountPlaceholder {}

#[derive(Accounts)]
pub struct CloseBundledPosition<'info> {
    #[account(mut)]
    pub bundled_position: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_bundle: Account<'info, AccountPlaceholder>,

    pub position_bundle_token_account: Account<'info, AccountPlaceholder>,

    pub position_bundle_authority: Signer<'info>,

    #[account(mut)]
    pub receiver: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    pub position_authority: Signer<'info>,

    #[account(mut)]
    pub receiver: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_mint: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_token_account: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct CollectFees<'info> {
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub position_authority: Signer<'info>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,

    pub position_token_account: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_b: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_b: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct CollectProtocolFees<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub collect_protocol_fees_authority: Signer<'info>,

    #[account(mut)]
    pub token_vault_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_vault_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_destination_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_destination_b: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct CollectReward<'info> {
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub position_authority: Signer<'info>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,
    pub position_token_account: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub reward_owner_account: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub reward_vault: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct DeletePositionBundle<'info> {
    #[account(mut)]
    pub position_bundle: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_bundle_mint: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_bundle_token_account: Account<'info, AccountPlaceholder>,

    pub position_bundle_owner: Signer<'info>,

    #[account(mut)]
    pub receiver: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct ModifyLiquidity<'info> {
    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,

    pub position_authority: Signer<'info>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,
    pub position_token_account: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_owner_account_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_vault_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_lower: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub tick_array_upper: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub config: Signer<'info>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub system_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(tick_spacing: u16)]
pub struct InitializeFeeTier<'info> {
    pub config: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub fee_tier: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub fee_authority: Signer<'info>,

    pub system_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(tick_spacing: u16)]
pub struct InitializePool<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    pub token_mint_a: Account<'info, AccountPlaceholder>,
    pub token_mint_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_vault_a: Signer<'info>,

    #[account(mut)]
    pub token_vault_b: Signer<'info>,

    pub fee_tier: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,
    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct InitializePositionBundle<'info> {
    #[account(mut)]
    pub position_bundle: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_bundle_mint: Signer<'info>,

    #[account(mut)]
    pub position_bundle_token_account: Account<'info, AccountPlaceholder>,

    pub position_bundle_owner: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub token_program: Account<'info, AccountPlaceholder>,
    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
    pub associated_token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct InitializePositionBundleWithMetadata<'info> {
    #[account(mut)]
    pub position_bundle: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_bundle_mint: Signer<'info>,

    #[account(mut)]
    pub position_bundle_metadata: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_bundle_token_account: Account<'info, AccountPlaceholder>,

    pub position_bundle_owner: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub metadata_update_auth: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,
    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
    pub associated_token_program: Account<'info, AccountPlaceholder>,

    pub metadata_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct InitializeReward<'info> {
    pub reward_authority: Signer<'info>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub reward_mint: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub reward_vault: Signer<'info>,

    pub token_program: Account<'info, AccountPlaceholder>,
    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct InitializeTickArray<'info> {
    pub whirlpool: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub tick_array: Account<'info, AccountPlaceholder>,

    pub system_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(bundle_index: u16)]
pub struct OpenBundledPosition<'info> {
    #[account(mut)]
    pub bundled_position: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_bundle: Account<'info, AccountPlaceholder>,

    pub position_bundle_token_account: Account<'info, AccountPlaceholder>,

    pub position_bundle_authority: Signer<'info>,

    pub whirlpool: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    pub owner: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_mint: Signer<'info>,

    #[account(mut)]
    pub position_token_account: Account<'info, AccountPlaceholder>,

    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,
    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
    pub associated_token_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct OpenPositionWithMetadata<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    pub owner: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_mint: Signer<'info>,

    #[account(mut)]
    pub position_metadata_account: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position_token_account: Account<'info, AccountPlaceholder>,

    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub token_program: Account<'info, AccountPlaceholder>,
    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
    pub associated_token_program: Account<'info, AccountPlaceholder>,

    pub metadata_program: Account<'info, AccountPlaceholder>,

    pub metadata_update_auth: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SetCollectProtocolFeesAuthority<'info> {
    #[account(mut)]
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    pub collect_protocol_fees_authority: Signer<'info>,

    pub new_collect_protocol_fees_authority: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SetDefaultFeeRate<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub fee_tier: Account<'info, AccountPlaceholder>,

    pub fee_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetDefaultProtocolFeeRate<'info> {
    #[account(mut)]
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    pub fee_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetFeeAuthority<'info> {
    #[account(mut)]
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    pub fee_authority: Signer<'info>,

    pub new_fee_authority: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SetFeeRate<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub fee_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetProtocolFeeRate<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub fee_authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct SetRewardAuthority<'info> {
    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub reward_authority: Signer<'info>,

    pub new_reward_authority: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct SetRewardAuthorityBySuperAuthority<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub reward_emissions_super_authority: Signer<'info>,

    pub new_reward_authority: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct SetRewardEmissions<'info> {
    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub reward_authority: Signer<'info>,

    pub reward_vault: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SetRewardEmissionsSuperAuthority<'info> {
    #[account(mut)]
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    pub reward_emissions_super_authority: Signer<'info>,

    pub new_reward_emissions_super_authority: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub token_program: Account<'info, AccountPlaceholder>,

    pub token_authority: Signer<'info>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_b: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_0: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_1: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_2: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub oracle: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct TwoHopSwap<'info> {
    pub token_program: Account<'info, AccountPlaceholder>,

    pub token_authority: Signer<'info>,

    #[account(mut)]
    pub whirlpool_one: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub whirlpool_two: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_one_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_one_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_one_b: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_one_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_two_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_two_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_two_b: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_two_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_one_0: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_one_1: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_one_2: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_two_0: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_two_1: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_two_2: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub oracle_one: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub oracle_two: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct UpdateFeesAndRewards<'info> {
    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,

    pub tick_array_lower: Account<'info, AccountPlaceholder>,
    pub tick_array_upper: Account<'info, AccountPlaceholder>,
}
////////////////////////////////////////////////////////////////////////////////
// V2 Context
////////////////////////////////////////////////////////////////////////////////
#[derive(Accounts)]
pub struct CollectFeesV2<'info> {
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub position_authority: Signer<'info>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,
    pub position_token_account: Account<'info, AccountPlaceholder>,

    pub token_mint_a: Account<'info, AccountPlaceholder>,
    pub token_mint_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_b: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_b: Account<'info, AccountPlaceholder>,

    pub token_program_a: Account<'info, AccountPlaceholder>,
    pub token_program_b: Account<'info, AccountPlaceholder>,
    pub memo_program: Account<'info, AccountPlaceholder>,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
}

#[derive(Accounts)]
pub struct CollectProtocolFeesV2<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub collect_protocol_fees_authority: Signer<'info>,

    pub token_mint_a: Account<'info, AccountPlaceholder>,
    pub token_mint_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_vault_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_vault_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_destination_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_destination_b: Account<'info, AccountPlaceholder>,

    pub token_program_a: Account<'info, AccountPlaceholder>,
    pub token_program_b: Account<'info, AccountPlaceholder>,
    pub memo_program: Account<'info, AccountPlaceholder>,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
}

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct CollectRewardV2<'info> {
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub position_authority: Signer<'info>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,
    pub position_token_account: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub reward_owner_account: Account<'info, AccountPlaceholder>,

    pub reward_mint: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub reward_vault: Account<'info, AccountPlaceholder>,

    pub reward_token_program: Account<'info, AccountPlaceholder>,
    pub memo_program: Account<'info, AccountPlaceholder>,
    // remaining accounts
    // - accounts for transfer hook program of reward_mint
}

#[derive(Accounts)]
pub struct DeleteTokenBadge<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    pub whirlpools_config_extension: Account<'info, AccountPlaceholder>,

    pub token_badge_authority: Signer<'info>,

    pub token_mint: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_badge: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub receiver: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct ModifyLiquidityV2<'info> {
    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub token_program_a: Account<'info, AccountPlaceholder>,
    pub token_program_b: Account<'info, AccountPlaceholder>,

    pub memo_program: Account<'info, AccountPlaceholder>,

    pub position_authority: Signer<'info>,

    #[account(mut)]
    pub position: Account<'info, AccountPlaceholder>,
    pub position_token_account: Account<'info, AccountPlaceholder>,

    pub token_mint_a: Account<'info, AccountPlaceholder>,
    pub token_mint_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_owner_account_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_vault_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_lower: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub tick_array_upper: Account<'info, AccountPlaceholder>,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
}

#[derive(Accounts)]
pub struct InitializeConfigExtension<'info> {
    pub config: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub config_extension: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub fee_authority: Signer<'info>,

    pub system_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(tick_spacing: u16)]
pub struct InitializePoolV2<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    pub token_mint_a: Account<'info, AccountPlaceholder>,
    pub token_mint_b: Account<'info, AccountPlaceholder>,

    pub token_badge_a: Account<'info, AccountPlaceholder>,
    pub token_badge_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_vault_a: Signer<'info>,

    #[account(mut)]
    pub token_vault_b: Signer<'info>,

    pub fee_tier: Account<'info, AccountPlaceholder>,

    pub token_program_a: Account<'info, AccountPlaceholder>,
    pub token_program_b: Account<'info, AccountPlaceholder>,
    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct InitializeRewardV2<'info> {
    pub reward_authority: Signer<'info>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub reward_mint: Account<'info, AccountPlaceholder>,

    pub reward_token_badge: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub reward_vault: Signer<'info>,

    pub reward_token_program: Account<'info, AccountPlaceholder>,
    pub system_program: Account<'info, AccountPlaceholder>,
    pub rent: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct InitializeTokenBadge<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    pub whirlpools_config_extension: Account<'info, AccountPlaceholder>,

    pub token_badge_authority: Signer<'info>,

    pub token_mint: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_badge: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub system_program: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SetConfigExtensionAuthority<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub whirlpools_config_extension: Account<'info, AccountPlaceholder>,

    pub config_extension_authority: Signer<'info>,

    pub new_config_extension_authority: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct SetRewardEmissionsV2<'info> {
    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub reward_authority: Signer<'info>,

    pub reward_vault: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SetTokenBadgeAuthority<'info> {
    pub whirlpools_config: Account<'info, AccountPlaceholder>,

    pub whirlpools_config_extension: Account<'info, AccountPlaceholder>,

    pub config_extension_authority: Signer<'info>,

    pub new_token_badge_authority: Account<'info, AccountPlaceholder>,
}

#[derive(Accounts)]
pub struct SwapV2<'info> {
    pub token_program_a: Account<'info, AccountPlaceholder>,
    pub token_program_b: Account<'info, AccountPlaceholder>,

    pub memo_program: Account<'info, AccountPlaceholder>,

    pub token_authority: Signer<'info>,

    #[account(mut)]
    pub whirlpool: Account<'info, AccountPlaceholder>,

    pub token_mint_a: Account<'info, AccountPlaceholder>,
    pub token_mint_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_a: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_a: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_b: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_b: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_0: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_1: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_2: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub oracle: Account<'info, AccountPlaceholder>,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
    // - supplemental TickArray accounts
}

#[derive(Accounts)]
#[instruction(
    amount: u64,
    other_amount_threshold: u64,
    amount_specified_is_input: bool,
    a_to_b_one: bool,
    a_to_b_two: bool,
)]
pub struct TwoHopSwapV2<'info> {
    #[account(mut)]
    pub whirlpool_one: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub whirlpool_two: Account<'info, AccountPlaceholder>,

    pub token_mint_input: Account<'info, AccountPlaceholder>,
    pub token_mint_intermediate: Account<'info, AccountPlaceholder>,
    pub token_mint_output: Account<'info, AccountPlaceholder>,

    pub token_program_input: Account<'info, AccountPlaceholder>,
    pub token_program_intermediate: Account<'info, AccountPlaceholder>,
    pub token_program_output: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_owner_account_input: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_one_input: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_one_intermediate: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub token_vault_two_intermediate: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_vault_two_output: Account<'info, AccountPlaceholder>,
    #[account(mut)]
    pub token_owner_account_output: Account<'info, AccountPlaceholder>,

    pub token_authority: Signer<'info>,

    #[account(mut)]
    pub tick_array_one_0: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_one_1: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_one_2: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_two_0: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_two_1: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub tick_array_two_2: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub oracle_one: Account<'info, AccountPlaceholder>,

    #[account(mut)]
    pub oracle_two: Account<'info, AccountPlaceholder>,

    pub memo_program: Account<'info, AccountPlaceholder>,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_input
    // - accounts for transfer hook program of token_mint_intermediate
    // - accounts for transfer hook program of token_mint_output
    // - supplemental TickArray accounts for whirlpool_one
    // - supplemental TickArray accounts for whirlpool_two
}
