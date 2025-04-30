use super::JupiterDecoder;
pub mod aldrin_swap;
pub mod aldrin_v2_swap;
pub mod create_open_orders;
pub mod crema_token_swap;
pub mod cropper_token_swap;
pub mod cykura_swap;
pub mod initialize_token_ledger;
pub mod lifinity_token_swap;
pub mod mercurial_exchange;
pub mod raydium_swap;
pub mod raydium_swap_v2;
pub mod risk_check_and_fee;
pub mod saber_add_decimals_deposit;
pub mod saber_add_decimals_withdraw;
pub mod saber_exchange;
pub mod saber_swap;
pub mod sencha_exchange;
pub mod serum_swap;
pub mod set_token_ledger;
pub mod step_token_swap;
pub mod token_swap;
pub mod whirlpool_swap;

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
pub enum JupiterInstruction {
    MercurialExchange(mercurial_exchange::MercurialExchange),
    SaberExchange(saber_exchange::SaberExchange),
    SaberSwap(saber_swap::SaberSwap),
    SaberAddDecimalsDeposit(saber_add_decimals_deposit::SaberAddDecimalsDeposit),
    SaberAddDecimalsWithdraw(saber_add_decimals_withdraw::SaberAddDecimalsWithdraw),
    SenchaExchange(sencha_exchange::SenchaExchange),
    SerumSwap(serum_swap::SerumSwap),
    TokenSwap(token_swap::TokenSwap),
    StepTokenSwap(step_token_swap::StepTokenSwap),
    CropperTokenSwap(cropper_token_swap::CropperTokenSwap),
    RaydiumSwap(raydium_swap::RaydiumSwap),
    RaydiumSwapV2(raydium_swap_v2::RaydiumSwapV2),
    AldrinSwap(aldrin_swap::AldrinSwap),
    AldrinV2Swap(aldrin_v2_swap::AldrinV2Swap),
    CremaTokenSwap(crema_token_swap::CremaTokenSwap),
    LifinityTokenSwap(lifinity_token_swap::LifinityTokenSwap),
    CykuraSwap(cykura_swap::CykuraSwap),
    WhirlpoolSwap(whirlpool_swap::WhirlpoolSwap),
    RiskCheckAndFee(risk_check_and_fee::RiskCheckAndFee),
    InitializeTokenLedger(initialize_token_ledger::InitializeTokenLedger),
    SetTokenLedger(set_token_ledger::SetTokenLedger),
    CreateOpenOrders(create_open_orders::CreateOpenOrders),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JupiterDecoder {
    type InstructionType = JupiterInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterInstruction::MercurialExchange => mercurial_exchange::MercurialExchange,
            JupiterInstruction::SaberExchange => saber_exchange::SaberExchange,
            JupiterInstruction::SaberSwap => saber_swap::SaberSwap,
            JupiterInstruction::SaberAddDecimalsDeposit => saber_add_decimals_deposit::SaberAddDecimalsDeposit,
            JupiterInstruction::SaberAddDecimalsWithdraw => saber_add_decimals_withdraw::SaberAddDecimalsWithdraw,
            JupiterInstruction::SenchaExchange => sencha_exchange::SenchaExchange,
            JupiterInstruction::SerumSwap => serum_swap::SerumSwap,
            JupiterInstruction::TokenSwap => token_swap::TokenSwap,
            JupiterInstruction::StepTokenSwap => step_token_swap::StepTokenSwap,
            JupiterInstruction::CropperTokenSwap => cropper_token_swap::CropperTokenSwap,
            JupiterInstruction::RaydiumSwap => raydium_swap::RaydiumSwap,
            JupiterInstruction::RaydiumSwapV2 => raydium_swap_v2::RaydiumSwapV2,
            JupiterInstruction::AldrinSwap => aldrin_swap::AldrinSwap,
            JupiterInstruction::AldrinV2Swap => aldrin_v2_swap::AldrinV2Swap,
            JupiterInstruction::CremaTokenSwap => crema_token_swap::CremaTokenSwap,
            JupiterInstruction::LifinityTokenSwap => lifinity_token_swap::LifinityTokenSwap,
            JupiterInstruction::CykuraSwap => cykura_swap::CykuraSwap,
            JupiterInstruction::WhirlpoolSwap => whirlpool_swap::WhirlpoolSwap,
            JupiterInstruction::RiskCheckAndFee => risk_check_and_fee::RiskCheckAndFee,
            JupiterInstruction::InitializeTokenLedger => initialize_token_ledger::InitializeTokenLedger,
            JupiterInstruction::SetTokenLedger => set_token_ledger::SetTokenLedger,
            JupiterInstruction::CreateOpenOrders => create_open_orders::CreateOpenOrders,
        )
    }
}
