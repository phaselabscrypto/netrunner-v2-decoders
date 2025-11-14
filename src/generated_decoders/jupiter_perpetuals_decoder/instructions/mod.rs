use super::JupiterPerpetualsDecoder;
pub mod add_custody;
pub mod add_liquidity2;
pub mod add_liquidity_event;
pub mod add_pool;
pub mod borrow_from_custody;
pub mod borrow_from_custody_event;
pub mod close_borrow_position;
pub mod close_position_request;
pub mod close_position_request2;
pub mod close_position_request_event;
pub mod create_and_delegate_stake_account;
pub mod create_decrease_position_market_request;
pub mod create_decrease_position_request2;
pub mod create_increase_position_market_request;
pub mod create_position_request_event;
pub mod create_token_ledger;
pub mod create_token_metadata;
pub mod decrease_position4;
pub mod decrease_position_event;
pub mod decrease_position_post_swap_event;
pub mod decrease_position_with_internal_swap;
pub mod decrease_position_with_tpsl;
pub mod decrease_position_with_tpsl_and_internal_swap;
pub mod delegate_stake_event;
pub mod deposit_collateral_event;
pub mod deposit_collateral_for_borrows;
pub mod get_add_liquidity_amount_and_fee2;
pub mod get_assets_under_management2;
pub mod get_remove_liquidity_amount_and_fee2;
pub mod increase_position4;
pub mod increase_position_event;
pub mod increase_position_pre_swap;
pub mod increase_position_pre_swap_event;
pub mod increase_position_with_internal_swap;
pub mod init;
pub mod instant_create_limit_order;
pub mod instant_create_limit_order_event;
pub mod instant_create_tpsl;
pub mod instant_create_tpsl_event;
pub mod instant_decrease_position;
pub mod instant_decrease_position_event;
pub mod instant_increase_position;
pub mod instant_increase_position_event;
pub mod instant_increase_position_pre_swap;
pub mod instant_update_limit_order;
pub mod instant_update_tpsl;
pub mod instant_update_tpsl_event;
pub mod liquidate_borrow_position;
pub mod liquidate_borrow_position_event;
pub mod liquidate_full_position4;
pub mod liquidate_full_position_event;
pub mod operator_set_custody_config;
pub mod operator_set_pool_config;
pub mod pool_swap_event;
pub mod pool_swap_exact_out_event;
pub mod realloc_custody;
pub mod realloc_pool;
pub mod redeem_stake;
pub mod redeem_stake_event;
pub mod refresh_assets_under_management;
pub mod remove_liquidity2;
pub mod remove_liquidity_event;
pub mod repay_to_custody;
pub mod repay_to_custody_event;
pub mod set_custody_config;
pub mod set_max_global_sizes;
pub mod set_perpetuals_config;
pub mod set_pool_config;
pub mod set_test_time;
pub mod set_token_ledger;
pub mod swap2;
pub mod swap_with_token_ledger;
pub mod test_init;
pub mod transfer_admin;
pub mod unstake;
pub mod update_decrease_position_request2;
pub mod withdraw_collateral_event;
pub mod withdraw_collateral_for_borrows;
pub mod withdraw_fees2;
pub mod withdraw_stake;
pub mod withdraw_stake_event;

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
pub enum JupiterPerpetualsInstruction {
    Init(init::Init),
    AddPool(add_pool::AddPool),
    AddCustody(add_custody::AddCustody),
    SetCustodyConfig(set_custody_config::SetCustodyConfig),
    SetPoolConfig(set_pool_config::SetPoolConfig),
    SetPerpetualsConfig(set_perpetuals_config::SetPerpetualsConfig),
    TransferAdmin(transfer_admin::TransferAdmin),
    WithdrawFees2(withdraw_fees2::WithdrawFees2),
    CreateTokenMetadata(create_token_metadata::CreateTokenMetadata),
    CreateTokenLedger(create_token_ledger::CreateTokenLedger),
    ReallocCustody(realloc_custody::ReallocCustody),
    ReallocPool(realloc_pool::ReallocPool),
    CreateAndDelegateStakeAccount(create_and_delegate_stake_account::CreateAndDelegateStakeAccount),
    Unstake(unstake::Unstake),
    WithdrawStake(withdraw_stake::WithdrawStake),
    RedeemStake(redeem_stake::RedeemStake),
    OperatorSetCustodyConfig(operator_set_custody_config::OperatorSetCustodyConfig),
    OperatorSetPoolConfig(operator_set_pool_config::OperatorSetPoolConfig),
    TestInit(test_init::TestInit),
    SetTestTime(set_test_time::SetTestTime),
    SetTokenLedger(set_token_ledger::SetTokenLedger),
    Swap2(swap2::Swap2),
    SwapWithTokenLedger(swap_with_token_ledger::SwapWithTokenLedger),
    InstantIncreasePositionPreSwap(
        instant_increase_position_pre_swap::InstantIncreasePositionPreSwap,
    ),
    AddLiquidity2(add_liquidity2::AddLiquidity2),
    RemoveLiquidity2(remove_liquidity2::RemoveLiquidity2),
    CreateIncreasePositionMarketRequest(
        create_increase_position_market_request::CreateIncreasePositionMarketRequest,
    ),
    CreateDecreasePositionRequest2(
        create_decrease_position_request2::CreateDecreasePositionRequest2,
    ),
    CreateDecreasePositionMarketRequest(
        create_decrease_position_market_request::CreateDecreasePositionMarketRequest,
    ),
    UpdateDecreasePositionRequest2(
        update_decrease_position_request2::UpdateDecreasePositionRequest2,
    ),
    ClosePositionRequest(close_position_request::ClosePositionRequest),
    ClosePositionRequest2(close_position_request2::ClosePositionRequest2),
    IncreasePosition4(increase_position4::IncreasePosition4),
    IncreasePositionPreSwap(increase_position_pre_swap::IncreasePositionPreSwap),
    IncreasePositionWithInternalSwap(
        increase_position_with_internal_swap::IncreasePositionWithInternalSwap,
    ),
    DecreasePosition4(decrease_position4::DecreasePosition4),
    DecreasePositionWithInternalSwap(
        decrease_position_with_internal_swap::DecreasePositionWithInternalSwap,
    ),
    DecreasePositionWithTpsl(decrease_position_with_tpsl::DecreasePositionWithTpsl),
    DecreasePositionWithTpslAndInternalSwap(
        decrease_position_with_tpsl_and_internal_swap::DecreasePositionWithTpslAndInternalSwap,
    ),
    LiquidateFullPosition4(liquidate_full_position4::LiquidateFullPosition4),
    RefreshAssetsUnderManagement(refresh_assets_under_management::RefreshAssetsUnderManagement),
    SetMaxGlobalSizes(set_max_global_sizes::SetMaxGlobalSizes),
    InstantCreateTpsl(instant_create_tpsl::InstantCreateTpsl),
    InstantCreateLimitOrder(instant_create_limit_order::InstantCreateLimitOrder),
    InstantIncreasePosition(instant_increase_position::InstantIncreasePosition),
    InstantDecreasePosition(instant_decrease_position::InstantDecreasePosition),
    InstantUpdateLimitOrder(instant_update_limit_order::InstantUpdateLimitOrder),
    InstantUpdateTpsl(instant_update_tpsl::InstantUpdateTpsl),
    GetAddLiquidityAmountAndFee2(get_add_liquidity_amount_and_fee2::GetAddLiquidityAmountAndFee2),
    GetRemoveLiquidityAmountAndFee2(
        get_remove_liquidity_amount_and_fee2::GetRemoveLiquidityAmountAndFee2,
    ),
    GetAssetsUnderManagement2(get_assets_under_management2::GetAssetsUnderManagement2),
    BorrowFromCustody(borrow_from_custody::BorrowFromCustody),
    RepayToCustody(repay_to_custody::RepayToCustody),
    DepositCollateralForBorrows(deposit_collateral_for_borrows::DepositCollateralForBorrows),
    WithdrawCollateralForBorrows(withdraw_collateral_for_borrows::WithdrawCollateralForBorrows),
    LiquidateBorrowPosition(liquidate_borrow_position::LiquidateBorrowPosition),
    CloseBorrowPosition(close_borrow_position::CloseBorrowPosition),
    CreatePositionRequestEvent(create_position_request_event::CreatePositionRequestEvent),
    InstantCreateTpslEvent(instant_create_tpsl_event::InstantCreateTpslEvent),
    InstantUpdateTpslEvent(instant_update_tpsl_event::InstantUpdateTpslEvent),
    ClosePositionRequestEvent(close_position_request_event::ClosePositionRequestEvent),
    IncreasePositionEvent(increase_position_event::IncreasePositionEvent),
    IncreasePositionPreSwapEvent(increase_position_pre_swap_event::IncreasePositionPreSwapEvent),
    DecreasePositionEvent(decrease_position_event::DecreasePositionEvent),
    DecreasePositionPostSwapEvent(decrease_position_post_swap_event::DecreasePositionPostSwapEvent),
    LiquidateFullPositionEvent(liquidate_full_position_event::LiquidateFullPositionEvent),
    PoolSwapEvent(pool_swap_event::PoolSwapEvent),
    PoolSwapExactOutEvent(pool_swap_exact_out_event::PoolSwapExactOutEvent),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    InstantCreateLimitOrderEvent(instant_create_limit_order_event::InstantCreateLimitOrderEvent),
    InstantIncreasePositionEvent(instant_increase_position_event::InstantIncreasePositionEvent),
    InstantDecreasePositionEvent(instant_decrease_position_event::InstantDecreasePositionEvent),
    DepositCollateralEvent(deposit_collateral_event::DepositCollateralEvent),
    WithdrawCollateralEvent(withdraw_collateral_event::WithdrawCollateralEvent),
    BorrowFromCustodyEvent(borrow_from_custody_event::BorrowFromCustodyEvent),
    RepayToCustodyEvent(repay_to_custody_event::RepayToCustodyEvent),
    LiquidateBorrowPositionEvent(liquidate_borrow_position_event::LiquidateBorrowPositionEvent),
    RedeemStakeEvent(redeem_stake_event::RedeemStakeEvent),
    WithdrawStakeEvent(withdraw_stake_event::WithdrawStakeEvent),
    DelegateStakeEvent(delegate_stake_event::DelegateStakeEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JupiterPerpetualsDecoder {
    type InstructionType = JupiterPerpetualsInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterPerpetualsInstruction::Init => init::Init,
            JupiterPerpetualsInstruction::AddPool => add_pool::AddPool,
            JupiterPerpetualsInstruction::AddCustody => add_custody::AddCustody,
            JupiterPerpetualsInstruction::SetCustodyConfig => set_custody_config::SetCustodyConfig,
            JupiterPerpetualsInstruction::SetPoolConfig => set_pool_config::SetPoolConfig,
            JupiterPerpetualsInstruction::SetPerpetualsConfig => set_perpetuals_config::SetPerpetualsConfig,
            JupiterPerpetualsInstruction::TransferAdmin => transfer_admin::TransferAdmin,
            JupiterPerpetualsInstruction::WithdrawFees2 => withdraw_fees2::WithdrawFees2,
            JupiterPerpetualsInstruction::CreateTokenMetadata => create_token_metadata::CreateTokenMetadata,
            JupiterPerpetualsInstruction::CreateTokenLedger => create_token_ledger::CreateTokenLedger,
            JupiterPerpetualsInstruction::ReallocCustody => realloc_custody::ReallocCustody,
            JupiterPerpetualsInstruction::ReallocPool => realloc_pool::ReallocPool,
            JupiterPerpetualsInstruction::CreateAndDelegateStakeAccount => create_and_delegate_stake_account::CreateAndDelegateStakeAccount,
            JupiterPerpetualsInstruction::Unstake => unstake::Unstake,
            JupiterPerpetualsInstruction::WithdrawStake => withdraw_stake::WithdrawStake,
            JupiterPerpetualsInstruction::RedeemStake => redeem_stake::RedeemStake,
            JupiterPerpetualsInstruction::OperatorSetCustodyConfig => operator_set_custody_config::OperatorSetCustodyConfig,
            JupiterPerpetualsInstruction::OperatorSetPoolConfig => operator_set_pool_config::OperatorSetPoolConfig,
            JupiterPerpetualsInstruction::TestInit => test_init::TestInit,
            JupiterPerpetualsInstruction::SetTestTime => set_test_time::SetTestTime,
            JupiterPerpetualsInstruction::SetTokenLedger => set_token_ledger::SetTokenLedger,
            JupiterPerpetualsInstruction::Swap2 => swap2::Swap2,
            JupiterPerpetualsInstruction::SwapWithTokenLedger => swap_with_token_ledger::SwapWithTokenLedger,
            JupiterPerpetualsInstruction::InstantIncreasePositionPreSwap => instant_increase_position_pre_swap::InstantIncreasePositionPreSwap,
            JupiterPerpetualsInstruction::AddLiquidity2 => add_liquidity2::AddLiquidity2,
            JupiterPerpetualsInstruction::RemoveLiquidity2 => remove_liquidity2::RemoveLiquidity2,
            JupiterPerpetualsInstruction::CreateIncreasePositionMarketRequest => create_increase_position_market_request::CreateIncreasePositionMarketRequest,
            JupiterPerpetualsInstruction::CreateDecreasePositionRequest2 => create_decrease_position_request2::CreateDecreasePositionRequest2,
            JupiterPerpetualsInstruction::CreateDecreasePositionMarketRequest => create_decrease_position_market_request::CreateDecreasePositionMarketRequest,
            JupiterPerpetualsInstruction::UpdateDecreasePositionRequest2 => update_decrease_position_request2::UpdateDecreasePositionRequest2,
            JupiterPerpetualsInstruction::ClosePositionRequest => close_position_request::ClosePositionRequest,
            JupiterPerpetualsInstruction::ClosePositionRequest2 => close_position_request2::ClosePositionRequest2,
            JupiterPerpetualsInstruction::IncreasePosition4 => increase_position4::IncreasePosition4,
            JupiterPerpetualsInstruction::IncreasePositionPreSwap => increase_position_pre_swap::IncreasePositionPreSwap,
            JupiterPerpetualsInstruction::IncreasePositionWithInternalSwap => increase_position_with_internal_swap::IncreasePositionWithInternalSwap,
            JupiterPerpetualsInstruction::DecreasePosition4 => decrease_position4::DecreasePosition4,
            JupiterPerpetualsInstruction::DecreasePositionWithInternalSwap => decrease_position_with_internal_swap::DecreasePositionWithInternalSwap,
            JupiterPerpetualsInstruction::DecreasePositionWithTpsl => decrease_position_with_tpsl::DecreasePositionWithTpsl,
            JupiterPerpetualsInstruction::DecreasePositionWithTpslAndInternalSwap => decrease_position_with_tpsl_and_internal_swap::DecreasePositionWithTpslAndInternalSwap,
            JupiterPerpetualsInstruction::LiquidateFullPosition4 => liquidate_full_position4::LiquidateFullPosition4,
            JupiterPerpetualsInstruction::RefreshAssetsUnderManagement => refresh_assets_under_management::RefreshAssetsUnderManagement,
            JupiterPerpetualsInstruction::SetMaxGlobalSizes => set_max_global_sizes::SetMaxGlobalSizes,
            JupiterPerpetualsInstruction::InstantCreateTpsl => instant_create_tpsl::InstantCreateTpsl,
            JupiterPerpetualsInstruction::InstantCreateLimitOrder => instant_create_limit_order::InstantCreateLimitOrder,
            JupiterPerpetualsInstruction::InstantIncreasePosition => instant_increase_position::InstantIncreasePosition,
            JupiterPerpetualsInstruction::InstantDecreasePosition => instant_decrease_position::InstantDecreasePosition,
            JupiterPerpetualsInstruction::InstantUpdateLimitOrder => instant_update_limit_order::InstantUpdateLimitOrder,
            JupiterPerpetualsInstruction::InstantUpdateTpsl => instant_update_tpsl::InstantUpdateTpsl,
            JupiterPerpetualsInstruction::GetAddLiquidityAmountAndFee2 => get_add_liquidity_amount_and_fee2::GetAddLiquidityAmountAndFee2,
            JupiterPerpetualsInstruction::GetRemoveLiquidityAmountAndFee2 => get_remove_liquidity_amount_and_fee2::GetRemoveLiquidityAmountAndFee2,
            JupiterPerpetualsInstruction::GetAssetsUnderManagement2 => get_assets_under_management2::GetAssetsUnderManagement2,
            JupiterPerpetualsInstruction::BorrowFromCustody => borrow_from_custody::BorrowFromCustody,
            JupiterPerpetualsInstruction::RepayToCustody => repay_to_custody::RepayToCustody,
            JupiterPerpetualsInstruction::DepositCollateralForBorrows => deposit_collateral_for_borrows::DepositCollateralForBorrows,
            JupiterPerpetualsInstruction::WithdrawCollateralForBorrows => withdraw_collateral_for_borrows::WithdrawCollateralForBorrows,
            JupiterPerpetualsInstruction::LiquidateBorrowPosition => liquidate_borrow_position::LiquidateBorrowPosition,
            JupiterPerpetualsInstruction::CloseBorrowPosition => close_borrow_position::CloseBorrowPosition,
            JupiterPerpetualsInstruction::CreatePositionRequestEvent => create_position_request_event::CreatePositionRequestEvent,
            JupiterPerpetualsInstruction::InstantCreateTpslEvent => instant_create_tpsl_event::InstantCreateTpslEvent,
            JupiterPerpetualsInstruction::InstantUpdateTpslEvent => instant_update_tpsl_event::InstantUpdateTpslEvent,
            JupiterPerpetualsInstruction::ClosePositionRequestEvent => close_position_request_event::ClosePositionRequestEvent,
            JupiterPerpetualsInstruction::IncreasePositionEvent => increase_position_event::IncreasePositionEvent,
            JupiterPerpetualsInstruction::IncreasePositionPreSwapEvent => increase_position_pre_swap_event::IncreasePositionPreSwapEvent,
            JupiterPerpetualsInstruction::DecreasePositionEvent => decrease_position_event::DecreasePositionEvent,
            JupiterPerpetualsInstruction::DecreasePositionPostSwapEvent => decrease_position_post_swap_event::DecreasePositionPostSwapEvent,
            JupiterPerpetualsInstruction::LiquidateFullPositionEvent => liquidate_full_position_event::LiquidateFullPositionEvent,
            JupiterPerpetualsInstruction::PoolSwapEvent => pool_swap_event::PoolSwapEvent,
            JupiterPerpetualsInstruction::PoolSwapExactOutEvent => pool_swap_exact_out_event::PoolSwapExactOutEvent,
            JupiterPerpetualsInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            JupiterPerpetualsInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            JupiterPerpetualsInstruction::InstantCreateLimitOrderEvent => instant_create_limit_order_event::InstantCreateLimitOrderEvent,
            JupiterPerpetualsInstruction::InstantIncreasePositionEvent => instant_increase_position_event::InstantIncreasePositionEvent,
            JupiterPerpetualsInstruction::InstantDecreasePositionEvent => instant_decrease_position_event::InstantDecreasePositionEvent,
            JupiterPerpetualsInstruction::DepositCollateralEvent => deposit_collateral_event::DepositCollateralEvent,
            JupiterPerpetualsInstruction::WithdrawCollateralEvent => withdraw_collateral_event::WithdrawCollateralEvent,
            JupiterPerpetualsInstruction::BorrowFromCustodyEvent => borrow_from_custody_event::BorrowFromCustodyEvent,
            JupiterPerpetualsInstruction::RepayToCustodyEvent => repay_to_custody_event::RepayToCustodyEvent,
            JupiterPerpetualsInstruction::LiquidateBorrowPositionEvent => liquidate_borrow_position_event::LiquidateBorrowPositionEvent,
            JupiterPerpetualsInstruction::RedeemStakeEvent => redeem_stake_event::RedeemStakeEvent,
            JupiterPerpetualsInstruction::WithdrawStakeEvent => withdraw_stake_event::WithdrawStakeEvent,
            JupiterPerpetualsInstruction::DelegateStakeEvent => delegate_stake_event::DelegateStakeEvent,
        )
    }
}
