use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa334c8e78c0345ba")]
pub struct Transfer {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

pub struct TransferInstructionAccounts {
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub leaf_owner: solana_sdk::pubkey::Pubkey,
    pub leaf_delegate: solana_sdk::pubkey::Pubkey,
    pub new_leaf_owner: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tree_authority = accounts.get(0)?;
        let leaf_owner = accounts.get(1)?;
        let leaf_delegate = accounts.get(2)?;
        let new_leaf_owner = accounts.get(3)?;
        let merkle_tree = accounts.get(4)?;
        let log_wrapper = accounts.get(5)?;
        let compression_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;

        Some(TransferInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            new_leaf_owner: new_leaf_owner.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
