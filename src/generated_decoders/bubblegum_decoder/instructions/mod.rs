



use super::BubblegumDecoder;
pub mod burn;
pub mod cancel_redeem;
pub mod create_tree;
pub mod decompress_v1;
pub mod delegate;
pub mod mint_to_collection_v1;
pub mod mint_v1;
pub mod redeem;
pub mod set_and_verify_collection;
pub mod set_decompressible_state;
pub mod set_tree_delegate;
pub mod transfer;
pub mod unverify_collection;
pub mod unverify_creator;
pub mod verify_collection;
pub mod verify_creator;
pub mod update_metadata;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum BubblegumInstruction {
    Burn(burn::Burn),
    CancelRedeem(cancel_redeem::CancelRedeem),
    CreateTree(create_tree::CreateTree),
    DecompressV1(decompress_v1::DecompressV1),
    Delegate(delegate::Delegate),
    MintToCollectionV1(mint_to_collection_v1::MintToCollectionV1),
    MintV1(mint_v1::MintV1),
    Redeem(redeem::Redeem),
    SetAndVerifyCollection(set_and_verify_collection::SetAndVerifyCollection),
    SetDecompressibleState(set_decompressible_state::SetDecompressibleState),
    SetTreeDelegate(set_tree_delegate::SetTreeDelegate),
    Transfer(transfer::Transfer),
    UnverifyCollection(unverify_collection::UnverifyCollection),
    UnverifyCreator(unverify_creator::UnverifyCreator),
    VerifyCollection(verify_collection::VerifyCollection),
    VerifyCreator(verify_creator::VerifyCreator),
    UpdateMetadata(update_metadata::UpdateMetadata),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for BubblegumDecoder {
    type InstructionType = BubblegumInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            BubblegumInstruction::Burn => burn::Burn,
            BubblegumInstruction::CancelRedeem => cancel_redeem::CancelRedeem,
            BubblegumInstruction::CreateTree => create_tree::CreateTree,
            BubblegumInstruction::DecompressV1 => decompress_v1::DecompressV1,
            BubblegumInstruction::Delegate => delegate::Delegate,
            BubblegumInstruction::MintToCollectionV1 => mint_to_collection_v1::MintToCollectionV1,
            BubblegumInstruction::MintV1 => mint_v1::MintV1,
            BubblegumInstruction::Redeem => redeem::Redeem,
            BubblegumInstruction::SetAndVerifyCollection => set_and_verify_collection::SetAndVerifyCollection,
            BubblegumInstruction::SetDecompressibleState => set_decompressible_state::SetDecompressibleState,
            BubblegumInstruction::SetTreeDelegate => set_tree_delegate::SetTreeDelegate,
            BubblegumInstruction::Transfer => transfer::Transfer,
            BubblegumInstruction::UnverifyCollection => unverify_collection::UnverifyCollection,
            BubblegumInstruction::UnverifyCreator => unverify_creator::UnverifyCreator,
            BubblegumInstruction::VerifyCollection => verify_collection::VerifyCollection,
            BubblegumInstruction::VerifyCreator => verify_creator::VerifyCreator,
            BubblegumInstruction::UpdateMetadata => update_metadata::UpdateMetadata,
        )
    }
}