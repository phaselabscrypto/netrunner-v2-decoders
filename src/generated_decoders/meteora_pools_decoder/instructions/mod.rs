use super::AmmDecoder;
pub mod add_balance_liquidity;
pub mod add_imbalance_liquidity;
pub mod add_liquidity_event;
pub mod bootstrap_liquidity;
pub mod bootstrap_liquidity_event;
pub mod claim_fee;
pub mod claim_fee_event;
pub mod close_config;
pub mod close_config_event;
pub mod create_config;
pub mod create_config_event;
pub mod create_lock_escrow;
pub mod create_lock_escrow_event;
pub mod create_mint_metadata;
pub mod enable_or_disable_pool;
pub mod get_pool_info;
pub mod initialize_customizable_permissionless_constant_product_pool;
pub mod initialize_permissioned_pool;
pub mod initialize_permissionless_constant_product_pool_with_config;
pub mod initialize_permissionless_constant_product_pool_with_config2;
pub mod initialize_permissionless_pool;
pub mod initialize_permissionless_pool_with_fee_tier;
pub mod lock;
pub mod lock_event;
pub mod migrate_fee_account_event;
pub mod override_curve_param;
pub mod override_curve_param_event;
pub mod partner_claim_fee;
pub mod partner_claim_fees_event;
pub mod pool_created_event;
pub mod pool_enabled_event;
pub mod pool_info_event;
pub mod remove_balance_liquidity;
pub mod remove_liquidity_event;
pub mod remove_liquidity_single_side;
pub mod set_pool_fees;
pub mod set_pool_fees_event;
pub mod set_whitelisted_vault;
pub mod swap;
pub mod swap_event;
pub mod transfer_admin_event;
pub mod update_activation_point;
pub mod withdraw_protocol_fees;
pub mod withdraw_protocol_fees_event;

#[derive(
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone,
)]
pub enum AmmInstruction {
    InitializePermissionedPool(initialize_permissioned_pool::InitializePermissionedPool),
    InitializePermissionlessPool(initialize_permissionless_pool::InitializePermissionlessPool),
    InitializePermissionlessPoolWithFeeTier(initialize_permissionless_pool_with_fee_tier::InitializePermissionlessPoolWithFeeTier),
    EnableOrDisablePool(enable_or_disable_pool::EnableOrDisablePool),
    Swap(swap::Swap),
    RemoveLiquiditySingleSide(remove_liquidity_single_side::RemoveLiquiditySingleSide),
    AddImbalanceLiquidity(add_imbalance_liquidity::AddImbalanceLiquidity),
    RemoveBalanceLiquidity(remove_balance_liquidity::RemoveBalanceLiquidity),
    AddBalanceLiquidity(add_balance_liquidity::AddBalanceLiquidity),
    SetPoolFees(set_pool_fees::SetPoolFees),
    OverrideCurveParam(override_curve_param::OverrideCurveParam),
    GetPoolInfo(get_pool_info::GetPoolInfo),
    BootstrapLiquidity(bootstrap_liquidity::BootstrapLiquidity),
    CreateMintMetadata(create_mint_metadata::CreateMintMetadata),
    CreateLockEscrow(create_lock_escrow::CreateLockEscrow),
    Lock(lock::Lock),
    ClaimFee(claim_fee::ClaimFee),
    CreateConfig(create_config::CreateConfig),
    CloseConfig(close_config::CloseConfig),
    InitializePermissionlessConstantProductPoolWithConfig(initialize_permissionless_constant_product_pool_with_config::InitializePermissionlessConstantProductPoolWithConfig),
    InitializePermissionlessConstantProductPoolWithConfig2(initialize_permissionless_constant_product_pool_with_config2::InitializePermissionlessConstantProductPoolWithConfig2),
    InitializeCustomizablePermissionlessConstantProductPool(initialize_customizable_permissionless_constant_product_pool::InitializeCustomizablePermissionlessConstantProductPool),
    UpdateActivationPoint(update_activation_point::UpdateActivationPoint),
    WithdrawProtocolFees(withdraw_protocol_fees::WithdrawProtocolFees),
    SetWhitelistedVault(set_whitelisted_vault::SetWhitelistedVault),
    PartnerClaimFee(partner_claim_fee::PartnerClaimFee),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    BootstrapLiquidityEvent(bootstrap_liquidity_event::BootstrapLiquidityEvent),
    SwapEvent(swap_event::SwapEvent),
    SetPoolFeesEvent(set_pool_fees_event::SetPoolFeesEvent),
    PoolInfoEvent(pool_info_event::PoolInfoEvent),
    TransferAdminEvent(transfer_admin_event::TransferAdminEvent),
    OverrideCurveParamEvent(override_curve_param_event::OverrideCurveParamEvent),
    PoolCreatedEvent(pool_created_event::PoolCreatedEvent),
    PoolEnabledEvent(pool_enabled_event::PoolEnabledEvent),
    MigrateFeeAccountEvent(migrate_fee_account_event::MigrateFeeAccountEvent),
    CreateLockEscrowEvent(create_lock_escrow_event::CreateLockEscrowEvent),
    LockEvent(lock_event::LockEvent),
    ClaimFeeEvent(claim_fee_event::ClaimFeeEvent),
    CreateConfigEvent(create_config_event::CreateConfigEvent),
    CloseConfigEvent(close_config_event::CloseConfigEvent),
    WithdrawProtocolFeesEvent(withdraw_protocol_fees_event::WithdrawProtocolFeesEvent),
    PartnerClaimFeesEvent(partner_claim_fees_event::PartnerClaimFeesEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for AmmDecoder {
    type InstructionType = AmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            AmmInstruction::InitializePermissionedPool => initialize_permissioned_pool::InitializePermissionedPool,
            AmmInstruction::InitializePermissionlessPool => initialize_permissionless_pool::InitializePermissionlessPool,
            AmmInstruction::InitializePermissionlessPoolWithFeeTier => initialize_permissionless_pool_with_fee_tier::InitializePermissionlessPoolWithFeeTier,
            AmmInstruction::EnableOrDisablePool => enable_or_disable_pool::EnableOrDisablePool,
            AmmInstruction::Swap => swap::Swap,
            AmmInstruction::RemoveLiquiditySingleSide => remove_liquidity_single_side::RemoveLiquiditySingleSide,
            AmmInstruction::AddImbalanceLiquidity => add_imbalance_liquidity::AddImbalanceLiquidity,
            AmmInstruction::RemoveBalanceLiquidity => remove_balance_liquidity::RemoveBalanceLiquidity,
            AmmInstruction::AddBalanceLiquidity => add_balance_liquidity::AddBalanceLiquidity,
            AmmInstruction::SetPoolFees => set_pool_fees::SetPoolFees,
            AmmInstruction::OverrideCurveParam => override_curve_param::OverrideCurveParam,
            AmmInstruction::GetPoolInfo => get_pool_info::GetPoolInfo,
            AmmInstruction::BootstrapLiquidity => bootstrap_liquidity::BootstrapLiquidity,
            AmmInstruction::CreateMintMetadata => create_mint_metadata::CreateMintMetadata,
            AmmInstruction::CreateLockEscrow => create_lock_escrow::CreateLockEscrow,
            AmmInstruction::Lock => lock::Lock,
            AmmInstruction::ClaimFee => claim_fee::ClaimFee,
            AmmInstruction::CreateConfig => create_config::CreateConfig,
            AmmInstruction::CloseConfig => close_config::CloseConfig,
            AmmInstruction::InitializePermissionlessConstantProductPoolWithConfig => initialize_permissionless_constant_product_pool_with_config::InitializePermissionlessConstantProductPoolWithConfig,
            AmmInstruction::InitializePermissionlessConstantProductPoolWithConfig2 => initialize_permissionless_constant_product_pool_with_config2::InitializePermissionlessConstantProductPoolWithConfig2,
            AmmInstruction::InitializeCustomizablePermissionlessConstantProductPool => initialize_customizable_permissionless_constant_product_pool::InitializeCustomizablePermissionlessConstantProductPool,
            AmmInstruction::UpdateActivationPoint => update_activation_point::UpdateActivationPoint,
            AmmInstruction::WithdrawProtocolFees => withdraw_protocol_fees::WithdrawProtocolFees,
            AmmInstruction::SetWhitelistedVault => set_whitelisted_vault::SetWhitelistedVault,
            AmmInstruction::PartnerClaimFee => partner_claim_fee::PartnerClaimFee,
            AmmInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            AmmInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            AmmInstruction::BootstrapLiquidityEvent => bootstrap_liquidity_event::BootstrapLiquidityEvent,
            AmmInstruction::SwapEvent => swap_event::SwapEvent,
            AmmInstruction::SetPoolFeesEvent => set_pool_fees_event::SetPoolFeesEvent,
            AmmInstruction::PoolInfoEvent => pool_info_event::PoolInfoEvent,
            AmmInstruction::TransferAdminEvent => transfer_admin_event::TransferAdminEvent,
            AmmInstruction::OverrideCurveParamEvent => override_curve_param_event::OverrideCurveParamEvent,
            AmmInstruction::PoolCreatedEvent => pool_created_event::PoolCreatedEvent,
            AmmInstruction::PoolEnabledEvent => pool_enabled_event::PoolEnabledEvent,
            AmmInstruction::MigrateFeeAccountEvent => migrate_fee_account_event::MigrateFeeAccountEvent,
            AmmInstruction::CreateLockEscrowEvent => create_lock_escrow_event::CreateLockEscrowEvent,
            AmmInstruction::LockEvent => lock_event::LockEvent,
            AmmInstruction::ClaimFeeEvent => claim_fee_event::ClaimFeeEvent,
            AmmInstruction::CreateConfigEvent => create_config_event::CreateConfigEvent,
            AmmInstruction::CloseConfigEvent => close_config_event::CloseConfigEvent,
            AmmInstruction::WithdrawProtocolFeesEvent => withdraw_protocol_fees_event::WithdrawProtocolFeesEvent,
            AmmInstruction::PartnerClaimFeesEvent => partner_claim_fees_event::PartnerClaimFeesEvent,
        )
    }
}
