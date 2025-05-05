use super::WhirlpoolDecoder;
pub mod close_bundled_position;
pub mod close_position;
pub mod close_position_with_token_extensions;
pub mod collect_fees;
pub mod collect_fees_v2;
pub mod collect_protocol_fees;
pub mod collect_protocol_fees_v2;
pub mod collect_reward;
pub mod collect_reward_v2;
pub mod decrease_liquidity;
pub mod decrease_liquidity_v2;
pub mod delete_position_bundle;
pub mod delete_token_badge;
pub mod increase_liquidity;
pub mod increase_liquidity_v2;
pub mod initialize_config;
pub mod initialize_config_extension;
pub mod initialize_fee_tier;
pub mod initialize_pool;
pub mod initialize_pool_v2;
pub mod initialize_position_bundle;
pub mod initialize_position_bundle_with_metadata;
pub mod initialize_reward;
pub mod initialize_reward_v2;
pub mod initialize_tick_array;
pub mod initialize_token_badge;
pub mod liquidity_decreased_event;
pub mod liquidity_increased_event;
pub mod lock_position;
pub mod open_bundled_position;
pub mod open_position;
pub mod open_position_with_metadata;
pub mod open_position_with_token_extensions;
pub mod pool_initialized_event;
pub mod set_collect_protocol_fees_authority;
pub mod set_config_extension_authority;
pub mod set_default_fee_rate;
pub mod set_default_protocol_fee_rate;
pub mod set_fee_authority;
pub mod set_fee_rate;
pub mod set_protocol_fee_rate;
pub mod set_reward_authority;
pub mod set_reward_authority_by_super_authority;
pub mod set_reward_emissions;
pub mod set_reward_emissions_super_authority;
pub mod set_reward_emissions_v2;
pub mod set_token_badge_authority;
pub mod swap;
pub mod swap_v2;
pub mod traded_event;
pub mod two_hop_swap;
pub mod two_hop_swap_v2;
pub mod update_fees_and_rewards;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum WhirlpoolInstruction {
    InitializeConfig(initialize_config::InitializeConfig),
    InitializePool(initialize_pool::InitializePool),
    InitializeTickArray(initialize_tick_array::InitializeTickArray),
    InitializeFeeTier(initialize_fee_tier::InitializeFeeTier),
    InitializeReward(initialize_reward::InitializeReward),
    SetRewardEmissions(set_reward_emissions::SetRewardEmissions),
    OpenPosition(open_position::OpenPosition),
    OpenPositionWithMetadata(open_position_with_metadata::OpenPositionWithMetadata),
    IncreaseLiquidity(increase_liquidity::IncreaseLiquidity),
    DecreaseLiquidity(decrease_liquidity::DecreaseLiquidity),
    UpdateFeesAndRewards(update_fees_and_rewards::UpdateFeesAndRewards),
    CollectFees(collect_fees::CollectFees),
    CollectReward(collect_reward::CollectReward),
    CollectProtocolFees(collect_protocol_fees::CollectProtocolFees),
    Swap(swap::Swap),
    ClosePosition(close_position::ClosePosition),
    SetDefaultFeeRate(set_default_fee_rate::SetDefaultFeeRate),
    SetDefaultProtocolFeeRate(set_default_protocol_fee_rate::SetDefaultProtocolFeeRate),
    SetFeeRate(set_fee_rate::SetFeeRate),
    SetProtocolFeeRate(set_protocol_fee_rate::SetProtocolFeeRate),
    SetFeeAuthority(set_fee_authority::SetFeeAuthority),
    SetCollectProtocolFeesAuthority(
        set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority,
    ),
    SetRewardAuthority(set_reward_authority::SetRewardAuthority),
    SetRewardAuthorityBySuperAuthority(
        set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority,
    ),
    SetRewardEmissionsSuperAuthority(
        set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority,
    ),
    TwoHopSwap(two_hop_swap::TwoHopSwap),
    InitializePositionBundle(initialize_position_bundle::InitializePositionBundle),
    InitializePositionBundleWithMetadata(
        initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata,
    ),
    DeletePositionBundle(delete_position_bundle::DeletePositionBundle),
    OpenBundledPosition(open_bundled_position::OpenBundledPosition),
    CloseBundledPosition(close_bundled_position::CloseBundledPosition),
    OpenPositionWithTokenExtensions(
        open_position_with_token_extensions::OpenPositionWithTokenExtensions,
    ),
    ClosePositionWithTokenExtensions(
        close_position_with_token_extensions::ClosePositionWithTokenExtensions,
    ),
    LockPosition(lock_position::LockPosition),
    CollectFeesV2(collect_fees_v2::CollectFeesV2),
    CollectProtocolFeesV2(collect_protocol_fees_v2::CollectProtocolFeesV2),
    CollectRewardV2(collect_reward_v2::CollectRewardV2),
    DecreaseLiquidityV2(decrease_liquidity_v2::DecreaseLiquidityV2),
    IncreaseLiquidityV2(increase_liquidity_v2::IncreaseLiquidityV2),
    InitializePoolV2(initialize_pool_v2::InitializePoolV2),
    InitializeRewardV2(initialize_reward_v2::InitializeRewardV2),
    SetRewardEmissionsV2(set_reward_emissions_v2::SetRewardEmissionsV2),
    SwapV2(swap_v2::SwapV2),
    TwoHopSwapV2(two_hop_swap_v2::TwoHopSwapV2),
    InitializeConfigExtension(initialize_config_extension::InitializeConfigExtension),
    SetConfigExtensionAuthority(set_config_extension_authority::SetConfigExtensionAuthority),
    SetTokenBadgeAuthority(set_token_badge_authority::SetTokenBadgeAuthority),
    InitializeTokenBadge(initialize_token_badge::InitializeTokenBadge),
    DeleteTokenBadge(delete_token_badge::DeleteTokenBadge),
    LiquidityDecreasedEvent(liquidity_decreased_event::LiquidityDecreasedEvent),
    LiquidityIncreasedEvent(liquidity_increased_event::LiquidityIncreasedEvent),
    PoolInitializedEvent(pool_initialized_event::PoolInitializedEvent),
    TradedEvent(traded_event::TradedEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for WhirlpoolDecoder {
    type InstructionType = WhirlpoolInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            WhirlpoolInstruction::InitializeConfig => initialize_config::InitializeConfig,
            WhirlpoolInstruction::InitializePool => initialize_pool::InitializePool,
            WhirlpoolInstruction::InitializeTickArray => initialize_tick_array::InitializeTickArray,
            WhirlpoolInstruction::InitializeFeeTier => initialize_fee_tier::InitializeFeeTier,
            WhirlpoolInstruction::InitializeReward => initialize_reward::InitializeReward,
            WhirlpoolInstruction::SetRewardEmissions => set_reward_emissions::SetRewardEmissions,
            WhirlpoolInstruction::OpenPosition => open_position::OpenPosition,
            WhirlpoolInstruction::OpenPositionWithMetadata => open_position_with_metadata::OpenPositionWithMetadata,
            WhirlpoolInstruction::IncreaseLiquidity => increase_liquidity::IncreaseLiquidity,
            WhirlpoolInstruction::DecreaseLiquidity => decrease_liquidity::DecreaseLiquidity,
            WhirlpoolInstruction::UpdateFeesAndRewards => update_fees_and_rewards::UpdateFeesAndRewards,
            WhirlpoolInstruction::CollectFees => collect_fees::CollectFees,
            WhirlpoolInstruction::CollectReward => collect_reward::CollectReward,
            WhirlpoolInstruction::CollectProtocolFees => collect_protocol_fees::CollectProtocolFees,
            WhirlpoolInstruction::Swap => swap::Swap,
            WhirlpoolInstruction::ClosePosition => close_position::ClosePosition,
            WhirlpoolInstruction::SetDefaultFeeRate => set_default_fee_rate::SetDefaultFeeRate,
            WhirlpoolInstruction::SetDefaultProtocolFeeRate => set_default_protocol_fee_rate::SetDefaultProtocolFeeRate,
            WhirlpoolInstruction::SetFeeRate => set_fee_rate::SetFeeRate,
            WhirlpoolInstruction::SetProtocolFeeRate => set_protocol_fee_rate::SetProtocolFeeRate,
            WhirlpoolInstruction::SetFeeAuthority => set_fee_authority::SetFeeAuthority,
            WhirlpoolInstruction::SetCollectProtocolFeesAuthority => set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority,
            WhirlpoolInstruction::SetRewardAuthority => set_reward_authority::SetRewardAuthority,
            WhirlpoolInstruction::SetRewardAuthorityBySuperAuthority => set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority,
            WhirlpoolInstruction::SetRewardEmissionsSuperAuthority => set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority,
            WhirlpoolInstruction::TwoHopSwap => two_hop_swap::TwoHopSwap,
            WhirlpoolInstruction::InitializePositionBundle => initialize_position_bundle::InitializePositionBundle,
            WhirlpoolInstruction::InitializePositionBundleWithMetadata => initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata,
            WhirlpoolInstruction::DeletePositionBundle => delete_position_bundle::DeletePositionBundle,
            WhirlpoolInstruction::OpenBundledPosition => open_bundled_position::OpenBundledPosition,
            WhirlpoolInstruction::CloseBundledPosition => close_bundled_position::CloseBundledPosition,
            WhirlpoolInstruction::OpenPositionWithTokenExtensions => open_position_with_token_extensions::OpenPositionWithTokenExtensions,
            WhirlpoolInstruction::ClosePositionWithTokenExtensions => close_position_with_token_extensions::ClosePositionWithTokenExtensions,
            WhirlpoolInstruction::LockPosition => lock_position::LockPosition,
            WhirlpoolInstruction::CollectFeesV2 => collect_fees_v2::CollectFeesV2,
            WhirlpoolInstruction::CollectProtocolFeesV2 => collect_protocol_fees_v2::CollectProtocolFeesV2,
            WhirlpoolInstruction::CollectRewardV2 => collect_reward_v2::CollectRewardV2,
            WhirlpoolInstruction::DecreaseLiquidityV2 => decrease_liquidity_v2::DecreaseLiquidityV2,
            WhirlpoolInstruction::IncreaseLiquidityV2 => increase_liquidity_v2::IncreaseLiquidityV2,
            WhirlpoolInstruction::InitializePoolV2 => initialize_pool_v2::InitializePoolV2,
            WhirlpoolInstruction::InitializeRewardV2 => initialize_reward_v2::InitializeRewardV2,
            WhirlpoolInstruction::SetRewardEmissionsV2 => set_reward_emissions_v2::SetRewardEmissionsV2,
            WhirlpoolInstruction::SwapV2 => swap_v2::SwapV2,
            WhirlpoolInstruction::TwoHopSwapV2 => two_hop_swap_v2::TwoHopSwapV2,
            WhirlpoolInstruction::InitializeConfigExtension => initialize_config_extension::InitializeConfigExtension,
            WhirlpoolInstruction::SetConfigExtensionAuthority => set_config_extension_authority::SetConfigExtensionAuthority,
            WhirlpoolInstruction::SetTokenBadgeAuthority => set_token_badge_authority::SetTokenBadgeAuthority,
            WhirlpoolInstruction::InitializeTokenBadge => initialize_token_badge::InitializeTokenBadge,
            WhirlpoolInstruction::DeleteTokenBadge => delete_token_badge::DeleteTokenBadge,
            WhirlpoolInstruction::LiquidityDecreasedEvent => liquidity_decreased_event::LiquidityDecreasedEvent,
            WhirlpoolInstruction::LiquidityIncreasedEvent => liquidity_increased_event::LiquidityIncreasedEvent,
            WhirlpoolInstruction::PoolInitializedEvent => pool_initialized_event::PoolInitializedEvent,
            WhirlpoolInstruction::TradedEvent => traded_event::TradedEvent,
        )
    }
}
