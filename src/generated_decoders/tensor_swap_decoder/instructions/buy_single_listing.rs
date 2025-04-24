use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf5dc694975624e8d")]
pub struct BuySingleListing {
    pub max_price: u64,
    pub rules_acc_present: bool,
    pub authorization_data: Option<AuthorizationDataLocal>,
    pub optional_royalty_pct: Option<u16>,
}

pub struct BuySingleListingInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub single_listing: solana_sdk::pubkey::Pubkey,
    pub nft_buyer_acc: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_metadata: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub nft_edition: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub dest_token_record: solana_sdk::pubkey::Pubkey,
    pub pnft_shared: solana_sdk::pubkey::Pubkey,
    pub auth_rules: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuySingleListing {
    type ArrangedAccounts = BuySingleListingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let fee_vault = accounts.get(1)?;
        let single_listing = accounts.get(2)?;
        let nft_buyer_acc = accounts.get(3)?;
        let nft_mint = accounts.get(4)?;
        let nft_metadata = accounts.get(5)?;
        let nft_escrow = accounts.get(6)?;
        let owner = accounts.get(7)?;
        let buyer = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let rent = accounts.get(12)?;
        let nft_edition = accounts.get(13)?;
        let owner_token_record = accounts.get(14)?;
        let dest_token_record = accounts.get(15)?;
        let pnft_shared = accounts.get(16)?;
        let auth_rules = accounts.get(17)?;
        let taker_broker = accounts.get(18)?;

        Some(BuySingleListingInstructionAccounts {
            tswap: tswap.pubkey,
            fee_vault: fee_vault.pubkey,
            single_listing: single_listing.pubkey,
            nft_buyer_acc: nft_buyer_acc.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_metadata: nft_metadata.pubkey,
            nft_escrow: nft_escrow.pubkey,
            owner: owner.pubkey,
            buyer: buyer.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            nft_edition: nft_edition.pubkey,
            owner_token_record: owner_token_record.pubkey,
            dest_token_record: dest_token_record.pubkey,
            pnft_shared: pnft_shared.pubkey,
            auth_rules: auth_rules.pubkey,
            taker_broker: taker_broker.pubkey,
        })
    }
}
