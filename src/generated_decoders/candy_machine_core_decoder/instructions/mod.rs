



use super::CandyMachineCoreDecoder;
pub mod add_config_lines;
pub mod initialize;
pub mod mint_asset;
pub mod set_authority;
pub mod set_collection;
pub mod set_mint_authority;
pub mod update;
pub mod withdraw;

#[derive(carbon_core::InstructionType, PartialEq, Eq, Debug, Clone)]
pub enum CandyMachineCoreInstruction {
    AddConfigLines(add_config_lines::AddConfigLines),
    Initialize(initialize::Initialize),
    MintAsset(mint_asset::MintAsset),
    SetAuthority(set_authority::SetAuthority),
    SetCollection(set_collection::SetCollection),
    SetMintAuthority(set_mint_authority::SetMintAuthority),
    Update(update::Update),
    Withdraw(withdraw::Withdraw),
}

impl carbon_core::instruction::InstructionDecoder<'_> for CandyMachineCoreDecoder {
    type InstructionType = CandyMachineCoreInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            CandyMachineCoreInstruction::AddConfigLines => add_config_lines::AddConfigLines,
            CandyMachineCoreInstruction::Initialize => initialize::Initialize,
            CandyMachineCoreInstruction::MintAsset => mint_asset::MintAsset,
            CandyMachineCoreInstruction::SetAuthority => set_authority::SetAuthority,
            CandyMachineCoreInstruction::SetCollection => set_collection::SetCollection,
            CandyMachineCoreInstruction::SetMintAuthority => set_mint_authority::SetMintAuthority,
            CandyMachineCoreInstruction::Update => update::Update,
            CandyMachineCoreInstruction::Withdraw => withdraw::Withdraw,
        )
    }
}