use super::JupiterDecoder;
pub mod aldrin_swap;
pub mod aldrin_v2_swap;
pub mod balansol_swap;
pub mod claim_bonk;
pub mod create_open_order_v2;
pub mod create_open_orders;
pub mod crema_swap;
pub mod cropper_swap;
pub mod cykura_swap;
pub mod deltafi_swap;
pub mod dradex_swap;
pub mod fee_event;
pub mod goosefx_swap;
pub mod invariant_swap;
pub mod lifinity_swap;
pub mod lifinity_v2_swap;
pub mod marco_polo_swap;
pub mod marinade_deposit;
pub mod marinade_unstake;
pub mod mercurial_swap;
pub mod meteora_swap;
pub mod phoenix_swap;
pub mod raydium_clmm_swap;
pub mod raydium_clmm_swap_exact_output;
pub mod raydium_swap;
pub mod raydium_swap_exact_output;
pub mod route;
pub mod saber_add_decimals;
pub mod saber_swap;
pub mod sencha_swap;
pub mod serum_swap;
pub mod step_swap;
pub mod swap_event;
pub mod token_swap;
pub mod whirlpool_swap;
pub mod whirlpool_swap_exact_output;

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
    Route(route::Route),
    WhirlpoolSwapExactOutput(whirlpool_swap_exact_output::WhirlpoolSwapExactOutput),
    RaydiumSwapExactOutput(raydium_swap_exact_output::RaydiumSwapExactOutput),
    RaydiumClmmSwapExactOutput(raydium_clmm_swap_exact_output::RaydiumClmmSwapExactOutput),
    CreateOpenOrders(create_open_orders::CreateOpenOrders),
    CreateOpenOrderV2(create_open_order_v2::CreateOpenOrderV2),
    MercurialSwap(mercurial_swap::MercurialSwap),
    CykuraSwap(cykura_swap::CykuraSwap),
    SerumSwap(serum_swap::SerumSwap),
    SaberSwap(saber_swap::SaberSwap),
    SaberAddDecimals(saber_add_decimals::SaberAddDecimals),
    TokenSwap(token_swap::TokenSwap),
    SenchaSwap(sencha_swap::SenchaSwap),
    StepSwap(step_swap::StepSwap),
    CropperSwap(cropper_swap::CropperSwap),
    RaydiumSwap(raydium_swap::RaydiumSwap),
    CremaSwap(crema_swap::CremaSwap),
    LifinitySwap(lifinity_swap::LifinitySwap),
    MarinadeDeposit(marinade_deposit::MarinadeDeposit),
    MarinadeUnstake(marinade_unstake::MarinadeUnstake),
    AldrinSwap(aldrin_swap::AldrinSwap),
    AldrinV2Swap(aldrin_v2_swap::AldrinV2Swap),
    WhirlpoolSwap(whirlpool_swap::WhirlpoolSwap),
    InvariantSwap(invariant_swap::InvariantSwap),
    MeteoraSwap(meteora_swap::MeteoraSwap),
    GoosefxSwap(goosefx_swap::GoosefxSwap),
    DeltafiSwap(deltafi_swap::DeltafiSwap),
    BalansolSwap(balansol_swap::BalansolSwap),
    MarcoPoloSwap(marco_polo_swap::MarcoPoloSwap),
    DradexSwap(dradex_swap::DradexSwap),
    LifinityV2Swap(lifinity_v2_swap::LifinityV2Swap),
    RaydiumClmmSwap(raydium_clmm_swap::RaydiumClmmSwap),
    PhoenixSwap(phoenix_swap::PhoenixSwap),
    ClaimBonk(claim_bonk::ClaimBonk),
    SwapEvent(swap_event::SwapEvent),
    FeeEvent(fee_event::FeeEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for JupiterDecoder {
    type InstructionType = JupiterInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterInstruction::Route => route::Route,
            JupiterInstruction::WhirlpoolSwapExactOutput => whirlpool_swap_exact_output::WhirlpoolSwapExactOutput,
            JupiterInstruction::RaydiumSwapExactOutput => raydium_swap_exact_output::RaydiumSwapExactOutput,
            JupiterInstruction::RaydiumClmmSwapExactOutput => raydium_clmm_swap_exact_output::RaydiumClmmSwapExactOutput,
            JupiterInstruction::CreateOpenOrders => create_open_orders::CreateOpenOrders,
            JupiterInstruction::CreateOpenOrderV2 => create_open_order_v2::CreateOpenOrderV2,
            JupiterInstruction::MercurialSwap => mercurial_swap::MercurialSwap,
            JupiterInstruction::CykuraSwap => cykura_swap::CykuraSwap,
            JupiterInstruction::SerumSwap => serum_swap::SerumSwap,
            JupiterInstruction::SaberSwap => saber_swap::SaberSwap,
            JupiterInstruction::SaberAddDecimals => saber_add_decimals::SaberAddDecimals,
            JupiterInstruction::TokenSwap => token_swap::TokenSwap,
            JupiterInstruction::SenchaSwap => sencha_swap::SenchaSwap,
            JupiterInstruction::StepSwap => step_swap::StepSwap,
            JupiterInstruction::CropperSwap => cropper_swap::CropperSwap,
            JupiterInstruction::RaydiumSwap => raydium_swap::RaydiumSwap,
            JupiterInstruction::CremaSwap => crema_swap::CremaSwap,
            JupiterInstruction::LifinitySwap => lifinity_swap::LifinitySwap,
            JupiterInstruction::MarinadeDeposit => marinade_deposit::MarinadeDeposit,
            JupiterInstruction::MarinadeUnstake => marinade_unstake::MarinadeUnstake,
            JupiterInstruction::AldrinSwap => aldrin_swap::AldrinSwap,
            JupiterInstruction::AldrinV2Swap => aldrin_v2_swap::AldrinV2Swap,
            JupiterInstruction::WhirlpoolSwap => whirlpool_swap::WhirlpoolSwap,
            JupiterInstruction::InvariantSwap => invariant_swap::InvariantSwap,
            JupiterInstruction::MeteoraSwap => meteora_swap::MeteoraSwap,
            JupiterInstruction::GoosefxSwap => goosefx_swap::GoosefxSwap,
            JupiterInstruction::DeltafiSwap => deltafi_swap::DeltafiSwap,
            JupiterInstruction::BalansolSwap => balansol_swap::BalansolSwap,
            JupiterInstruction::MarcoPoloSwap => marco_polo_swap::MarcoPoloSwap,
            JupiterInstruction::DradexSwap => dradex_swap::DradexSwap,
            JupiterInstruction::LifinityV2Swap => lifinity_v2_swap::LifinityV2Swap,
            JupiterInstruction::RaydiumClmmSwap => raydium_clmm_swap::RaydiumClmmSwap,
            JupiterInstruction::PhoenixSwap => phoenix_swap::PhoenixSwap,
            JupiterInstruction::ClaimBonk => claim_bonk::ClaimBonk,
            JupiterInstruction::SwapEvent => swap_event::SwapEvent,
            JupiterInstruction::FeeEvent => fee_event::FeeEvent,
        )
    }
}
