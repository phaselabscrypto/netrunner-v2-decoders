use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb80c569546c461e1")]
pub struct Redeem {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

pub struct RedeemInstructionAccounts {
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub leaf_owner: solana_sdk::pubkey::Pubkey,
    pub leaf_delegate: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub voucher: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Redeem {
    type ArrangedAccounts = RedeemInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tree_authority = accounts.get(0)?;
        let leaf_owner = accounts.get(1)?;
        let leaf_delegate = accounts.get(2)?;
        let merkle_tree = accounts.get(3)?;
        let voucher = accounts.get(4)?;
        let log_wrapper = accounts.get(5)?;
        let compression_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;

        Some(RedeemInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            voucher: voucher.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
