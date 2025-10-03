use super::OkxDexV2Decoder;
pub mod add_resolver_event;
pub mod cancel_order_event;
pub mod claim;
pub mod commission_sol_proxy_swap;
pub mod commission_sol_swap;
pub mod commission_spl_proxy_swap;
pub mod commission_spl_swap;
pub mod commission_wrap_unwrap;
pub mod create_token_account;
pub mod create_token_account_with_seed;
pub mod fill_order_event;
pub mod init_global_config_event;
pub mod pause_trading_event;
pub mod place_order_event;
pub mod platform_fee_sol_proxy_swap_v2;
pub mod platform_fee_sol_wrap_unwrap_v2;
pub mod platform_fee_spl_proxy_swap_v2;
pub mod proxy_swap;
pub mod refund_event;
pub mod remove_resolver_event;
pub mod set_admin_event;
pub mod set_fee_multiplier_event;
pub mod set_trade_fee_event;
pub mod swap;
pub mod swap_event;
pub mod swap_tob_v3;
pub mod swap_tob_v3_with_receiver;
pub mod swap_v3;
pub mod update_order_event;
pub mod wrap_unwrap_v3;

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
pub enum OkxDexV2Instruction {
    Claim(claim::Claim),
    CommissionSolProxySwap(commission_sol_proxy_swap::CommissionSolProxySwap),
    CommissionSolSwap(commission_sol_swap::CommissionSolSwap),
    CommissionSplProxySwap(commission_spl_proxy_swap::CommissionSplProxySwap),
    CommissionSplSwap(commission_spl_swap::CommissionSplSwap),
    CommissionWrapUnwrap(commission_wrap_unwrap::CommissionWrapUnwrap),
    CreateTokenAccount(create_token_account::CreateTokenAccount),
    CreateTokenAccountWithSeed(create_token_account_with_seed::CreateTokenAccountWithSeed),
    PlatformFeeSolProxySwapV2(platform_fee_sol_proxy_swap_v2::PlatformFeeSolProxySwapV2),
    PlatformFeeSolWrapUnwrapV2(platform_fee_sol_wrap_unwrap_v2::PlatformFeeSolWrapUnwrapV2),
    PlatformFeeSplProxySwapV2(platform_fee_spl_proxy_swap_v2::PlatformFeeSplProxySwapV2),
    ProxySwap(proxy_swap::ProxySwap),
    Swap(swap::Swap),
    SwapTobV3(swap_tob_v3::SwapTobV3),
    SwapTobV3WithReceiver(swap_tob_v3_with_receiver::SwapTobV3WithReceiver),
    SwapV3(swap_v3::SwapV3),
    WrapUnwrapV3(wrap_unwrap_v3::WrapUnwrapV3),
    AddResolverEvent(add_resolver_event::AddResolverEvent),
    CancelOrderEvent(cancel_order_event::CancelOrderEvent),
    FillOrderEvent(fill_order_event::FillOrderEvent),
    InitGlobalConfigEvent(init_global_config_event::InitGlobalConfigEvent),
    PauseTradingEvent(pause_trading_event::PauseTradingEvent),
    PlaceOrderEvent(place_order_event::PlaceOrderEvent),
    RefundEvent(refund_event::RefundEvent),
    RemoveResolverEvent(remove_resolver_event::RemoveResolverEvent),
    SetAdminEvent(set_admin_event::SetAdminEvent),
    SetFeeMultiplierEvent(set_fee_multiplier_event::SetFeeMultiplierEvent),
    SetTradeFeeEvent(set_trade_fee_event::SetTradeFeeEvent),
    SwapEvent(swap_event::SwapEvent),
    UpdateOrderEvent(update_order_event::UpdateOrderEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for OkxDexV2Decoder {
    type InstructionType = OkxDexV2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            OkxDexV2Instruction::Claim => claim::Claim,
            OkxDexV2Instruction::CommissionSolProxySwap => commission_sol_proxy_swap::CommissionSolProxySwap,
            OkxDexV2Instruction::CommissionSolSwap => commission_sol_swap::CommissionSolSwap,
            OkxDexV2Instruction::CommissionSplProxySwap => commission_spl_proxy_swap::CommissionSplProxySwap,
            OkxDexV2Instruction::CommissionSplSwap => commission_spl_swap::CommissionSplSwap,
            OkxDexV2Instruction::CommissionWrapUnwrap => commission_wrap_unwrap::CommissionWrapUnwrap,
            OkxDexV2Instruction::CreateTokenAccount => create_token_account::CreateTokenAccount,
            OkxDexV2Instruction::CreateTokenAccountWithSeed => create_token_account_with_seed::CreateTokenAccountWithSeed,
            OkxDexV2Instruction::PlatformFeeSolProxySwapV2 => platform_fee_sol_proxy_swap_v2::PlatformFeeSolProxySwapV2,
            OkxDexV2Instruction::PlatformFeeSolWrapUnwrapV2 => platform_fee_sol_wrap_unwrap_v2::PlatformFeeSolWrapUnwrapV2,
            OkxDexV2Instruction::PlatformFeeSplProxySwapV2 => platform_fee_spl_proxy_swap_v2::PlatformFeeSplProxySwapV2,
            OkxDexV2Instruction::ProxySwap => proxy_swap::ProxySwap,
            OkxDexV2Instruction::Swap => swap::Swap,
            OkxDexV2Instruction::SwapTobV3 => swap_tob_v3::SwapTobV3,
            OkxDexV2Instruction::SwapTobV3WithReceiver => swap_tob_v3_with_receiver::SwapTobV3WithReceiver,
            OkxDexV2Instruction::SwapV3 => swap_v3::SwapV3,
            OkxDexV2Instruction::WrapUnwrapV3 => wrap_unwrap_v3::WrapUnwrapV3,
            OkxDexV2Instruction::AddResolverEvent => add_resolver_event::AddResolverEvent,
            OkxDexV2Instruction::CancelOrderEvent => cancel_order_event::CancelOrderEvent,
            OkxDexV2Instruction::FillOrderEvent => fill_order_event::FillOrderEvent,
            OkxDexV2Instruction::InitGlobalConfigEvent => init_global_config_event::InitGlobalConfigEvent,
            OkxDexV2Instruction::PauseTradingEvent => pause_trading_event::PauseTradingEvent,
            OkxDexV2Instruction::PlaceOrderEvent => place_order_event::PlaceOrderEvent,
            OkxDexV2Instruction::RefundEvent => refund_event::RefundEvent,
            OkxDexV2Instruction::RemoveResolverEvent => remove_resolver_event::RemoveResolverEvent,
            OkxDexV2Instruction::SetAdminEvent => set_admin_event::SetAdminEvent,
            OkxDexV2Instruction::SetFeeMultiplierEvent => set_fee_multiplier_event::SetFeeMultiplierEvent,
            OkxDexV2Instruction::SetTradeFeeEvent => set_trade_fee_event::SetTradeFeeEvent,
            OkxDexV2Instruction::SwapEvent => swap_event::SwapEvent,
            OkxDexV2Instruction::UpdateOrderEvent => update_order_event::UpdateOrderEvent,
        )
    }
}
