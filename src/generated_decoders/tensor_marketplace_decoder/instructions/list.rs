

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x36aec14311298426")]
pub struct List{
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub amount: u64,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<solana_sdk::pubkey::Pubkey>,
    pub private_taker: Option<solana_sdk::pubkey::Pubkey>,
    pub maker_broker: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct ListInstructionAccounts {
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub bubblegum_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub list_state: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for List {
    type ArrangedAccounts = ListInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let tree_authority = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let delegate = accounts.get(2)?;
        let merkle_tree = accounts.get(3)?;
        let log_wrapper = accounts.get(4)?;
        let compression_program = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let bubblegum_program = accounts.get(7)?;
        let tcomp_program = accounts.get(8)?;
        let list_state = accounts.get(9)?;
        let rent_payer = accounts.get(10)?;

        Some(ListInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            owner: owner.pubkey,
            delegate: delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            list_state: list_state.pubkey,
            rent_payer: rent_payer.pubkey,
        })
    }
}