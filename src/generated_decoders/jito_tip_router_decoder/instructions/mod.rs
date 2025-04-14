



use super::JitoTipRouterDecoder;
pub mod claim_with_payer;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum JitoTipRouterInstruction {
    ClaimWithPayer(claim_with_payer::ClaimWithPayer),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JitoTipRouterDecoder {
    type InstructionType = JitoTipRouterInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JitoTipRouterInstruction::ClaimWithPayer => claim_with_payer::ClaimWithPayer,
        )
    }
}