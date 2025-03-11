

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x96460d8709cc4b04")]
pub struct CloseExpiredListing{
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
}

pub struct CloseExpiredListingInstructionAccounts {
    pub list_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub bubblegum_program: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseExpiredListing {
    type ArrangedAccounts = CloseExpiredListingInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let list_state = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let tcomp_program = accounts.get(3)?;
        let tree_authority = accounts.get(4)?;
        let merkle_tree = accounts.get(5)?;
        let log_wrapper = accounts.get(6)?;
        let compression_program = accounts.get(7)?;
        let bubblegum_program = accounts.get(8)?;
        let rent_dest = accounts.get(9)?;

        Some(CloseExpiredListingInstructionAccounts {
            list_state: list_state.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            tree_authority: tree_authority.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}