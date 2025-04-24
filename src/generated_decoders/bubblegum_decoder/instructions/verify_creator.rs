use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x34116084470455c2")]
pub struct VerifyCreator {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub message: MetadataArgs,
}

pub struct VerifyCreatorInstructionAccounts {
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub leaf_owner: solana_sdk::pubkey::Pubkey,
    pub leaf_delegate: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub creator: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for VerifyCreator {
    type ArrangedAccounts = VerifyCreatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tree_authority = accounts.get(0)?;
        let leaf_owner = accounts.get(1)?;
        let leaf_delegate = accounts.get(2)?;
        let merkle_tree = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let creator = accounts.get(5)?;
        let log_wrapper = accounts.get(6)?;
        let compression_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;

        Some(VerifyCreatorInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            payer: payer.pubkey,
            creator: creator.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
