use super::CandyMachineDecoder;
pub mod add_config_lines;
pub mod initialize_candy_machine;
pub mod mint_nft;
pub mod remove_collection;
pub mod remove_freeze;
pub mod set_collection;
pub mod set_collection_during_mint;
pub mod set_freeze;
pub mod thaw_nft;
pub mod unlock_funds;
pub mod update_authority;
pub mod update_candy_machine;
pub mod withdraw_funds;

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
pub enum CandyMachineInstruction {
    InitializeCandyMachine(initialize_candy_machine::InitializeCandyMachine),
    UpdateCandyMachine(update_candy_machine::UpdateCandyMachine),
    UpdateAuthority(update_authority::UpdateAuthority),
    AddConfigLines(add_config_lines::AddConfigLines),
    SetCollection(set_collection::SetCollection),
    RemoveCollection(remove_collection::RemoveCollection),
    MintNft(mint_nft::MintNft),
    SetCollectionDuringMint(set_collection_during_mint::SetCollectionDuringMint),
    WithdrawFunds(withdraw_funds::WithdrawFunds),
    SetFreeze(set_freeze::SetFreeze),
    RemoveFreeze(remove_freeze::RemoveFreeze),
    ThawNft(thaw_nft::ThawNft),
    UnlockFunds(unlock_funds::UnlockFunds),
}

impl carbon_core::instruction::InstructionDecoder<'_> for CandyMachineDecoder {
    type InstructionType = CandyMachineInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            CandyMachineInstruction::InitializeCandyMachine => initialize_candy_machine::InitializeCandyMachine,
            CandyMachineInstruction::UpdateCandyMachine => update_candy_machine::UpdateCandyMachine,
            CandyMachineInstruction::UpdateAuthority => update_authority::UpdateAuthority,
            CandyMachineInstruction::AddConfigLines => add_config_lines::AddConfigLines,
            CandyMachineInstruction::SetCollection => set_collection::SetCollection,
            CandyMachineInstruction::RemoveCollection => remove_collection::RemoveCollection,
            CandyMachineInstruction::MintNft => mint_nft::MintNft,
            CandyMachineInstruction::SetCollectionDuringMint => set_collection_during_mint::SetCollectionDuringMint,
            CandyMachineInstruction::WithdrawFunds => withdraw_funds::WithdrawFunds,
            CandyMachineInstruction::SetFreeze => set_freeze::SetFreeze,
            CandyMachineInstruction::RemoveFreeze => remove_freeze::RemoveFreeze,
            CandyMachineInstruction::ThawNft => thaw_nft::ThawNft,
            CandyMachineInstruction::UnlockFunds => unlock_funds::UnlockFunds,
        )
    }
}
