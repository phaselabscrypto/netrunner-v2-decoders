
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x392cc03053086b30")]
pub struct SellNftTokenPool{
    pub config: PoolConfig,
    pub min_price: u64,
    pub rules_acc_present: bool,
    pub authorization_data: Option<AuthorizationDataLocal>,
    pub optional_royalty_pct: Option<u16>,
}

pub struct SellNftTokenPoolInstructionAccounts {
    pub shared: solana_sdk::pubkey::Pubkey,
    pub owner_ata_acc: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub nft_edition: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub dest_token_record: solana_sdk::pubkey::Pubkey,
    pub pnft_shared: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub temp_escrow_token_record: solana_sdk::pubkey::Pubkey,
    pub auth_rules: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellNftTokenPool {
    type ArrangedAccounts = SellNftTokenPoolInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let shared = accounts.get(0)?;
        let owner_ata_acc = accounts.get(1)?;
        let token_program = accounts.get(2)?;
        let associated_token_program = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let rent = accounts.get(5)?;
        let nft_edition = accounts.get(6)?;
        let owner_token_record = accounts.get(7)?;
        let dest_token_record = accounts.get(8)?;
        let pnft_shared = accounts.get(9)?;
        let nft_escrow = accounts.get(10)?;
        let temp_escrow_token_record = accounts.get(11)?;
        let auth_rules = accounts.get(12)?;
        let margin_account = accounts.get(13)?;
        let taker_broker = accounts.get(14)?;

        Some(SellNftTokenPoolInstructionAccounts {
            shared: shared.pubkey,
            owner_ata_acc: owner_ata_acc.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            nft_edition: nft_edition.pubkey,
            owner_token_record: owner_token_record.pubkey,
            dest_token_record: dest_token_record.pubkey,
            pnft_shared: pnft_shared.pubkey,
            nft_escrow: nft_escrow.pubkey,
            temp_escrow_token_record: temp_escrow_token_record.pubkey,
            auth_rules: auth_rules.pubkey,
            margin_account: margin_account.pubkey,
            taker_broker: taker_broker.pubkey,
        })
    }
}