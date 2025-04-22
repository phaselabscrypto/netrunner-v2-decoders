

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa553888e59ca2fdc")]
pub struct CreateTree{
    pub max_depth: u32,
    pub max_buffer_size: u32,
    pub public: Option<bool>,
}

pub struct CreateTreeInstructionAccounts {
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub tree_creator: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateTree {
    type ArrangedAccounts = CreateTreeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tree_authority = accounts.get(0)?;
        let merkle_tree = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let tree_creator = accounts.get(3)?;
        let log_wrapper = accounts.get(4)?;
        let compression_program = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(CreateTreeInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            merkle_tree: merkle_tree.pubkey,
            payer: payer.pubkey,
            tree_creator: tree_creator.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}