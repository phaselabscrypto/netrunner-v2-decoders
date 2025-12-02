use super::FlashTradePerpsV2Decoder;
pub mod add_collateral;
pub mod add_collateral_log_event;
pub mod add_collateral_log_us_dv1_event;
pub mod add_collateral_log_v2_event;
pub mod add_collateral_log_v3_event;
pub mod add_compounding_liquidity;
pub mod add_compounding_liquidity_log_event;
pub mod add_custody;
pub mod add_custody_token22_account;
pub mod add_internal_oracle;
pub mod add_liquidity;
pub mod add_liquidity_and_stake;
pub mod add_liquidity_and_stake_log_event;
pub mod add_liquidity_log_event;
pub mod add_liquidity_log_v2_event;
pub mod add_market;
pub mod add_pool;
pub mod burn_and_claim_log_event;
pub mod burn_and_stake_log_event;
pub mod cancel_all_trigger_orders;
pub mod cancel_limit_order_log_event;
pub mod cancel_trigger_order;
pub mod cancel_trigger_order_log_event;
pub mod cancel_unstake_token_request;
pub mod cancel_unstake_token_request_log_event;
pub mod close_and_swap;
pub mod close_and_swap_log_event;
pub mod close_and_swap_log_us_dv1_event;
pub mod close_position;
pub mod close_position_log_event;
pub mod close_position_log_us_dv1_event;
pub mod close_position_log_v2_event;
pub mod close_position_log_v3_event;
pub mod collect_rebate;
pub mod collect_rebate_log_event;
pub mod collect_revenue;
pub mod collect_revenue_log_event;
pub mod collect_stake_fees;
pub mod collect_stake_reward_log_event;
pub mod collect_stake_reward_log_v2_event;
pub mod collect_token_reward;
pub mod collect_token_reward_log_event;
pub mod compound_fees;
pub mod compounding_fees_log_event;
pub mod create_referral;
pub mod decrease_size;
pub mod decrease_size_log_event;
pub mod decrease_size_log_us_dv1_event;
pub mod decrease_size_log_v2_event;
pub mod decrease_size_log_v3_event;
pub mod deposit_stake;
pub mod deposit_stake_log_event;
pub mod deposit_token_stake;
pub mod deposit_token_stake_log_event;
pub mod distribute_token_reward;
pub mod distribute_token_reward_log_event;
pub mod edit_limit_order;
pub mod edit_limit_order_log_event;
pub mod edit_trigger_order;
pub mod edit_trigger_order_log_event;
pub mod execute_limit_order;
pub mod execute_limit_order_log_event;
pub mod execute_limit_order_log_us_dv1_event;
pub mod execute_limit_order_log_v2_event;
pub mod execute_limit_with_swap;
pub mod execute_limit_with_swap_log_event;
pub mod execute_limit_with_swap_log_us_dv1_event;
pub mod execute_limit_with_swap_log_v2_event;
pub mod execute_trigger_order;
pub mod execute_trigger_order_log_event;
pub mod execute_trigger_order_log_us_dv1_event;
pub mod execute_trigger_with_swap;
pub mod execute_trigger_with_swap_log_event;
pub mod execute_trigger_with_swap_log_us_dv1_event;
pub mod get_add_compounding_liquidity_amount_and_fee;
pub mod get_add_liquidity_amount_and_fee;
pub mod get_assets_under_management;
pub mod get_compounding_token_data;
pub mod get_compounding_token_price;
pub mod get_entry_price_and_fee;
pub mod get_exit_price_and_fee;
pub mod get_liquidation_price;
pub mod get_liquidation_state;
pub mod get_lp_token_price;
pub mod get_oracle_price;
pub mod get_pnl;
pub mod get_position_data;
pub mod get_remove_compounding_liquidity_amount_and_fee;
pub mod get_remove_liquidity_amount_and_fee;
pub mod get_swap_amount_and_fees;
pub mod increase_size;
pub mod increase_size_log_event;
pub mod increase_size_log_us_dv1_event;
pub mod increase_size_log_v2_event;
pub mod increase_size_log_v3_event;
pub mod increase_size_log_v4_event;
pub mod init;
pub mod init_compounding;
pub mod init_rebate_vault;
pub mod init_revenue_token_account;
pub mod init_staking;
pub mod init_token_vault;
pub mod liquidate;
pub mod liquidate_log_event;
pub mod liquidate_log_us_dv1_event;
pub mod liquidate_log_v2_event;
pub mod liquidate_log_v3_event;
pub mod migrate_flp;
pub mod migrate_flp_log_event;
pub mod migrate_position_log_event;
pub mod migrate_stake;
pub mod migrate_stake_log_event;
pub mod move_protocol_fees;
pub mod move_protocol_fees_log_event;
pub mod open_position;
pub mod open_position_log_event;
pub mod open_position_log_us_dv1_event;
pub mod open_position_log_v2_event;
pub mod open_position_log_v3_event;
pub mod open_position_log_v4_event;
pub mod place_limit_order;
pub mod place_limit_order_log_event;
pub mod place_trigger_order;
pub mod place_trigger_order_log_event;
pub mod referral_rebate_log_event;
pub mod refresh_stake;
pub mod refresh_stake_log_event;
pub mod refresh_stake_user_log_event;
pub mod reimburse;
pub mod remove_collateral;
pub mod remove_collateral_and_swap;
pub mod remove_collateral_and_swap_log_event;
pub mod remove_collateral_and_swap_log_us_dv1_event;
pub mod remove_collateral_log_event;
pub mod remove_collateral_log_us_dv1_event;
pub mod remove_collateral_log_v2_event;
pub mod remove_collateral_log_v3_event;
pub mod remove_compounding_liquidity;
pub mod remove_compounding_liquidity_log_event;
pub mod remove_custody;
pub mod remove_liquidity;
pub mod remove_liquidity_log_event;
pub mod remove_liquidity_log_v2_event;
pub mod remove_market;
pub mod remove_pool;
pub mod rename_flp;
pub mod resize_internal_oracle;
pub mod set_admin_signers;
pub mod set_custody_config;
pub mod set_custom_oracle_price;
pub mod set_fee_share;
pub mod set_flp_stake_config;
pub mod set_internal_current_price;
pub mod set_internal_ema_price;
pub mod set_internal_oracle_price;
pub mod set_lp_token_price;
pub mod set_market_config;
pub mod set_permissions;
pub mod set_perpetuals_config;
pub mod set_pool_config;
pub mod set_protocol_fee_share;
pub mod set_test_time;
pub mod set_token_reward;
pub mod set_token_reward_log_event;
pub mod set_token_stake_level;
pub mod set_token_vault_config;
pub mod settle_rebates;
pub mod settle_rebates_log_event;
pub mod swap;
pub mod swap_and_add_collateral;
pub mod swap_and_add_collateral_log_event;
pub mod swap_and_add_collateral_log_us_dv1_event;
pub mod swap_and_open;
pub mod swap_and_open_log_event;
pub mod swap_and_open_log_us_dv1_event;
pub mod swap_and_open_log_v2_event;
pub mod swap_fee_internal;
pub mod swap_fee_internal_log_event;
pub mod swap_fee_internal_log_v2_event;
pub mod swap_fee_internal_log_v3_event;
pub mod swap_log_event;
pub mod swap_log_v2_event;
pub mod test_init;
pub mod unstake_instant;
pub mod unstake_instant_log_event;
pub mod unstake_request;
pub mod unstake_request_log_event;
pub mod unstake_token_instant;
pub mod unstake_token_instant_log_event;
pub mod unstake_token_request;
pub mod unstake_token_request_log_event;
pub mod update_custody;
pub mod update_token_ratios;
pub mod voltage_points_log_event;
pub mod withdraw_fees;
pub mod withdraw_instant_fees;
pub mod withdraw_sol_fees;
pub mod withdraw_stake;
pub mod withdraw_stake_log_event;
pub mod withdraw_token;
pub mod withdraw_token_log_event;
pub mod withdraw_unclaimed_tokens;

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
pub enum FlashTradePerpsV2Instruction {
    Init(init::Init),
    AddPool(add_pool::AddPool),
    RemovePool(remove_pool::RemovePool),
    AddCustody(add_custody::AddCustody),
    AddCustodyToken22Account(add_custody_token22_account::AddCustodyToken22Account),
    AddInternalOracle(add_internal_oracle::AddInternalOracle),
    RemoveCustody(remove_custody::RemoveCustody),
    AddMarket(add_market::AddMarket),
    RemoveMarket(remove_market::RemoveMarket),
    Reimburse(reimburse::Reimburse),
    ResizeInternalOracle(resize_internal_oracle::ResizeInternalOracle),
    SetAdminSigners(set_admin_signers::SetAdminSigners),
    SetCustodyConfig(set_custody_config::SetCustodyConfig),
    SetPerpetualsConfig(set_perpetuals_config::SetPerpetualsConfig),
    SetPermissions(set_permissions::SetPermissions),
    SetPoolConfig(set_pool_config::SetPoolConfig),
    SetProtocolFeeShare(set_protocol_fee_share::SetProtocolFeeShare),
    SetMarketConfig(set_market_config::SetMarketConfig),
    SetFlpStakeConfig(set_flp_stake_config::SetFlpStakeConfig),
    SetTokenReward(set_token_reward::SetTokenReward),
    SetTokenStakeLevel(set_token_stake_level::SetTokenStakeLevel),
    SetTokenVaultConfig(set_token_vault_config::SetTokenVaultConfig),
    WithdrawFees(withdraw_fees::WithdrawFees),
    WithdrawInstantFees(withdraw_instant_fees::WithdrawInstantFees),
    WithdrawUnclaimedTokens(withdraw_unclaimed_tokens::WithdrawUnclaimedTokens),
    WithdrawSolFees(withdraw_sol_fees::WithdrawSolFees),
    UpdateCustody(update_custody::UpdateCustody),
    UpdateTokenRatios(update_token_ratios::UpdateTokenRatios),
    InitStaking(init_staking::InitStaking),
    InitCompounding(init_compounding::InitCompounding),
    InitRebateVault(init_rebate_vault::InitRebateVault),
    InitRevenueTokenAccount(init_revenue_token_account::InitRevenueTokenAccount),
    InitTokenVault(init_token_vault::InitTokenVault),
    SetCustomOraclePrice(set_custom_oracle_price::SetCustomOraclePrice),
    SetInternalCurrentPrice(set_internal_current_price::SetInternalCurrentPrice),
    SetInternalEmaPrice(set_internal_ema_price::SetInternalEmaPrice),
    SetInternalOraclePrice(set_internal_oracle_price::SetInternalOraclePrice),
    SetLpTokenPrice(set_lp_token_price::SetLpTokenPrice),
    SetFeeShare(set_fee_share::SetFeeShare),
    TestInit(test_init::TestInit),
    SetTestTime(set_test_time::SetTestTime),
    SwapFeeInternal(swap_fee_internal::SwapFeeInternal),
    Swap(swap::Swap),
    SettleRebates(settle_rebates::SettleRebates),
    SwapAndAddCollateral(swap_and_add_collateral::SwapAndAddCollateral),
    SwapAndOpen(swap_and_open::SwapAndOpen),
    CloseAndSwap(close_and_swap::CloseAndSwap),
    AddLiquidityAndStake(add_liquidity_and_stake::AddLiquidityAndStake),
    AddLiquidity(add_liquidity::AddLiquidity),
    AddCompoundingLiquidity(add_compounding_liquidity::AddCompoundingLiquidity),
    RemoveLiquidity(remove_liquidity::RemoveLiquidity),
    DepositTokenStake(deposit_token_stake::DepositTokenStake),
    DistributeTokenReward(distribute_token_reward::DistributeTokenReward),
    DepositStake(deposit_stake::DepositStake),
    RefreshStake(refresh_stake::RefreshStake),
    UnstakeInstant(unstake_instant::UnstakeInstant),
    WithdrawStake(withdraw_stake::WithdrawStake),
    WithdrawToken(withdraw_token::WithdrawToken),
    CollectRebate(collect_rebate::CollectRebate),
    CollectRevenue(collect_revenue::CollectRevenue),
    CollectStakeFees(collect_stake_fees::CollectStakeFees),
    CollectTokenReward(collect_token_reward::CollectTokenReward),
    UnstakeRequest(unstake_request::UnstakeRequest),
    UnstakeTokenInstant(unstake_token_instant::UnstakeTokenInstant),
    UnstakeTokenRequest(unstake_token_request::UnstakeTokenRequest),
    MigrateFlp(migrate_flp::MigrateFlp),
    MigrateStake(migrate_stake::MigrateStake),
    MoveProtocolFees(move_protocol_fees::MoveProtocolFees),
    CompoundFees(compound_fees::CompoundFees),
    RemoveCompoundingLiquidity(remove_compounding_liquidity::RemoveCompoundingLiquidity),
    CreateReferral(create_referral::CreateReferral),
    OpenPosition(open_position::OpenPosition),
    AddCollateral(add_collateral::AddCollateral),
    RemoveCollateralAndSwap(remove_collateral_and_swap::RemoveCollateralAndSwap),
    RemoveCollateral(remove_collateral::RemoveCollateral),
    IncreaseSize(increase_size::IncreaseSize),
    DecreaseSize(decrease_size::DecreaseSize),
    CancelAllTriggerOrders(cancel_all_trigger_orders::CancelAllTriggerOrders),
    CancelTriggerOrder(cancel_trigger_order::CancelTriggerOrder),
    CancelUnstakeTokenRequest(cancel_unstake_token_request::CancelUnstakeTokenRequest),
    ClosePosition(close_position::ClosePosition),
    ExecuteLimitWithSwap(execute_limit_with_swap::ExecuteLimitWithSwap),
    ExecuteLimitOrder(execute_limit_order::ExecuteLimitOrder),
    PlaceLimitOrder(place_limit_order::PlaceLimitOrder),
    EditLimitOrder(edit_limit_order::EditLimitOrder),
    EditTriggerOrder(edit_trigger_order::EditTriggerOrder),
    PlaceTriggerOrder(place_trigger_order::PlaceTriggerOrder),
    ExecuteTriggerWithSwap(execute_trigger_with_swap::ExecuteTriggerWithSwap),
    ExecuteTriggerOrder(execute_trigger_order::ExecuteTriggerOrder),
    Liquidate(liquidate::Liquidate),
    GetAddCompoundingLiquidityAmountAndFee(
        get_add_compounding_liquidity_amount_and_fee::GetAddCompoundingLiquidityAmountAndFee,
    ),
    GetAddLiquidityAmountAndFee(get_add_liquidity_amount_and_fee::GetAddLiquidityAmountAndFee),
    GetRemoveCompoundingLiquidityAmountAndFee(
        get_remove_compounding_liquidity_amount_and_fee::GetRemoveCompoundingLiquidityAmountAndFee,
    ),
    GetRemoveLiquidityAmountAndFee(
        get_remove_liquidity_amount_and_fee::GetRemoveLiquidityAmountAndFee,
    ),
    GetEntryPriceAndFee(get_entry_price_and_fee::GetEntryPriceAndFee),
    GetExitPriceAndFee(get_exit_price_and_fee::GetExitPriceAndFee),
    GetPnl(get_pnl::GetPnl),
    GetPositionData(get_position_data::GetPositionData),
    GetLiquidationState(get_liquidation_state::GetLiquidationState),
    GetLiquidationPrice(get_liquidation_price::GetLiquidationPrice),
    GetOraclePrice(get_oracle_price::GetOraclePrice),
    GetSwapAmountAndFees(get_swap_amount_and_fees::GetSwapAmountAndFees),
    GetAssetsUnderManagement(get_assets_under_management::GetAssetsUnderManagement),
    GetCompoundingTokenData(get_compounding_token_data::GetCompoundingTokenData),
    GetLpTokenPrice(get_lp_token_price::GetLpTokenPrice),
    GetCompoundingTokenPrice(get_compounding_token_price::GetCompoundingTokenPrice),
    RenameFlp(rename_flp::RenameFlp),
    AddCollateralLogEvent(add_collateral_log_event::AddCollateralLogEvent),
    AddCollateralLogV2Event(add_collateral_log_v2_event::AddCollateralLogV2Event),
    AddCollateralLogV3Event(add_collateral_log_v3_event::AddCollateralLogV3Event),
    AddCollateralLogUsDv1Event(add_collateral_log_us_dv1_event::AddCollateralLogUsDv1Event),
    AddCompoundingLiquidityLogEvent(
        add_compounding_liquidity_log_event::AddCompoundingLiquidityLogEvent,
    ),
    AddLiquidityAndStakeLogEvent(add_liquidity_and_stake_log_event::AddLiquidityAndStakeLogEvent),
    AddLiquidityLogEvent(add_liquidity_log_event::AddLiquidityLogEvent),
    AddLiquidityLogV2Event(add_liquidity_log_v2_event::AddLiquidityLogV2Event),
    CancelTriggerOrderLogEvent(cancel_trigger_order_log_event::CancelTriggerOrderLogEvent),
    CancelUnstakeTokenRequestLogEvent(
        cancel_unstake_token_request_log_event::CancelUnstakeTokenRequestLogEvent,
    ),
    CloseAndSwapLogEvent(close_and_swap_log_event::CloseAndSwapLogEvent),
    CloseAndSwapLogUsDv1Event(close_and_swap_log_us_dv1_event::CloseAndSwapLogUsDv1Event),
    ClosePositionLogEvent(close_position_log_event::ClosePositionLogEvent),
    ClosePositionLogV2Event(close_position_log_v2_event::ClosePositionLogV2Event),
    ClosePositionLogV3Event(close_position_log_v3_event::ClosePositionLogV3Event),
    ClosePositionLogUsDv1Event(close_position_log_us_dv1_event::ClosePositionLogUsDv1Event),
    CollectRebateLogEvent(collect_rebate_log_event::CollectRebateLogEvent),
    CollectRevenueLogEvent(collect_revenue_log_event::CollectRevenueLogEvent),
    CollectStakeRewardLogEvent(collect_stake_reward_log_event::CollectStakeRewardLogEvent),
    CollectStakeRewardLogV2Event(collect_stake_reward_log_v2_event::CollectStakeRewardLogV2Event),
    CollectTokenRewardLogEvent(collect_token_reward_log_event::CollectTokenRewardLogEvent),
    CompoundingFeesLogEvent(compounding_fees_log_event::CompoundingFeesLogEvent),
    DecreaseSizeLogEvent(decrease_size_log_event::DecreaseSizeLogEvent),
    DecreaseSizeLogV2Event(decrease_size_log_v2_event::DecreaseSizeLogV2Event),
    DecreaseSizeLogV3Event(decrease_size_log_v3_event::DecreaseSizeLogV3Event),
    DecreaseSizeLogUsDv1Event(decrease_size_log_us_dv1_event::DecreaseSizeLogUsDv1Event),
    DepositStakeLogEvent(deposit_stake_log_event::DepositStakeLogEvent),
    DepositTokenStakeLogEvent(deposit_token_stake_log_event::DepositTokenStakeLogEvent),
    DistributeTokenRewardLogEvent(distribute_token_reward_log_event::DistributeTokenRewardLogEvent),
    EditLimitOrderLogEvent(edit_limit_order_log_event::EditLimitOrderLogEvent),
    CancelLimitOrderLogEvent(cancel_limit_order_log_event::CancelLimitOrderLogEvent),
    EditTriggerOrderLogEvent(edit_trigger_order_log_event::EditTriggerOrderLogEvent),
    ExecuteLimitOrderLogEvent(execute_limit_order_log_event::ExecuteLimitOrderLogEvent),
    ExecuteLimitOrderLogV2Event(execute_limit_order_log_v2_event::ExecuteLimitOrderLogV2Event),
    ExecuteLimitOrderLogUsDv1Event(
        execute_limit_order_log_us_dv1_event::ExecuteLimitOrderLogUsDv1Event,
    ),
    ExecuteLimitWithSwapLogEvent(execute_limit_with_swap_log_event::ExecuteLimitWithSwapLogEvent),
    ExecuteLimitWithSwapLogV2Event(
        execute_limit_with_swap_log_v2_event::ExecuteLimitWithSwapLogV2Event,
    ),
    ExecuteLimitWithSwapLogUsDv1Event(
        execute_limit_with_swap_log_us_dv1_event::ExecuteLimitWithSwapLogUsDv1Event,
    ),
    ExecuteTriggerOrderLogEvent(execute_trigger_order_log_event::ExecuteTriggerOrderLogEvent),
    ExecuteTriggerOrderLogUsDv1Event(
        execute_trigger_order_log_us_dv1_event::ExecuteTriggerOrderLogUsDv1Event,
    ),
    ExecuteTriggerWithSwapLogEvent(
        execute_trigger_with_swap_log_event::ExecuteTriggerWithSwapLogEvent,
    ),
    ExecuteTriggerWithSwapLogUsDv1Event(
        execute_trigger_with_swap_log_us_dv1_event::ExecuteTriggerWithSwapLogUsDv1Event,
    ),
    IncreaseSizeLogEvent(increase_size_log_event::IncreaseSizeLogEvent),
    IncreaseSizeLogV2Event(increase_size_log_v2_event::IncreaseSizeLogV2Event),
    IncreaseSizeLogV3Event(increase_size_log_v3_event::IncreaseSizeLogV3Event),
    IncreaseSizeLogV4Event(increase_size_log_v4_event::IncreaseSizeLogV4Event),
    IncreaseSizeLogUsDv1Event(increase_size_log_us_dv1_event::IncreaseSizeLogUsDv1Event),
    LiquidateLogEvent(liquidate_log_event::LiquidateLogEvent),
    LiquidateLogV2Event(liquidate_log_v2_event::LiquidateLogV2Event),
    LiquidateLogV3Event(liquidate_log_v3_event::LiquidateLogV3Event),
    LiquidateLogUsDv1Event(liquidate_log_us_dv1_event::LiquidateLogUsDv1Event),
    MigrateFlpLogEvent(migrate_flp_log_event::MigrateFlpLogEvent),
    MigrateStakeLogEvent(migrate_stake_log_event::MigrateStakeLogEvent),
    MoveProtocolFeesLogEvent(move_protocol_fees_log_event::MoveProtocolFeesLogEvent),
    OpenPositionLogEvent(open_position_log_event::OpenPositionLogEvent),
    OpenPositionLogV2Event(open_position_log_v2_event::OpenPositionLogV2Event),
    OpenPositionLogV3Event(open_position_log_v3_event::OpenPositionLogV3Event),
    OpenPositionLogV4Event(open_position_log_v4_event::OpenPositionLogV4Event),
    OpenPositionLogUsDv1Event(open_position_log_us_dv1_event::OpenPositionLogUsDv1Event),
    PlaceLimitOrderLogEvent(place_limit_order_log_event::PlaceLimitOrderLogEvent),
    PlaceTriggerOrderLogEvent(place_trigger_order_log_event::PlaceTriggerOrderLogEvent),
    RefreshStakeLogEvent(refresh_stake_log_event::RefreshStakeLogEvent),
    RefreshStakeUserLogEvent(refresh_stake_user_log_event::RefreshStakeUserLogEvent),
    RemoveCollateralAndSwapLogEvent(
        remove_collateral_and_swap_log_event::RemoveCollateralAndSwapLogEvent,
    ),
    RemoveCollateralAndSwapLogUsDv1Event(
        remove_collateral_and_swap_log_us_dv1_event::RemoveCollateralAndSwapLogUsDv1Event,
    ),
    RemoveCollateralLogEvent(remove_collateral_log_event::RemoveCollateralLogEvent),
    RemoveCollateralLogV2Event(remove_collateral_log_v2_event::RemoveCollateralLogV2Event),
    RemoveCollateralLogV3Event(remove_collateral_log_v3_event::RemoveCollateralLogV3Event),
    RemoveCollateralLogUsDv1Event(
        remove_collateral_log_us_dv1_event::RemoveCollateralLogUsDv1Event,
    ),
    RemoveCompoundingLiquidityLogEvent(
        remove_compounding_liquidity_log_event::RemoveCompoundingLiquidityLogEvent,
    ),
    RemoveLiquidityLogEvent(remove_liquidity_log_event::RemoveLiquidityLogEvent),
    RemoveLiquidityLogV2Event(remove_liquidity_log_v2_event::RemoveLiquidityLogV2Event),
    SetTokenRewardLogEvent(set_token_reward_log_event::SetTokenRewardLogEvent),
    SettleRebatesLogEvent(settle_rebates_log_event::SettleRebatesLogEvent),
    SwapAndAddCollateralLogEvent(swap_and_add_collateral_log_event::SwapAndAddCollateralLogEvent),
    SwapAndAddCollateralLogUsDv1Event(
        swap_and_add_collateral_log_us_dv1_event::SwapAndAddCollateralLogUsDv1Event,
    ),
    SwapAndOpenLogEvent(swap_and_open_log_event::SwapAndOpenLogEvent),
    SwapAndOpenLogV2Event(swap_and_open_log_v2_event::SwapAndOpenLogV2Event),
    SwapAndOpenLogUsDv1Event(swap_and_open_log_us_dv1_event::SwapAndOpenLogUsDv1Event),
    SwapFeeInternalLogEvent(swap_fee_internal_log_event::SwapFeeInternalLogEvent),
    SwapFeeInternalLogV2Event(swap_fee_internal_log_v2_event::SwapFeeInternalLogV2Event),
    SwapFeeInternalLogV3Event(swap_fee_internal_log_v3_event::SwapFeeInternalLogV3Event),
    SwapLogEvent(swap_log_event::SwapLogEvent),
    SwapLogV2Event(swap_log_v2_event::SwapLogV2Event),
    UnstakeInstantLogEvent(unstake_instant_log_event::UnstakeInstantLogEvent),
    UnstakeRequestLogEvent(unstake_request_log_event::UnstakeRequestLogEvent),
    UnstakeTokenInstantLogEvent(unstake_token_instant_log_event::UnstakeTokenInstantLogEvent),
    UnstakeTokenRequestLogEvent(unstake_token_request_log_event::UnstakeTokenRequestLogEvent),
    WithdrawStakeLogEvent(withdraw_stake_log_event::WithdrawStakeLogEvent),
    WithdrawTokenLogEvent(withdraw_token_log_event::WithdrawTokenLogEvent),
    MigratePositionLogEvent(migrate_position_log_event::MigratePositionLogEvent),
    BurnAndClaimLogEvent(burn_and_claim_log_event::BurnAndClaimLogEvent),
    BurnAndStakeLogEvent(burn_and_stake_log_event::BurnAndStakeLogEvent),
    VoltagePointsLogEvent(voltage_points_log_event::VoltagePointsLogEvent),
    ReferralRebateLogEvent(referral_rebate_log_event::ReferralRebateLogEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for FlashTradePerpsV2Decoder {
    type InstructionType = FlashTradePerpsV2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            FlashTradePerpsV2Instruction::Init => init::Init,
            FlashTradePerpsV2Instruction::AddPool => add_pool::AddPool,
            FlashTradePerpsV2Instruction::RemovePool => remove_pool::RemovePool,
            FlashTradePerpsV2Instruction::AddCustody => add_custody::AddCustody,
            FlashTradePerpsV2Instruction::AddCustodyToken22Account => add_custody_token22_account::AddCustodyToken22Account,
            FlashTradePerpsV2Instruction::AddInternalOracle => add_internal_oracle::AddInternalOracle,
            FlashTradePerpsV2Instruction::RemoveCustody => remove_custody::RemoveCustody,
            FlashTradePerpsV2Instruction::AddMarket => add_market::AddMarket,
            FlashTradePerpsV2Instruction::RemoveMarket => remove_market::RemoveMarket,
            FlashTradePerpsV2Instruction::Reimburse => reimburse::Reimburse,
            FlashTradePerpsV2Instruction::ResizeInternalOracle => resize_internal_oracle::ResizeInternalOracle,
            FlashTradePerpsV2Instruction::SetAdminSigners => set_admin_signers::SetAdminSigners,
            FlashTradePerpsV2Instruction::SetCustodyConfig => set_custody_config::SetCustodyConfig,
            FlashTradePerpsV2Instruction::SetPerpetualsConfig => set_perpetuals_config::SetPerpetualsConfig,
            FlashTradePerpsV2Instruction::SetPermissions => set_permissions::SetPermissions,
            FlashTradePerpsV2Instruction::SetPoolConfig => set_pool_config::SetPoolConfig,
            FlashTradePerpsV2Instruction::SetProtocolFeeShare => set_protocol_fee_share::SetProtocolFeeShare,
            FlashTradePerpsV2Instruction::SetMarketConfig => set_market_config::SetMarketConfig,
            FlashTradePerpsV2Instruction::SetFlpStakeConfig => set_flp_stake_config::SetFlpStakeConfig,
            FlashTradePerpsV2Instruction::SetTokenReward => set_token_reward::SetTokenReward,
            FlashTradePerpsV2Instruction::SetTokenStakeLevel => set_token_stake_level::SetTokenStakeLevel,
            FlashTradePerpsV2Instruction::SetTokenVaultConfig => set_token_vault_config::SetTokenVaultConfig,
            FlashTradePerpsV2Instruction::WithdrawFees => withdraw_fees::WithdrawFees,
            FlashTradePerpsV2Instruction::WithdrawInstantFees => withdraw_instant_fees::WithdrawInstantFees,
            FlashTradePerpsV2Instruction::WithdrawUnclaimedTokens => withdraw_unclaimed_tokens::WithdrawUnclaimedTokens,
            FlashTradePerpsV2Instruction::WithdrawSolFees => withdraw_sol_fees::WithdrawSolFees,
            FlashTradePerpsV2Instruction::UpdateCustody => update_custody::UpdateCustody,
            FlashTradePerpsV2Instruction::UpdateTokenRatios => update_token_ratios::UpdateTokenRatios,
            FlashTradePerpsV2Instruction::InitStaking => init_staking::InitStaking,
            FlashTradePerpsV2Instruction::InitCompounding => init_compounding::InitCompounding,
            FlashTradePerpsV2Instruction::InitRebateVault => init_rebate_vault::InitRebateVault,
            FlashTradePerpsV2Instruction::InitRevenueTokenAccount => init_revenue_token_account::InitRevenueTokenAccount,
            FlashTradePerpsV2Instruction::InitTokenVault => init_token_vault::InitTokenVault,
            FlashTradePerpsV2Instruction::SetCustomOraclePrice => set_custom_oracle_price::SetCustomOraclePrice,
            FlashTradePerpsV2Instruction::SetInternalCurrentPrice => set_internal_current_price::SetInternalCurrentPrice,
            FlashTradePerpsV2Instruction::SetInternalEmaPrice => set_internal_ema_price::SetInternalEmaPrice,
            FlashTradePerpsV2Instruction::SetInternalOraclePrice => set_internal_oracle_price::SetInternalOraclePrice,
            FlashTradePerpsV2Instruction::SetLpTokenPrice => set_lp_token_price::SetLpTokenPrice,
            FlashTradePerpsV2Instruction::SetFeeShare => set_fee_share::SetFeeShare,
            FlashTradePerpsV2Instruction::TestInit => test_init::TestInit,
            FlashTradePerpsV2Instruction::SetTestTime => set_test_time::SetTestTime,
            FlashTradePerpsV2Instruction::SwapFeeInternal => swap_fee_internal::SwapFeeInternal,
            FlashTradePerpsV2Instruction::Swap => swap::Swap,
            FlashTradePerpsV2Instruction::SettleRebates => settle_rebates::SettleRebates,
            FlashTradePerpsV2Instruction::SwapAndAddCollateral => swap_and_add_collateral::SwapAndAddCollateral,
            FlashTradePerpsV2Instruction::SwapAndOpen => swap_and_open::SwapAndOpen,
            FlashTradePerpsV2Instruction::CloseAndSwap => close_and_swap::CloseAndSwap,
            FlashTradePerpsV2Instruction::AddLiquidityAndStake => add_liquidity_and_stake::AddLiquidityAndStake,
            FlashTradePerpsV2Instruction::AddLiquidity => add_liquidity::AddLiquidity,
            FlashTradePerpsV2Instruction::AddCompoundingLiquidity => add_compounding_liquidity::AddCompoundingLiquidity,
            FlashTradePerpsV2Instruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            FlashTradePerpsV2Instruction::DepositTokenStake => deposit_token_stake::DepositTokenStake,
            FlashTradePerpsV2Instruction::DistributeTokenReward => distribute_token_reward::DistributeTokenReward,
            FlashTradePerpsV2Instruction::DepositStake => deposit_stake::DepositStake,
            FlashTradePerpsV2Instruction::RefreshStake => refresh_stake::RefreshStake,
            FlashTradePerpsV2Instruction::UnstakeInstant => unstake_instant::UnstakeInstant,
            FlashTradePerpsV2Instruction::WithdrawStake => withdraw_stake::WithdrawStake,
            FlashTradePerpsV2Instruction::WithdrawToken => withdraw_token::WithdrawToken,
            FlashTradePerpsV2Instruction::CollectRebate => collect_rebate::CollectRebate,
            FlashTradePerpsV2Instruction::CollectRevenue => collect_revenue::CollectRevenue,
            FlashTradePerpsV2Instruction::CollectStakeFees => collect_stake_fees::CollectStakeFees,
            FlashTradePerpsV2Instruction::CollectTokenReward => collect_token_reward::CollectTokenReward,
            FlashTradePerpsV2Instruction::UnstakeRequest => unstake_request::UnstakeRequest,
            FlashTradePerpsV2Instruction::UnstakeTokenInstant => unstake_token_instant::UnstakeTokenInstant,
            FlashTradePerpsV2Instruction::UnstakeTokenRequest => unstake_token_request::UnstakeTokenRequest,
            FlashTradePerpsV2Instruction::MigrateFlp => migrate_flp::MigrateFlp,
            FlashTradePerpsV2Instruction::MigrateStake => migrate_stake::MigrateStake,
            FlashTradePerpsV2Instruction::MoveProtocolFees => move_protocol_fees::MoveProtocolFees,
            FlashTradePerpsV2Instruction::CompoundFees => compound_fees::CompoundFees,
            FlashTradePerpsV2Instruction::RemoveCompoundingLiquidity => remove_compounding_liquidity::RemoveCompoundingLiquidity,
            FlashTradePerpsV2Instruction::CreateReferral => create_referral::CreateReferral,
            FlashTradePerpsV2Instruction::OpenPosition => open_position::OpenPosition,
            FlashTradePerpsV2Instruction::AddCollateral => add_collateral::AddCollateral,
            FlashTradePerpsV2Instruction::RemoveCollateralAndSwap => remove_collateral_and_swap::RemoveCollateralAndSwap,
            FlashTradePerpsV2Instruction::RemoveCollateral => remove_collateral::RemoveCollateral,
            FlashTradePerpsV2Instruction::IncreaseSize => increase_size::IncreaseSize,
            FlashTradePerpsV2Instruction::DecreaseSize => decrease_size::DecreaseSize,
            FlashTradePerpsV2Instruction::CancelAllTriggerOrders => cancel_all_trigger_orders::CancelAllTriggerOrders,
            FlashTradePerpsV2Instruction::CancelTriggerOrder => cancel_trigger_order::CancelTriggerOrder,
            FlashTradePerpsV2Instruction::CancelUnstakeTokenRequest => cancel_unstake_token_request::CancelUnstakeTokenRequest,
            FlashTradePerpsV2Instruction::ClosePosition => close_position::ClosePosition,
            FlashTradePerpsV2Instruction::ExecuteLimitWithSwap => execute_limit_with_swap::ExecuteLimitWithSwap,
            FlashTradePerpsV2Instruction::ExecuteLimitOrder => execute_limit_order::ExecuteLimitOrder,
            FlashTradePerpsV2Instruction::PlaceLimitOrder => place_limit_order::PlaceLimitOrder,
            FlashTradePerpsV2Instruction::EditLimitOrder => edit_limit_order::EditLimitOrder,
            FlashTradePerpsV2Instruction::EditTriggerOrder => edit_trigger_order::EditTriggerOrder,
            FlashTradePerpsV2Instruction::PlaceTriggerOrder => place_trigger_order::PlaceTriggerOrder,
            FlashTradePerpsV2Instruction::ExecuteTriggerWithSwap => execute_trigger_with_swap::ExecuteTriggerWithSwap,
            FlashTradePerpsV2Instruction::ExecuteTriggerOrder => execute_trigger_order::ExecuteTriggerOrder,
            FlashTradePerpsV2Instruction::Liquidate => liquidate::Liquidate,
            FlashTradePerpsV2Instruction::GetAddCompoundingLiquidityAmountAndFee => get_add_compounding_liquidity_amount_and_fee::GetAddCompoundingLiquidityAmountAndFee,
            FlashTradePerpsV2Instruction::GetAddLiquidityAmountAndFee => get_add_liquidity_amount_and_fee::GetAddLiquidityAmountAndFee,
            FlashTradePerpsV2Instruction::GetRemoveCompoundingLiquidityAmountAndFee => get_remove_compounding_liquidity_amount_and_fee::GetRemoveCompoundingLiquidityAmountAndFee,
            FlashTradePerpsV2Instruction::GetRemoveLiquidityAmountAndFee => get_remove_liquidity_amount_and_fee::GetRemoveLiquidityAmountAndFee,
            FlashTradePerpsV2Instruction::GetEntryPriceAndFee => get_entry_price_and_fee::GetEntryPriceAndFee,
            FlashTradePerpsV2Instruction::GetExitPriceAndFee => get_exit_price_and_fee::GetExitPriceAndFee,
            FlashTradePerpsV2Instruction::GetPnl => get_pnl::GetPnl,
            FlashTradePerpsV2Instruction::GetPositionData => get_position_data::GetPositionData,
            FlashTradePerpsV2Instruction::GetLiquidationState => get_liquidation_state::GetLiquidationState,
            FlashTradePerpsV2Instruction::GetLiquidationPrice => get_liquidation_price::GetLiquidationPrice,
            FlashTradePerpsV2Instruction::GetOraclePrice => get_oracle_price::GetOraclePrice,
            FlashTradePerpsV2Instruction::GetSwapAmountAndFees => get_swap_amount_and_fees::GetSwapAmountAndFees,
            FlashTradePerpsV2Instruction::GetAssetsUnderManagement => get_assets_under_management::GetAssetsUnderManagement,
            FlashTradePerpsV2Instruction::GetCompoundingTokenData => get_compounding_token_data::GetCompoundingTokenData,
            FlashTradePerpsV2Instruction::GetLpTokenPrice => get_lp_token_price::GetLpTokenPrice,
            FlashTradePerpsV2Instruction::GetCompoundingTokenPrice => get_compounding_token_price::GetCompoundingTokenPrice,
            FlashTradePerpsV2Instruction::RenameFlp => rename_flp::RenameFlp,
            FlashTradePerpsV2Instruction::AddCollateralLogEvent => add_collateral_log_event::AddCollateralLogEvent,
            FlashTradePerpsV2Instruction::AddCollateralLogV2Event => add_collateral_log_v2_event::AddCollateralLogV2Event,
            FlashTradePerpsV2Instruction::AddCollateralLogV3Event => add_collateral_log_v3_event::AddCollateralLogV3Event,
            FlashTradePerpsV2Instruction::AddCollateralLogUsDv1Event => add_collateral_log_us_dv1_event::AddCollateralLogUsDv1Event,
            FlashTradePerpsV2Instruction::AddCompoundingLiquidityLogEvent => add_compounding_liquidity_log_event::AddCompoundingLiquidityLogEvent,
            FlashTradePerpsV2Instruction::AddLiquidityAndStakeLogEvent => add_liquidity_and_stake_log_event::AddLiquidityAndStakeLogEvent,
            FlashTradePerpsV2Instruction::AddLiquidityLogEvent => add_liquidity_log_event::AddLiquidityLogEvent,
            FlashTradePerpsV2Instruction::AddLiquidityLogV2Event => add_liquidity_log_v2_event::AddLiquidityLogV2Event,
            FlashTradePerpsV2Instruction::CancelTriggerOrderLogEvent => cancel_trigger_order_log_event::CancelTriggerOrderLogEvent,
            FlashTradePerpsV2Instruction::CancelUnstakeTokenRequestLogEvent => cancel_unstake_token_request_log_event::CancelUnstakeTokenRequestLogEvent,
            FlashTradePerpsV2Instruction::CloseAndSwapLogEvent => close_and_swap_log_event::CloseAndSwapLogEvent,
            FlashTradePerpsV2Instruction::CloseAndSwapLogUsDv1Event => close_and_swap_log_us_dv1_event::CloseAndSwapLogUsDv1Event,
            FlashTradePerpsV2Instruction::ClosePositionLogEvent => close_position_log_event::ClosePositionLogEvent,
            FlashTradePerpsV2Instruction::ClosePositionLogV2Event => close_position_log_v2_event::ClosePositionLogV2Event,
            FlashTradePerpsV2Instruction::ClosePositionLogV3Event => close_position_log_v3_event::ClosePositionLogV3Event,
            FlashTradePerpsV2Instruction::ClosePositionLogUsDv1Event => close_position_log_us_dv1_event::ClosePositionLogUsDv1Event,
            FlashTradePerpsV2Instruction::CollectRebateLogEvent => collect_rebate_log_event::CollectRebateLogEvent,
            FlashTradePerpsV2Instruction::CollectRevenueLogEvent => collect_revenue_log_event::CollectRevenueLogEvent,
            FlashTradePerpsV2Instruction::CollectStakeRewardLogEvent => collect_stake_reward_log_event::CollectStakeRewardLogEvent,
            FlashTradePerpsV2Instruction::CollectStakeRewardLogV2Event => collect_stake_reward_log_v2_event::CollectStakeRewardLogV2Event,
            FlashTradePerpsV2Instruction::CollectTokenRewardLogEvent => collect_token_reward_log_event::CollectTokenRewardLogEvent,
            FlashTradePerpsV2Instruction::CompoundingFeesLogEvent => compounding_fees_log_event::CompoundingFeesLogEvent,
            FlashTradePerpsV2Instruction::DecreaseSizeLogEvent => decrease_size_log_event::DecreaseSizeLogEvent,
            FlashTradePerpsV2Instruction::DecreaseSizeLogV2Event => decrease_size_log_v2_event::DecreaseSizeLogV2Event,
            FlashTradePerpsV2Instruction::DecreaseSizeLogV3Event => decrease_size_log_v3_event::DecreaseSizeLogV3Event,
            FlashTradePerpsV2Instruction::DecreaseSizeLogUsDv1Event => decrease_size_log_us_dv1_event::DecreaseSizeLogUsDv1Event,
            FlashTradePerpsV2Instruction::DepositStakeLogEvent => deposit_stake_log_event::DepositStakeLogEvent,
            FlashTradePerpsV2Instruction::DepositTokenStakeLogEvent => deposit_token_stake_log_event::DepositTokenStakeLogEvent,
            FlashTradePerpsV2Instruction::DistributeTokenRewardLogEvent => distribute_token_reward_log_event::DistributeTokenRewardLogEvent,
            FlashTradePerpsV2Instruction::EditLimitOrderLogEvent => edit_limit_order_log_event::EditLimitOrderLogEvent,
            FlashTradePerpsV2Instruction::CancelLimitOrderLogEvent => cancel_limit_order_log_event::CancelLimitOrderLogEvent,
            FlashTradePerpsV2Instruction::EditTriggerOrderLogEvent => edit_trigger_order_log_event::EditTriggerOrderLogEvent,
            FlashTradePerpsV2Instruction::ExecuteLimitOrderLogEvent => execute_limit_order_log_event::ExecuteLimitOrderLogEvent,
            FlashTradePerpsV2Instruction::ExecuteLimitOrderLogV2Event => execute_limit_order_log_v2_event::ExecuteLimitOrderLogV2Event,
            FlashTradePerpsV2Instruction::ExecuteLimitOrderLogUsDv1Event => execute_limit_order_log_us_dv1_event::ExecuteLimitOrderLogUsDv1Event,
            FlashTradePerpsV2Instruction::ExecuteLimitWithSwapLogEvent => execute_limit_with_swap_log_event::ExecuteLimitWithSwapLogEvent,
            FlashTradePerpsV2Instruction::ExecuteLimitWithSwapLogV2Event => execute_limit_with_swap_log_v2_event::ExecuteLimitWithSwapLogV2Event,
            FlashTradePerpsV2Instruction::ExecuteLimitWithSwapLogUsDv1Event => execute_limit_with_swap_log_us_dv1_event::ExecuteLimitWithSwapLogUsDv1Event,
            FlashTradePerpsV2Instruction::ExecuteTriggerOrderLogEvent => execute_trigger_order_log_event::ExecuteTriggerOrderLogEvent,
            FlashTradePerpsV2Instruction::ExecuteTriggerOrderLogUsDv1Event => execute_trigger_order_log_us_dv1_event::ExecuteTriggerOrderLogUsDv1Event,
            FlashTradePerpsV2Instruction::ExecuteTriggerWithSwapLogEvent => execute_trigger_with_swap_log_event::ExecuteTriggerWithSwapLogEvent,
            FlashTradePerpsV2Instruction::ExecuteTriggerWithSwapLogUsDv1Event => execute_trigger_with_swap_log_us_dv1_event::ExecuteTriggerWithSwapLogUsDv1Event,
            FlashTradePerpsV2Instruction::IncreaseSizeLogEvent => increase_size_log_event::IncreaseSizeLogEvent,
            FlashTradePerpsV2Instruction::IncreaseSizeLogV2Event => increase_size_log_v2_event::IncreaseSizeLogV2Event,
            FlashTradePerpsV2Instruction::IncreaseSizeLogV3Event => increase_size_log_v3_event::IncreaseSizeLogV3Event,
            FlashTradePerpsV2Instruction::IncreaseSizeLogV4Event => increase_size_log_v4_event::IncreaseSizeLogV4Event,
            FlashTradePerpsV2Instruction::IncreaseSizeLogUsDv1Event => increase_size_log_us_dv1_event::IncreaseSizeLogUsDv1Event,
            FlashTradePerpsV2Instruction::LiquidateLogEvent => liquidate_log_event::LiquidateLogEvent,
            FlashTradePerpsV2Instruction::LiquidateLogV2Event => liquidate_log_v2_event::LiquidateLogV2Event,
            FlashTradePerpsV2Instruction::LiquidateLogV3Event => liquidate_log_v3_event::LiquidateLogV3Event,
            FlashTradePerpsV2Instruction::LiquidateLogUsDv1Event => liquidate_log_us_dv1_event::LiquidateLogUsDv1Event,
            FlashTradePerpsV2Instruction::MigrateFlpLogEvent => migrate_flp_log_event::MigrateFlpLogEvent,
            FlashTradePerpsV2Instruction::MigrateStakeLogEvent => migrate_stake_log_event::MigrateStakeLogEvent,
            FlashTradePerpsV2Instruction::MoveProtocolFeesLogEvent => move_protocol_fees_log_event::MoveProtocolFeesLogEvent,
            FlashTradePerpsV2Instruction::OpenPositionLogEvent => open_position_log_event::OpenPositionLogEvent,
            FlashTradePerpsV2Instruction::OpenPositionLogV2Event => open_position_log_v2_event::OpenPositionLogV2Event,
            FlashTradePerpsV2Instruction::OpenPositionLogV3Event => open_position_log_v3_event::OpenPositionLogV3Event,
            FlashTradePerpsV2Instruction::OpenPositionLogV4Event => open_position_log_v4_event::OpenPositionLogV4Event,
            FlashTradePerpsV2Instruction::OpenPositionLogUsDv1Event => open_position_log_us_dv1_event::OpenPositionLogUsDv1Event,
            FlashTradePerpsV2Instruction::PlaceLimitOrderLogEvent => place_limit_order_log_event::PlaceLimitOrderLogEvent,
            FlashTradePerpsV2Instruction::PlaceTriggerOrderLogEvent => place_trigger_order_log_event::PlaceTriggerOrderLogEvent,
            FlashTradePerpsV2Instruction::RefreshStakeLogEvent => refresh_stake_log_event::RefreshStakeLogEvent,
            FlashTradePerpsV2Instruction::RefreshStakeUserLogEvent => refresh_stake_user_log_event::RefreshStakeUserLogEvent,
            FlashTradePerpsV2Instruction::RemoveCollateralAndSwapLogEvent => remove_collateral_and_swap_log_event::RemoveCollateralAndSwapLogEvent,
            FlashTradePerpsV2Instruction::RemoveCollateralAndSwapLogUsDv1Event => remove_collateral_and_swap_log_us_dv1_event::RemoveCollateralAndSwapLogUsDv1Event,
            FlashTradePerpsV2Instruction::RemoveCollateralLogEvent => remove_collateral_log_event::RemoveCollateralLogEvent,
            FlashTradePerpsV2Instruction::RemoveCollateralLogV2Event => remove_collateral_log_v2_event::RemoveCollateralLogV2Event,
            FlashTradePerpsV2Instruction::RemoveCollateralLogV3Event => remove_collateral_log_v3_event::RemoveCollateralLogV3Event,
            FlashTradePerpsV2Instruction::RemoveCollateralLogUsDv1Event => remove_collateral_log_us_dv1_event::RemoveCollateralLogUsDv1Event,
            FlashTradePerpsV2Instruction::RemoveCompoundingLiquidityLogEvent => remove_compounding_liquidity_log_event::RemoveCompoundingLiquidityLogEvent,
            FlashTradePerpsV2Instruction::RemoveLiquidityLogEvent => remove_liquidity_log_event::RemoveLiquidityLogEvent,
            FlashTradePerpsV2Instruction::RemoveLiquidityLogV2Event => remove_liquidity_log_v2_event::RemoveLiquidityLogV2Event,
            FlashTradePerpsV2Instruction::SetTokenRewardLogEvent => set_token_reward_log_event::SetTokenRewardLogEvent,
            FlashTradePerpsV2Instruction::SettleRebatesLogEvent => settle_rebates_log_event::SettleRebatesLogEvent,
            FlashTradePerpsV2Instruction::SwapAndAddCollateralLogEvent => swap_and_add_collateral_log_event::SwapAndAddCollateralLogEvent,
            FlashTradePerpsV2Instruction::SwapAndAddCollateralLogUsDv1Event => swap_and_add_collateral_log_us_dv1_event::SwapAndAddCollateralLogUsDv1Event,
            FlashTradePerpsV2Instruction::SwapAndOpenLogEvent => swap_and_open_log_event::SwapAndOpenLogEvent,
            FlashTradePerpsV2Instruction::SwapAndOpenLogV2Event => swap_and_open_log_v2_event::SwapAndOpenLogV2Event,
            FlashTradePerpsV2Instruction::SwapAndOpenLogUsDv1Event => swap_and_open_log_us_dv1_event::SwapAndOpenLogUsDv1Event,
            FlashTradePerpsV2Instruction::SwapFeeInternalLogEvent => swap_fee_internal_log_event::SwapFeeInternalLogEvent,
            FlashTradePerpsV2Instruction::SwapFeeInternalLogV2Event => swap_fee_internal_log_v2_event::SwapFeeInternalLogV2Event,
            FlashTradePerpsV2Instruction::SwapFeeInternalLogV3Event => swap_fee_internal_log_v3_event::SwapFeeInternalLogV3Event,
            FlashTradePerpsV2Instruction::SwapLogEvent => swap_log_event::SwapLogEvent,
            FlashTradePerpsV2Instruction::SwapLogV2Event => swap_log_v2_event::SwapLogV2Event,
            FlashTradePerpsV2Instruction::UnstakeInstantLogEvent => unstake_instant_log_event::UnstakeInstantLogEvent,
            FlashTradePerpsV2Instruction::UnstakeRequestLogEvent => unstake_request_log_event::UnstakeRequestLogEvent,
            FlashTradePerpsV2Instruction::UnstakeTokenInstantLogEvent => unstake_token_instant_log_event::UnstakeTokenInstantLogEvent,
            FlashTradePerpsV2Instruction::UnstakeTokenRequestLogEvent => unstake_token_request_log_event::UnstakeTokenRequestLogEvent,
            FlashTradePerpsV2Instruction::WithdrawStakeLogEvent => withdraw_stake_log_event::WithdrawStakeLogEvent,
            FlashTradePerpsV2Instruction::WithdrawTokenLogEvent => withdraw_token_log_event::WithdrawTokenLogEvent,
            FlashTradePerpsV2Instruction::MigratePositionLogEvent => migrate_position_log_event::MigratePositionLogEvent,
            FlashTradePerpsV2Instruction::BurnAndClaimLogEvent => burn_and_claim_log_event::BurnAndClaimLogEvent,
            FlashTradePerpsV2Instruction::BurnAndStakeLogEvent => burn_and_stake_log_event::BurnAndStakeLogEvent,
            FlashTradePerpsV2Instruction::VoltagePointsLogEvent => voltage_points_log_event::VoltagePointsLogEvent,
            FlashTradePerpsV2Instruction::ReferralRebateLogEvent => referral_rebate_log_event::ReferralRebateLogEvent,
        )
    }
}
