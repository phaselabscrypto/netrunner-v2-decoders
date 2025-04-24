use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfafb2a6a2989baa8")]
pub struct UnverifyCollection {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub message: MetadataArgs,
}

pub struct UnverifyCollectionInstructionAccounts {
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub leaf_owner: solana_sdk::pubkey::Pubkey,
    pub leaf_delegate: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub tree_delegate: solana_sdk::pubkey::Pubkey,
    pub collection_authority: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record_pda: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub edition_account: solana_sdk::pubkey::Pubkey,
    pub bubblegum_signer: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnverifyCollection {
    type ArrangedAccounts = UnverifyCollectionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tree_authority = accounts.get(0)?;
        let leaf_owner = accounts.get(1)?;
        let leaf_delegate = accounts.get(2)?;
        let merkle_tree = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let tree_delegate = accounts.get(5)?;
        let collection_authority = accounts.get(6)?;
        let collection_authority_record_pda = accounts.get(7)?;
        let collection_mint = accounts.get(8)?;
        let collection_metadata = accounts.get(9)?;
        let edition_account = accounts.get(10)?;
        let bubblegum_signer = accounts.get(11)?;
        let log_wrapper = accounts.get(12)?;
        let compression_program = accounts.get(13)?;
        let token_metadata_program = accounts.get(14)?;
        let system_program = accounts.get(15)?;

        Some(UnverifyCollectionInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            payer: payer.pubkey,
            tree_delegate: tree_delegate.pubkey,
            collection_authority: collection_authority.pubkey,
            collection_authority_record_pda: collection_authority_record_pda.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            edition_account: edition_account.pubkey,
            bubblegum_signer: bubblegum_signer.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
