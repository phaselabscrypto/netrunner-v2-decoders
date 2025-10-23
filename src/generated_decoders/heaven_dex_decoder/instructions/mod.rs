use super::HeavenDexDecoder;
pub mod admin_borrow_sol;
pub mod admin_claim_msol;
pub mod admin_claim_staking_rewards;
pub mod admin_claim_standard_creator_trading_fees;
pub mod admin_deposit_msol;
pub mod admin_mint_msol;
pub mod admin_repay_sol;
pub mod admin_set_transfer_fee;
pub mod admin_unstake_msol;
pub mod admin_update_standard_liquidity_pool_state;
pub mod admin_withdraw_msol;
pub mod admin_withdraw_transfer_fee;
pub mod buy;
pub mod claim_standard_creator_trading_fee_protocol_fees;
pub mod claim_standard_creator_trading_fees;
pub mod claim_standard_protocol_trading_fees;
pub mod claim_standard_reflection_trading_fees;
pub mod close_protocol_lookup_table;
pub mod create_liquidity_pool_event;
pub mod create_or_update_protocol_fee_admin;
pub mod create_or_update_protocol_owner;
pub mod create_or_update_protocol_staking_admin;
pub mod create_protocol_config;
pub mod create_protocol_lookup_table;
pub mod create_standard_liquidity_pool;
pub mod create_standard_liquidity_pool_event;
pub mod creating_liquidity_pool_event;
pub mod deactivate_protocol_lookup_table;
pub mod extend_protocol_lookup_table;
pub mod initialize_protocol_lending;
pub mod migration_event;
pub mod remaining_accounts_stub;
pub mod sell;
pub mod set_protocol_slot_fees;
pub mod trade_event;
pub mod update_allow_create_pool;
pub mod update_creator_trading_fee_receiver;
pub mod update_protocol_config;
pub mod user_defined_event;

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
pub enum HeavenDexInstruction {
    AdminBorrowSol(admin_borrow_sol::AdminBorrowSol),
    AdminClaimMsol(admin_claim_msol::AdminClaimMsol),
    AdminClaimStakingRewards(admin_claim_staking_rewards::AdminClaimStakingRewards),
    AdminClaimStandardCreatorTradingFees(admin_claim_standard_creator_trading_fees::AdminClaimStandardCreatorTradingFees),
    AdminDepositMsol(admin_deposit_msol::AdminDepositMsol),
    AdminMintMsol(admin_mint_msol::AdminMintMsol),
    AdminRepaySol(admin_repay_sol::AdminRepaySol),
    AdminSetTransferFee(admin_set_transfer_fee::AdminSetTransferFee),
    AdminUnstakeMsol(admin_unstake_msol::AdminUnstakeMsol),
    AdminUpdateStandardLiquidityPoolState(admin_update_standard_liquidity_pool_state::AdminUpdateStandardLiquidityPoolState),
    AdminWithdrawMsol(admin_withdraw_msol::AdminWithdrawMsol),
    AdminWithdrawTransferFee(admin_withdraw_transfer_fee::AdminWithdrawTransferFee),
    Buy(buy::Buy),
    ClaimStandardCreatorTradingFeeProtocolFees(claim_standard_creator_trading_fee_protocol_fees::ClaimStandardCreatorTradingFeeProtocolFees),
    ClaimStandardCreatorTradingFees(claim_standard_creator_trading_fees::ClaimStandardCreatorTradingFees),
    ClaimStandardProtocolTradingFees(claim_standard_protocol_trading_fees::ClaimStandardProtocolTradingFees),
    ClaimStandardReflectionTradingFees(claim_standard_reflection_trading_fees::ClaimStandardReflectionTradingFees),
    CloseProtocolLookupTable(close_protocol_lookup_table::CloseProtocolLookupTable),
    CreateOrUpdateProtocolFeeAdmin(create_or_update_protocol_fee_admin::CreateOrUpdateProtocolFeeAdmin),
    CreateOrUpdateProtocolOwner(create_or_update_protocol_owner::CreateOrUpdateProtocolOwner),
    CreateOrUpdateProtocolStakingAdmin(create_or_update_protocol_staking_admin::CreateOrUpdateProtocolStakingAdmin),
    CreateProtocolConfig(create_protocol_config::CreateProtocolConfig),
    CreateProtocolLookupTable(create_protocol_lookup_table::CreateProtocolLookupTable),
    CreateStandardLiquidityPool(create_standard_liquidity_pool::CreateStandardLiquidityPool),
    DeactivateProtocolLookupTable(deactivate_protocol_lookup_table::DeactivateProtocolLookupTable),
    ExtendProtocolLookupTable(extend_protocol_lookup_table::ExtendProtocolLookupTable),
    InitializeProtocolLending(initialize_protocol_lending::InitializeProtocolLending),
    RemainingAccountsStub(remaining_accounts_stub::RemainingAccountsStub),
    Sell(sell::Sell),
    SetProtocolSlotFees(set_protocol_slot_fees::SetProtocolSlotFees),
    UpdateAllowCreatePool(update_allow_create_pool::UpdateAllowCreatePool),
    UpdateCreatorTradingFeeReceiver(update_creator_trading_fee_receiver::UpdateCreatorTradingFeeReceiver),
    UpdateProtocolConfig(update_protocol_config::UpdateProtocolConfig),
    CreateLiquidityPoolEvent(create_liquidity_pool_event::CreateLiquidityPoolEvent),
    CreateStandardLiquidityPoolEvent(create_standard_liquidity_pool_event::CreateStandardLiquidityPoolEvent),
    CreatingLiquidityPoolEvent(creating_liquidity_pool_event::CreatingLiquidityPoolEvent),
    MigrationEvent(migration_event::MigrationEvent),
    TradeEvent(trade_event::TradeEvent),
    UserDefinedEvent(user_defined_event::UserDefinedEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for HeavenDexDecoder {
    type InstructionType = HeavenDexInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            HeavenDexInstruction::AdminBorrowSol => admin_borrow_sol::AdminBorrowSol,
            HeavenDexInstruction::AdminClaimMsol => admin_claim_msol::AdminClaimMsol,
            HeavenDexInstruction::AdminClaimStakingRewards => admin_claim_staking_rewards::AdminClaimStakingRewards,
            HeavenDexInstruction::AdminClaimStandardCreatorTradingFees => admin_claim_standard_creator_trading_fees::AdminClaimStandardCreatorTradingFees,
            HeavenDexInstruction::AdminDepositMsol => admin_deposit_msol::AdminDepositMsol,
            HeavenDexInstruction::AdminMintMsol => admin_mint_msol::AdminMintMsol,
            HeavenDexInstruction::AdminRepaySol => admin_repay_sol::AdminRepaySol,
            HeavenDexInstruction::AdminSetTransferFee => admin_set_transfer_fee::AdminSetTransferFee,
            HeavenDexInstruction::AdminUnstakeMsol => admin_unstake_msol::AdminUnstakeMsol,
            HeavenDexInstruction::AdminUpdateStandardLiquidityPoolState => admin_update_standard_liquidity_pool_state::AdminUpdateStandardLiquidityPoolState,
            HeavenDexInstruction::AdminWithdrawMsol => admin_withdraw_msol::AdminWithdrawMsol,
            HeavenDexInstruction::AdminWithdrawTransferFee => admin_withdraw_transfer_fee::AdminWithdrawTransferFee,
            HeavenDexInstruction::Buy => buy::Buy,
            HeavenDexInstruction::ClaimStandardCreatorTradingFeeProtocolFees => claim_standard_creator_trading_fee_protocol_fees::ClaimStandardCreatorTradingFeeProtocolFees,
            HeavenDexInstruction::ClaimStandardCreatorTradingFees => claim_standard_creator_trading_fees::ClaimStandardCreatorTradingFees,
            HeavenDexInstruction::ClaimStandardProtocolTradingFees => claim_standard_protocol_trading_fees::ClaimStandardProtocolTradingFees,
            HeavenDexInstruction::ClaimStandardReflectionTradingFees => claim_standard_reflection_trading_fees::ClaimStandardReflectionTradingFees,
            HeavenDexInstruction::CloseProtocolLookupTable => close_protocol_lookup_table::CloseProtocolLookupTable,
            HeavenDexInstruction::CreateOrUpdateProtocolFeeAdmin => create_or_update_protocol_fee_admin::CreateOrUpdateProtocolFeeAdmin,
            HeavenDexInstruction::CreateOrUpdateProtocolOwner => create_or_update_protocol_owner::CreateOrUpdateProtocolOwner,
            HeavenDexInstruction::CreateOrUpdateProtocolStakingAdmin => create_or_update_protocol_staking_admin::CreateOrUpdateProtocolStakingAdmin,
            HeavenDexInstruction::CreateProtocolConfig => create_protocol_config::CreateProtocolConfig,
            HeavenDexInstruction::CreateProtocolLookupTable => create_protocol_lookup_table::CreateProtocolLookupTable,
            HeavenDexInstruction::CreateStandardLiquidityPool => create_standard_liquidity_pool::CreateStandardLiquidityPool,
            HeavenDexInstruction::DeactivateProtocolLookupTable => deactivate_protocol_lookup_table::DeactivateProtocolLookupTable,
            HeavenDexInstruction::ExtendProtocolLookupTable => extend_protocol_lookup_table::ExtendProtocolLookupTable,
            HeavenDexInstruction::InitializeProtocolLending => initialize_protocol_lending::InitializeProtocolLending,
            HeavenDexInstruction::RemainingAccountsStub => remaining_accounts_stub::RemainingAccountsStub,
            HeavenDexInstruction::Sell => sell::Sell,
            HeavenDexInstruction::SetProtocolSlotFees => set_protocol_slot_fees::SetProtocolSlotFees,
            HeavenDexInstruction::UpdateAllowCreatePool => update_allow_create_pool::UpdateAllowCreatePool,
            HeavenDexInstruction::UpdateCreatorTradingFeeReceiver => update_creator_trading_fee_receiver::UpdateCreatorTradingFeeReceiver,
            HeavenDexInstruction::UpdateProtocolConfig => update_protocol_config::UpdateProtocolConfig,
            HeavenDexInstruction::CreateLiquidityPoolEvent => create_liquidity_pool_event::CreateLiquidityPoolEvent,
            HeavenDexInstruction::CreateStandardLiquidityPoolEvent => create_standard_liquidity_pool_event::CreateStandardLiquidityPoolEvent,
            HeavenDexInstruction::CreatingLiquidityPoolEvent => creating_liquidity_pool_event::CreatingLiquidityPoolEvent,
            HeavenDexInstruction::MigrationEvent => migration_event::MigrationEvent,
            HeavenDexInstruction::TradeEvent => trade_event::TradeEvent,
            HeavenDexInstruction::UserDefinedEvent => user_defined_event::UserDefinedEvent,
        )
    }
}
