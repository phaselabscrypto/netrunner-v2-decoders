use super::FlashTradePerpsDecoder;
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
pub enum FlashTradePerpsInstruction {
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

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for FlashTradePerpsDecoder {
    type InstructionType = FlashTradePerpsInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            FlashTradePerpsInstruction::Init => init::Init,
            FlashTradePerpsInstruction::AddPool => add_pool::AddPool,
            FlashTradePerpsInstruction::RemovePool => remove_pool::RemovePool,
            FlashTradePerpsInstruction::AddCustody => add_custody::AddCustody,
            FlashTradePerpsInstruction::AddCustodyToken22Account => add_custody_token22_account::AddCustodyToken22Account,
            FlashTradePerpsInstruction::AddInternalOracle => add_internal_oracle::AddInternalOracle,
            FlashTradePerpsInstruction::RemoveCustody => remove_custody::RemoveCustody,
            FlashTradePerpsInstruction::AddMarket => add_market::AddMarket,
            FlashTradePerpsInstruction::RemoveMarket => remove_market::RemoveMarket,
            FlashTradePerpsInstruction::Reimburse => reimburse::Reimburse,
            FlashTradePerpsInstruction::ResizeInternalOracle => resize_internal_oracle::ResizeInternalOracle,
            FlashTradePerpsInstruction::SetAdminSigners => set_admin_signers::SetAdminSigners,
            FlashTradePerpsInstruction::SetCustodyConfig => set_custody_config::SetCustodyConfig,
            FlashTradePerpsInstruction::SetPerpetualsConfig => set_perpetuals_config::SetPerpetualsConfig,
            FlashTradePerpsInstruction::SetPermissions => set_permissions::SetPermissions,
            FlashTradePerpsInstruction::SetPoolConfig => set_pool_config::SetPoolConfig,
            FlashTradePerpsInstruction::SetProtocolFeeShare => set_protocol_fee_share::SetProtocolFeeShare,
            FlashTradePerpsInstruction::SetMarketConfig => set_market_config::SetMarketConfig,
            FlashTradePerpsInstruction::SetFlpStakeConfig => set_flp_stake_config::SetFlpStakeConfig,
            FlashTradePerpsInstruction::SetTokenReward => set_token_reward::SetTokenReward,
            FlashTradePerpsInstruction::SetTokenStakeLevel => set_token_stake_level::SetTokenStakeLevel,
            FlashTradePerpsInstruction::SetTokenVaultConfig => set_token_vault_config::SetTokenVaultConfig,
            FlashTradePerpsInstruction::WithdrawFees => withdraw_fees::WithdrawFees,
            FlashTradePerpsInstruction::WithdrawInstantFees => withdraw_instant_fees::WithdrawInstantFees,
            FlashTradePerpsInstruction::WithdrawUnclaimedTokens => withdraw_unclaimed_tokens::WithdrawUnclaimedTokens,
            FlashTradePerpsInstruction::WithdrawSolFees => withdraw_sol_fees::WithdrawSolFees,
            FlashTradePerpsInstruction::UpdateCustody => update_custody::UpdateCustody,
            FlashTradePerpsInstruction::UpdateTokenRatios => update_token_ratios::UpdateTokenRatios,
            FlashTradePerpsInstruction::InitStaking => init_staking::InitStaking,
            FlashTradePerpsInstruction::InitCompounding => init_compounding::InitCompounding,
            FlashTradePerpsInstruction::InitRebateVault => init_rebate_vault::InitRebateVault,
            FlashTradePerpsInstruction::InitRevenueTokenAccount => init_revenue_token_account::InitRevenueTokenAccount,
            FlashTradePerpsInstruction::InitTokenVault => init_token_vault::InitTokenVault,
            FlashTradePerpsInstruction::SetCustomOraclePrice => set_custom_oracle_price::SetCustomOraclePrice,
            FlashTradePerpsInstruction::SetInternalCurrentPrice => set_internal_current_price::SetInternalCurrentPrice,
            FlashTradePerpsInstruction::SetInternalEmaPrice => set_internal_ema_price::SetInternalEmaPrice,
            FlashTradePerpsInstruction::SetInternalOraclePrice => set_internal_oracle_price::SetInternalOraclePrice,
            FlashTradePerpsInstruction::SetLpTokenPrice => set_lp_token_price::SetLpTokenPrice,
            FlashTradePerpsInstruction::SetFeeShare => set_fee_share::SetFeeShare,
            FlashTradePerpsInstruction::TestInit => test_init::TestInit,
            FlashTradePerpsInstruction::SetTestTime => set_test_time::SetTestTime,
            FlashTradePerpsInstruction::SwapFeeInternal => swap_fee_internal::SwapFeeInternal,
            FlashTradePerpsInstruction::Swap => swap::Swap,
            FlashTradePerpsInstruction::SettleRebates => settle_rebates::SettleRebates,
            FlashTradePerpsInstruction::SwapAndAddCollateral => swap_and_add_collateral::SwapAndAddCollateral,
            FlashTradePerpsInstruction::SwapAndOpen => swap_and_open::SwapAndOpen,
            FlashTradePerpsInstruction::CloseAndSwap => close_and_swap::CloseAndSwap,
            FlashTradePerpsInstruction::AddLiquidityAndStake => add_liquidity_and_stake::AddLiquidityAndStake,
            FlashTradePerpsInstruction::AddLiquidity => add_liquidity::AddLiquidity,
            FlashTradePerpsInstruction::AddCompoundingLiquidity => add_compounding_liquidity::AddCompoundingLiquidity,
            FlashTradePerpsInstruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            FlashTradePerpsInstruction::DepositTokenStake => deposit_token_stake::DepositTokenStake,
            FlashTradePerpsInstruction::DistributeTokenReward => distribute_token_reward::DistributeTokenReward,
            FlashTradePerpsInstruction::DepositStake => deposit_stake::DepositStake,
            FlashTradePerpsInstruction::RefreshStake => refresh_stake::RefreshStake,
            FlashTradePerpsInstruction::UnstakeInstant => unstake_instant::UnstakeInstant,
            FlashTradePerpsInstruction::WithdrawStake => withdraw_stake::WithdrawStake,
            FlashTradePerpsInstruction::WithdrawToken => withdraw_token::WithdrawToken,
            FlashTradePerpsInstruction::CollectRebate => collect_rebate::CollectRebate,
            FlashTradePerpsInstruction::CollectRevenue => collect_revenue::CollectRevenue,
            FlashTradePerpsInstruction::CollectStakeFees => collect_stake_fees::CollectStakeFees,
            FlashTradePerpsInstruction::CollectTokenReward => collect_token_reward::CollectTokenReward,
            FlashTradePerpsInstruction::UnstakeRequest => unstake_request::UnstakeRequest,
            FlashTradePerpsInstruction::UnstakeTokenInstant => unstake_token_instant::UnstakeTokenInstant,
            FlashTradePerpsInstruction::UnstakeTokenRequest => unstake_token_request::UnstakeTokenRequest,
            FlashTradePerpsInstruction::MigrateFlp => migrate_flp::MigrateFlp,
            FlashTradePerpsInstruction::MigrateStake => migrate_stake::MigrateStake,
            FlashTradePerpsInstruction::MoveProtocolFees => move_protocol_fees::MoveProtocolFees,
            FlashTradePerpsInstruction::CompoundFees => compound_fees::CompoundFees,
            FlashTradePerpsInstruction::RemoveCompoundingLiquidity => remove_compounding_liquidity::RemoveCompoundingLiquidity,
            FlashTradePerpsInstruction::CreateReferral => create_referral::CreateReferral,
            FlashTradePerpsInstruction::OpenPosition => open_position::OpenPosition,
            FlashTradePerpsInstruction::AddCollateral => add_collateral::AddCollateral,
            FlashTradePerpsInstruction::RemoveCollateralAndSwap => remove_collateral_and_swap::RemoveCollateralAndSwap,
            FlashTradePerpsInstruction::RemoveCollateral => remove_collateral::RemoveCollateral,
            FlashTradePerpsInstruction::IncreaseSize => increase_size::IncreaseSize,
            FlashTradePerpsInstruction::DecreaseSize => decrease_size::DecreaseSize,
            FlashTradePerpsInstruction::CancelAllTriggerOrders => cancel_all_trigger_orders::CancelAllTriggerOrders,
            FlashTradePerpsInstruction::CancelTriggerOrder => cancel_trigger_order::CancelTriggerOrder,
            FlashTradePerpsInstruction::CancelUnstakeTokenRequest => cancel_unstake_token_request::CancelUnstakeTokenRequest,
            FlashTradePerpsInstruction::ClosePosition => close_position::ClosePosition,
            FlashTradePerpsInstruction::ExecuteLimitWithSwap => execute_limit_with_swap::ExecuteLimitWithSwap,
            FlashTradePerpsInstruction::ExecuteLimitOrder => execute_limit_order::ExecuteLimitOrder,
            FlashTradePerpsInstruction::PlaceLimitOrder => place_limit_order::PlaceLimitOrder,
            FlashTradePerpsInstruction::EditLimitOrder => edit_limit_order::EditLimitOrder,
            FlashTradePerpsInstruction::EditTriggerOrder => edit_trigger_order::EditTriggerOrder,
            FlashTradePerpsInstruction::PlaceTriggerOrder => place_trigger_order::PlaceTriggerOrder,
            FlashTradePerpsInstruction::ExecuteTriggerWithSwap => execute_trigger_with_swap::ExecuteTriggerWithSwap,
            FlashTradePerpsInstruction::ExecuteTriggerOrder => execute_trigger_order::ExecuteTriggerOrder,
            FlashTradePerpsInstruction::Liquidate => liquidate::Liquidate,
            FlashTradePerpsInstruction::GetAddCompoundingLiquidityAmountAndFee => get_add_compounding_liquidity_amount_and_fee::GetAddCompoundingLiquidityAmountAndFee,
            FlashTradePerpsInstruction::GetAddLiquidityAmountAndFee => get_add_liquidity_amount_and_fee::GetAddLiquidityAmountAndFee,
            FlashTradePerpsInstruction::GetRemoveCompoundingLiquidityAmountAndFee => get_remove_compounding_liquidity_amount_and_fee::GetRemoveCompoundingLiquidityAmountAndFee,
            FlashTradePerpsInstruction::GetRemoveLiquidityAmountAndFee => get_remove_liquidity_amount_and_fee::GetRemoveLiquidityAmountAndFee,
            FlashTradePerpsInstruction::GetEntryPriceAndFee => get_entry_price_and_fee::GetEntryPriceAndFee,
            FlashTradePerpsInstruction::GetExitPriceAndFee => get_exit_price_and_fee::GetExitPriceAndFee,
            FlashTradePerpsInstruction::GetPnl => get_pnl::GetPnl,
            FlashTradePerpsInstruction::GetPositionData => get_position_data::GetPositionData,
            FlashTradePerpsInstruction::GetLiquidationState => get_liquidation_state::GetLiquidationState,
            FlashTradePerpsInstruction::GetLiquidationPrice => get_liquidation_price::GetLiquidationPrice,
            FlashTradePerpsInstruction::GetOraclePrice => get_oracle_price::GetOraclePrice,
            FlashTradePerpsInstruction::GetSwapAmountAndFees => get_swap_amount_and_fees::GetSwapAmountAndFees,
            FlashTradePerpsInstruction::GetAssetsUnderManagement => get_assets_under_management::GetAssetsUnderManagement,
            FlashTradePerpsInstruction::GetCompoundingTokenData => get_compounding_token_data::GetCompoundingTokenData,
            FlashTradePerpsInstruction::GetLpTokenPrice => get_lp_token_price::GetLpTokenPrice,
            FlashTradePerpsInstruction::GetCompoundingTokenPrice => get_compounding_token_price::GetCompoundingTokenPrice,
            FlashTradePerpsInstruction::RenameFlp => rename_flp::RenameFlp,
            FlashTradePerpsInstruction::AddCollateralLogEvent => add_collateral_log_event::AddCollateralLogEvent,
            FlashTradePerpsInstruction::AddCollateralLogV2Event => add_collateral_log_v2_event::AddCollateralLogV2Event,
            FlashTradePerpsInstruction::AddCollateralLogV3Event => add_collateral_log_v3_event::AddCollateralLogV3Event,
            FlashTradePerpsInstruction::AddCollateralLogUsDv1Event => add_collateral_log_us_dv1_event::AddCollateralLogUsDv1Event,
            FlashTradePerpsInstruction::AddCompoundingLiquidityLogEvent => add_compounding_liquidity_log_event::AddCompoundingLiquidityLogEvent,
            FlashTradePerpsInstruction::AddLiquidityAndStakeLogEvent => add_liquidity_and_stake_log_event::AddLiquidityAndStakeLogEvent,
            FlashTradePerpsInstruction::AddLiquidityLogEvent => add_liquidity_log_event::AddLiquidityLogEvent,
            FlashTradePerpsInstruction::AddLiquidityLogV2Event => add_liquidity_log_v2_event::AddLiquidityLogV2Event,
            FlashTradePerpsInstruction::CancelTriggerOrderLogEvent => cancel_trigger_order_log_event::CancelTriggerOrderLogEvent,
            FlashTradePerpsInstruction::CancelUnstakeTokenRequestLogEvent => cancel_unstake_token_request_log_event::CancelUnstakeTokenRequestLogEvent,
            FlashTradePerpsInstruction::CloseAndSwapLogEvent => close_and_swap_log_event::CloseAndSwapLogEvent,
            FlashTradePerpsInstruction::CloseAndSwapLogUsDv1Event => close_and_swap_log_us_dv1_event::CloseAndSwapLogUsDv1Event,
            FlashTradePerpsInstruction::ClosePositionLogEvent => close_position_log_event::ClosePositionLogEvent,
            FlashTradePerpsInstruction::ClosePositionLogV2Event => close_position_log_v2_event::ClosePositionLogV2Event,
            FlashTradePerpsInstruction::ClosePositionLogV3Event => close_position_log_v3_event::ClosePositionLogV3Event,
            FlashTradePerpsInstruction::ClosePositionLogUsDv1Event => close_position_log_us_dv1_event::ClosePositionLogUsDv1Event,
            FlashTradePerpsInstruction::CollectRebateLogEvent => collect_rebate_log_event::CollectRebateLogEvent,
            FlashTradePerpsInstruction::CollectRevenueLogEvent => collect_revenue_log_event::CollectRevenueLogEvent,
            FlashTradePerpsInstruction::CollectStakeRewardLogEvent => collect_stake_reward_log_event::CollectStakeRewardLogEvent,
            FlashTradePerpsInstruction::CollectStakeRewardLogV2Event => collect_stake_reward_log_v2_event::CollectStakeRewardLogV2Event,
            FlashTradePerpsInstruction::CollectTokenRewardLogEvent => collect_token_reward_log_event::CollectTokenRewardLogEvent,
            FlashTradePerpsInstruction::CompoundingFeesLogEvent => compounding_fees_log_event::CompoundingFeesLogEvent,
            FlashTradePerpsInstruction::DecreaseSizeLogEvent => decrease_size_log_event::DecreaseSizeLogEvent,
            FlashTradePerpsInstruction::DecreaseSizeLogV2Event => decrease_size_log_v2_event::DecreaseSizeLogV2Event,
            FlashTradePerpsInstruction::DecreaseSizeLogV3Event => decrease_size_log_v3_event::DecreaseSizeLogV3Event,
            FlashTradePerpsInstruction::DecreaseSizeLogUsDv1Event => decrease_size_log_us_dv1_event::DecreaseSizeLogUsDv1Event,
            FlashTradePerpsInstruction::DepositStakeLogEvent => deposit_stake_log_event::DepositStakeLogEvent,
            FlashTradePerpsInstruction::DepositTokenStakeLogEvent => deposit_token_stake_log_event::DepositTokenStakeLogEvent,
            FlashTradePerpsInstruction::DistributeTokenRewardLogEvent => distribute_token_reward_log_event::DistributeTokenRewardLogEvent,
            FlashTradePerpsInstruction::EditLimitOrderLogEvent => edit_limit_order_log_event::EditLimitOrderLogEvent,
            FlashTradePerpsInstruction::CancelLimitOrderLogEvent => cancel_limit_order_log_event::CancelLimitOrderLogEvent,
            FlashTradePerpsInstruction::EditTriggerOrderLogEvent => edit_trigger_order_log_event::EditTriggerOrderLogEvent,
            FlashTradePerpsInstruction::ExecuteLimitOrderLogEvent => execute_limit_order_log_event::ExecuteLimitOrderLogEvent,
            FlashTradePerpsInstruction::ExecuteLimitOrderLogV2Event => execute_limit_order_log_v2_event::ExecuteLimitOrderLogV2Event,
            FlashTradePerpsInstruction::ExecuteLimitOrderLogUsDv1Event => execute_limit_order_log_us_dv1_event::ExecuteLimitOrderLogUsDv1Event,
            FlashTradePerpsInstruction::ExecuteLimitWithSwapLogEvent => execute_limit_with_swap_log_event::ExecuteLimitWithSwapLogEvent,
            FlashTradePerpsInstruction::ExecuteLimitWithSwapLogV2Event => execute_limit_with_swap_log_v2_event::ExecuteLimitWithSwapLogV2Event,
            FlashTradePerpsInstruction::ExecuteLimitWithSwapLogUsDv1Event => execute_limit_with_swap_log_us_dv1_event::ExecuteLimitWithSwapLogUsDv1Event,
            FlashTradePerpsInstruction::ExecuteTriggerOrderLogEvent => execute_trigger_order_log_event::ExecuteTriggerOrderLogEvent,
            FlashTradePerpsInstruction::ExecuteTriggerOrderLogUsDv1Event => execute_trigger_order_log_us_dv1_event::ExecuteTriggerOrderLogUsDv1Event,
            FlashTradePerpsInstruction::ExecuteTriggerWithSwapLogEvent => execute_trigger_with_swap_log_event::ExecuteTriggerWithSwapLogEvent,
            FlashTradePerpsInstruction::ExecuteTriggerWithSwapLogUsDv1Event => execute_trigger_with_swap_log_us_dv1_event::ExecuteTriggerWithSwapLogUsDv1Event,
            FlashTradePerpsInstruction::IncreaseSizeLogEvent => increase_size_log_event::IncreaseSizeLogEvent,
            FlashTradePerpsInstruction::IncreaseSizeLogV2Event => increase_size_log_v2_event::IncreaseSizeLogV2Event,
            FlashTradePerpsInstruction::IncreaseSizeLogV3Event => increase_size_log_v3_event::IncreaseSizeLogV3Event,
            FlashTradePerpsInstruction::IncreaseSizeLogV4Event => increase_size_log_v4_event::IncreaseSizeLogV4Event,
            FlashTradePerpsInstruction::IncreaseSizeLogUsDv1Event => increase_size_log_us_dv1_event::IncreaseSizeLogUsDv1Event,
            FlashTradePerpsInstruction::LiquidateLogEvent => liquidate_log_event::LiquidateLogEvent,
            FlashTradePerpsInstruction::LiquidateLogV2Event => liquidate_log_v2_event::LiquidateLogV2Event,
            FlashTradePerpsInstruction::LiquidateLogV3Event => liquidate_log_v3_event::LiquidateLogV3Event,
            FlashTradePerpsInstruction::LiquidateLogUsDv1Event => liquidate_log_us_dv1_event::LiquidateLogUsDv1Event,
            FlashTradePerpsInstruction::MigrateFlpLogEvent => migrate_flp_log_event::MigrateFlpLogEvent,
            FlashTradePerpsInstruction::MigrateStakeLogEvent => migrate_stake_log_event::MigrateStakeLogEvent,
            FlashTradePerpsInstruction::MoveProtocolFeesLogEvent => move_protocol_fees_log_event::MoveProtocolFeesLogEvent,
            FlashTradePerpsInstruction::OpenPositionLogEvent => open_position_log_event::OpenPositionLogEvent,
            FlashTradePerpsInstruction::OpenPositionLogV2Event => open_position_log_v2_event::OpenPositionLogV2Event,
            FlashTradePerpsInstruction::OpenPositionLogV3Event => open_position_log_v3_event::OpenPositionLogV3Event,
            FlashTradePerpsInstruction::OpenPositionLogV4Event => open_position_log_v4_event::OpenPositionLogV4Event,
            FlashTradePerpsInstruction::OpenPositionLogUsDv1Event => open_position_log_us_dv1_event::OpenPositionLogUsDv1Event,
            FlashTradePerpsInstruction::PlaceLimitOrderLogEvent => place_limit_order_log_event::PlaceLimitOrderLogEvent,
            FlashTradePerpsInstruction::PlaceTriggerOrderLogEvent => place_trigger_order_log_event::PlaceTriggerOrderLogEvent,
            FlashTradePerpsInstruction::RefreshStakeLogEvent => refresh_stake_log_event::RefreshStakeLogEvent,
            FlashTradePerpsInstruction::RefreshStakeUserLogEvent => refresh_stake_user_log_event::RefreshStakeUserLogEvent,
            FlashTradePerpsInstruction::RemoveCollateralAndSwapLogEvent => remove_collateral_and_swap_log_event::RemoveCollateralAndSwapLogEvent,
            FlashTradePerpsInstruction::RemoveCollateralAndSwapLogUsDv1Event => remove_collateral_and_swap_log_us_dv1_event::RemoveCollateralAndSwapLogUsDv1Event,
            FlashTradePerpsInstruction::RemoveCollateralLogEvent => remove_collateral_log_event::RemoveCollateralLogEvent,
            FlashTradePerpsInstruction::RemoveCollateralLogV2Event => remove_collateral_log_v2_event::RemoveCollateralLogV2Event,
            FlashTradePerpsInstruction::RemoveCollateralLogV3Event => remove_collateral_log_v3_event::RemoveCollateralLogV3Event,
            FlashTradePerpsInstruction::RemoveCollateralLogUsDv1Event => remove_collateral_log_us_dv1_event::RemoveCollateralLogUsDv1Event,
            FlashTradePerpsInstruction::RemoveCompoundingLiquidityLogEvent => remove_compounding_liquidity_log_event::RemoveCompoundingLiquidityLogEvent,
            FlashTradePerpsInstruction::RemoveLiquidityLogEvent => remove_liquidity_log_event::RemoveLiquidityLogEvent,
            FlashTradePerpsInstruction::RemoveLiquidityLogV2Event => remove_liquidity_log_v2_event::RemoveLiquidityLogV2Event,
            FlashTradePerpsInstruction::SetTokenRewardLogEvent => set_token_reward_log_event::SetTokenRewardLogEvent,
            FlashTradePerpsInstruction::SettleRebatesLogEvent => settle_rebates_log_event::SettleRebatesLogEvent,
            FlashTradePerpsInstruction::SwapAndAddCollateralLogEvent => swap_and_add_collateral_log_event::SwapAndAddCollateralLogEvent,
            FlashTradePerpsInstruction::SwapAndAddCollateralLogUsDv1Event => swap_and_add_collateral_log_us_dv1_event::SwapAndAddCollateralLogUsDv1Event,
            FlashTradePerpsInstruction::SwapAndOpenLogEvent => swap_and_open_log_event::SwapAndOpenLogEvent,
            FlashTradePerpsInstruction::SwapAndOpenLogV2Event => swap_and_open_log_v2_event::SwapAndOpenLogV2Event,
            FlashTradePerpsInstruction::SwapAndOpenLogUsDv1Event => swap_and_open_log_us_dv1_event::SwapAndOpenLogUsDv1Event,
            FlashTradePerpsInstruction::SwapFeeInternalLogEvent => swap_fee_internal_log_event::SwapFeeInternalLogEvent,
            FlashTradePerpsInstruction::SwapFeeInternalLogV2Event => swap_fee_internal_log_v2_event::SwapFeeInternalLogV2Event,
            FlashTradePerpsInstruction::SwapFeeInternalLogV3Event => swap_fee_internal_log_v3_event::SwapFeeInternalLogV3Event,
            FlashTradePerpsInstruction::SwapLogEvent => swap_log_event::SwapLogEvent,
            FlashTradePerpsInstruction::SwapLogV2Event => swap_log_v2_event::SwapLogV2Event,
            FlashTradePerpsInstruction::UnstakeInstantLogEvent => unstake_instant_log_event::UnstakeInstantLogEvent,
            FlashTradePerpsInstruction::UnstakeRequestLogEvent => unstake_request_log_event::UnstakeRequestLogEvent,
            FlashTradePerpsInstruction::UnstakeTokenInstantLogEvent => unstake_token_instant_log_event::UnstakeTokenInstantLogEvent,
            FlashTradePerpsInstruction::UnstakeTokenRequestLogEvent => unstake_token_request_log_event::UnstakeTokenRequestLogEvent,
            FlashTradePerpsInstruction::WithdrawStakeLogEvent => withdraw_stake_log_event::WithdrawStakeLogEvent,
            FlashTradePerpsInstruction::WithdrawTokenLogEvent => withdraw_token_log_event::WithdrawTokenLogEvent,
            FlashTradePerpsInstruction::MigratePositionLogEvent => migrate_position_log_event::MigratePositionLogEvent,
            FlashTradePerpsInstruction::BurnAndClaimLogEvent => burn_and_claim_log_event::BurnAndClaimLogEvent,
            FlashTradePerpsInstruction::BurnAndStakeLogEvent => burn_and_stake_log_event::BurnAndStakeLogEvent,
            FlashTradePerpsInstruction::VoltagePointsLogEvent => voltage_points_log_event::VoltagePointsLogEvent,
            FlashTradePerpsInstruction::ReferralRebateLogEvent => referral_rebate_log_event::ReferralRebateLogEvent,
        )
    }
}
