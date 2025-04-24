use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3788cd6b6bad041f")]
pub struct Delist {
    pub authorization_data: Option<AuthorizationDataLocal>,
    pub rules_acc_present: bool,
}

pub struct DelistInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub nft_dest: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub single_listing: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub nft_metadata: solana_sdk::pubkey::Pubkey,
    pub nft_edition: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub dest_token_record: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub pnft_shared: solana_sdk::pubkey::Pubkey,
    pub auth_rules: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Delist {
    type ArrangedAccounts = DelistInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let nft_dest = accounts.get(1)?;
        let nft_mint = accounts.get(2)?;
        let nft_escrow = accounts.get(3)?;
        let single_listing = accounts.get(4)?;
        let owner = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;
        let nft_metadata = accounts.get(9)?;
        let nft_edition = accounts.get(10)?;
        let owner_token_record = accounts.get(11)?;
        let dest_token_record = accounts.get(12)?;
        let associated_token_program = accounts.get(13)?;
        let pnft_shared = accounts.get(14)?;
        let auth_rules = accounts.get(15)?;
        let payer = accounts.get(16)?;

        Some(DelistInstructionAccounts {
            tswap: tswap.pubkey,
            nft_dest: nft_dest.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_escrow: nft_escrow.pubkey,
            single_listing: single_listing.pubkey,
            owner: owner.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            nft_metadata: nft_metadata.pubkey,
            nft_edition: nft_edition.pubkey,
            owner_token_record: owner_token_record.pubkey,
            dest_token_record: dest_token_record.pubkey,
            associated_token_program: associated_token_program.pubkey,
            pnft_shared: pnft_shared.pubkey,
            auth_rules: auth_rules.pubkey,
            payer: payer.pubkey,
        })
    }
}
