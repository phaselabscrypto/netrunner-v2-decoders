use super::CandyGuardDecoder;
pub mod initialize;
pub mod mint;
pub mod mint_v2;
pub mod route;
pub mod set_authority;
pub mod unwrap;
pub mod update;
pub mod withdraw;
pub mod wrap;

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
pub enum CandyGuardInstruction {
    Initialize(initialize::Initialize),
    Mint(mint::Mint),
    MintV2(mint_v2::MintV2),
    Route(route::Route),
    SetAuthority(set_authority::SetAuthority),
    Unwrap(unwrap::Unwrap),
    Update(update::Update),
    Withdraw(withdraw::Withdraw),
    Wrap(wrap::Wrap),
}

impl carbon_core::instruction::InstructionDecoder<'_> for CandyGuardDecoder {
    type InstructionType = CandyGuardInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            CandyGuardInstruction::Initialize => initialize::Initialize,
            CandyGuardInstruction::Mint => mint::Mint,
            CandyGuardInstruction::MintV2 => mint_v2::MintV2,
            CandyGuardInstruction::Route => route::Route,
            CandyGuardInstruction::SetAuthority => set_authority::SetAuthority,
            CandyGuardInstruction::Unwrap => unwrap::Unwrap,
            CandyGuardInstruction::Update => update::Update,
            CandyGuardInstruction::Withdraw => withdraw::Withdraw,
            CandyGuardInstruction::Wrap => wrap::Wrap,
        )
    }
}
