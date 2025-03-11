
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xbc23746c00e9edc9")]
pub struct TakeBidLegacy{
    pub min_amount: u64,
    pub optional_royalty_pct: Option<u16>,
    pub rules_acc_present: bool,
    pub authorization_data: Option<AuthorizationDataLocal>,
}

pub struct TakeBidLegacyInstructionAccounts {
    pub tcomp: solana_sdk::pubkey::Pubkey,
    pub seller: solana_sdk::pubkey::Pubkey,
    pub bid_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub maker_broker: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub nft_seller_acc: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_metadata: solana_sdk::pubkey::Pubkey,
    pub owner_ata_acc: solana_sdk::pubkey::Pubkey,
    pub nft_edition: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub dest_token_record: solana_sdk::pubkey::Pubkey,
    pub pnft_shared: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub temp_escrow_token_record: solana_sdk::pubkey::Pubkey,
    pub auth_rules: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub tensorswap_program: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub mint_proof: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidLegacy {
    type ArrangedAccounts = TakeBidLegacyInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let tcomp = accounts.get(0)?;
        let seller = accounts.get(1)?;
        let bid_state = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let taker_broker = accounts.get(4)?;
        let maker_broker = accounts.get(5)?;
        let margin_account = accounts.get(6)?;
        let whitelist = accounts.get(7)?;
        let nft_seller_acc = accounts.get(8)?;
        let nft_mint = accounts.get(9)?;
        let nft_metadata = accounts.get(10)?;
        let owner_ata_acc = accounts.get(11)?;
        let nft_edition = accounts.get(12)?;
        let owner_token_record = accounts.get(13)?;
        let dest_token_record = accounts.get(14)?;
        let pnft_shared = accounts.get(15)?;
        let nft_escrow = accounts.get(16)?;
        let temp_escrow_token_record = accounts.get(17)?;
        let auth_rules = accounts.get(18)?;
        let token_program = accounts.get(19)?;
        let associated_token_program = accounts.get(20)?;
        let system_program = accounts.get(21)?;
        let tcomp_program = accounts.get(22)?;
        let tensorswap_program = accounts.get(23)?;
        let cosigner = accounts.get(24)?;
        let mint_proof = accounts.get(25)?;
        let rent_dest = accounts.get(26)?;

        Some(TakeBidLegacyInstructionAccounts {
            tcomp: tcomp.pubkey,
            seller: seller.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            margin_account: margin_account.pubkey,
            whitelist: whitelist.pubkey,
            nft_seller_acc: nft_seller_acc.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_metadata: nft_metadata.pubkey,
            owner_ata_acc: owner_ata_acc.pubkey,
            nft_edition: nft_edition.pubkey,
            owner_token_record: owner_token_record.pubkey,
            dest_token_record: dest_token_record.pubkey,
            pnft_shared: pnft_shared.pubkey,
            nft_escrow: nft_escrow.pubkey,
            temp_escrow_token_record: temp_escrow_token_record.pubkey,
            auth_rules: auth_rules.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            tensorswap_program: tensorswap_program.pubkey,
            cosigner: cosigner.pubkey,
            mint_proof: mint_proof.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}