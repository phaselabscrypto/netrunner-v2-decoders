use super::TitanSwapDecoder;

pub mod initialize;
pub mod swap_route;
pub mod swap_route_v2;

pub use self::initialize::*;
pub use self::swap_route::*;
pub use self::swap_route_v2::*;

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
pub enum TitanSwapInstruction {
    Initialize(Initialize),
    SwapRoute(SwapRoute),
    SwapRouteV2(SwapRouteV2),
}

impl carbon_core::instruction::InstructionDecoder<'_> for TitanSwapDecoder {
    type InstructionType = TitanSwapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            TitanSwapInstruction::Initialize => initialize::Initialize,
            TitanSwapInstruction::SwapRoute => swap_route::SwapRoute,
            TitanSwapInstruction::SwapRouteV2 => swap_route_v2::SwapRouteV2,
        )
    }
}
