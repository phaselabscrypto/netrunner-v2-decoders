



use super::CandyMachineCoreDecoder;
pub mod add_config_lines;
pub mod initialize;
pub mod initialize_v2;
pub mod mint;
pub mod mint_v2;
pub mod set_authority;
pub mod set_collection;
pub mod set_collection_v2;
pub mod set_mint_authority;
pub mod set_token_standard;
pub mod update;
pub mod withdraw;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum CandyMachineCoreInstruction {
    AddConfigLines(add_config_lines::AddConfigLines),
    Initialize(initialize::Initialize),
    InitializeV2(initialize_v2::InitializeV2),
    Mint(mint::Mint),
    MintV2(mint_v2::MintV2),
    SetAuthority(set_authority::SetAuthority),
    SetCollection(set_collection::SetCollection),
    SetCollectionV2(set_collection_v2::SetCollectionV2),
    SetMintAuthority(set_mint_authority::SetMintAuthority),
    SetTokenStandard(set_token_standard::SetTokenStandard),
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
            CandyMachineCoreInstruction::InitializeV2 => initialize_v2::InitializeV2,
            CandyMachineCoreInstruction::Mint => mint::Mint,
            CandyMachineCoreInstruction::MintV2 => mint_v2::MintV2,
            CandyMachineCoreInstruction::SetAuthority => set_authority::SetAuthority,
            CandyMachineCoreInstruction::SetCollection => set_collection::SetCollection,
            CandyMachineCoreInstruction::SetCollectionV2 => set_collection_v2::SetCollectionV2,
            CandyMachineCoreInstruction::SetMintAuthority => set_mint_authority::SetMintAuthority,
            CandyMachineCoreInstruction::SetTokenStandard => set_token_standard::SetTokenStandard,
            CandyMachineCoreInstruction::Update => update::Update,
            CandyMachineCoreInstruction::Withdraw => withdraw::Withdraw,
        )
    }
}