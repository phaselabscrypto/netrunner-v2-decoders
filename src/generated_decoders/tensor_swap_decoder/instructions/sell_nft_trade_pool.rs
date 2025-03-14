
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x83527d4d0d9d245a")]
pub struct SellNftTradePool{
    pub config: PoolConfig,
    pub min_price: u64,
    pub rules_acc_present: bool,
    pub authorization_data: Option<AuthorizationDataLocal>,
    pub optional_royalty_pct: Option<u16>,
}

pub struct SellNftTradePoolInstructionAccounts {
    pub shared: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub nft_edition: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub dest_token_record: solana_sdk::pubkey::Pubkey,
    pub pnft_shared: solana_sdk::pubkey::Pubkey,
    pub auth_rules: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellNftTradePool {
    type ArrangedAccounts = SellNftTradePoolInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let shared = accounts.get(0)?;
        let nft_escrow = accounts.get(1)?;
        let nft_receipt = accounts.get(2)?;
        let token_program = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let rent = accounts.get(5)?;
        let associated_token_program = accounts.get(6)?;
        let nft_edition = accounts.get(7)?;
        let owner_token_record = accounts.get(8)?;
        let dest_token_record = accounts.get(9)?;
        let pnft_shared = accounts.get(10)?;
        let auth_rules = accounts.get(11)?;
        let margin_account = accounts.get(12)?;
        let taker_broker = accounts.get(13)?;

        Some(SellNftTradePoolInstructionAccounts {
            shared: shared.pubkey,
            nft_escrow: nft_escrow.pubkey,
            nft_receipt: nft_receipt.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            associated_token_program: associated_token_program.pubkey,
            nft_edition: nft_edition.pubkey,
            owner_token_record: owner_token_record.pubkey,
            dest_token_record: dest_token_record.pubkey,
            pnft_shared: pnft_shared.pubkey,
            auth_rules: auth_rules.pubkey,
            margin_account: margin_account.pubkey,
            taker_broker: taker_broker.pubkey,
        })
    }
}