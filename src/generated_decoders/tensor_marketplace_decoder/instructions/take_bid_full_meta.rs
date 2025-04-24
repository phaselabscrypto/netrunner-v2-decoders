use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf2c2cbe1ea350a60")]
pub struct TakeBidFullMeta {
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub meta_args: TMetadataArgs,
    pub min_amount: u64,
    pub optional_royalty_pct: Option<u16>,
}

pub struct TakeBidFullMetaInstructionAccounts {
    pub tcomp: solana_sdk::pubkey::Pubkey,
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub seller: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub bubblegum_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub tensorswap_program: solana_sdk::pubkey::Pubkey,
    pub bid_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub maker_broker: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidFullMeta {
    type ArrangedAccounts = TakeBidFullMetaInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tcomp = accounts.get(0)?;
        let tree_authority = accounts.get(1)?;
        let seller = accounts.get(2)?;
        let delegate = accounts.get(3)?;
        let merkle_tree = accounts.get(4)?;
        let log_wrapper = accounts.get(5)?;
        let compression_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let bubblegum_program = accounts.get(8)?;
        let tcomp_program = accounts.get(9)?;
        let tensorswap_program = accounts.get(10)?;
        let bid_state = accounts.get(11)?;
        let owner = accounts.get(12)?;
        let taker_broker = accounts.get(13)?;
        let maker_broker = accounts.get(14)?;
        let margin_account = accounts.get(15)?;
        let whitelist = accounts.get(16)?;
        let cosigner = accounts.get(17)?;
        let rent_dest = accounts.get(18)?;

        Some(TakeBidFullMetaInstructionAccounts {
            tcomp: tcomp.pubkey,
            tree_authority: tree_authority.pubkey,
            seller: seller.pubkey,
            delegate: delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            tensorswap_program: tensorswap_program.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            margin_account: margin_account.pubkey,
            whitelist: whitelist.pubkey,
            cosigner: cosigner.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}
