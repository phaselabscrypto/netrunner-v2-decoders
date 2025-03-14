

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x66063d1201daebea")]
pub struct Buy{
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub meta_hash: [u8; 32],
    pub creator_shares: Vec<u8>,
    pub creator_verified: Vec<bool>,
    pub seller_fee_basis_points: u16,
    pub max_amount: u64,
    pub optional_royalty_pct: Option<u16>,
}

pub struct BuyInstructionAccounts {
    pub tcomp: solana_sdk::pubkey::Pubkey,
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub bubblegum_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub list_state: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub maker_broker: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Buy {
    type ArrangedAccounts = BuyInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tcomp = accounts.get(0)?;
        let tree_authority = accounts.get(1)?;
        let merkle_tree = accounts.get(2)?;
        let log_wrapper = accounts.get(3)?;
        let compression_program = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let bubblegum_program = accounts.get(6)?;
        let tcomp_program = accounts.get(7)?;
        let list_state = accounts.get(8)?;
        let buyer = accounts.get(9)?;
        let payer = accounts.get(10)?;
        let owner = accounts.get(11)?;
        let taker_broker = accounts.get(12)?;
        let maker_broker = accounts.get(13)?;
        let rent_dest = accounts.get(14)?;

        Some(BuyInstructionAccounts {
            tcomp: tcomp.pubkey,
            tree_authority: tree_authority.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            list_state: list_state.pubkey,
            buyer: buyer.pubkey,
            payer: payer.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}