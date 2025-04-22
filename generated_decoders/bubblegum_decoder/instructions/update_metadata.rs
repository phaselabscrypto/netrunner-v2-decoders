
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xaab62bef614ee1ba")]
pub struct UpdateMetadata{
    pub root: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub current_metadata: MetadataArgs,
    pub update_args: UpdateArgs,
}

pub struct UpdateMetadataInstructionAccounts {
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record_pda: solana_sdk::pubkey::Pubkey,
    pub leaf_owner: solana_sdk::pubkey::Pubkey,
    pub leaf_delegate: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateMetadata {
    type ArrangedAccounts = UpdateMetadataInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tree_authority = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let collection_mint = accounts.get(2)?;
        let collection_metadata = accounts.get(3)?;
        let collection_authority_record_pda = accounts.get(4)?;
        let leaf_owner = accounts.get(5)?;
        let leaf_delegate = accounts.get(6)?;
        let payer = accounts.get(7)?;
        let merkle_tree = accounts.get(8)?;
        let log_wrapper = accounts.get(9)?;
        let compression_program = accounts.get(10)?;
        let token_metadata_program = accounts.get(11)?;
        let system_program = accounts.get(12)?;

        Some(UpdateMetadataInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            authority: authority.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_authority_record_pda: collection_authority_record_pda.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            payer: payer.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}