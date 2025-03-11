
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7c91173448715509")]
pub struct SellNftTradePoolT22{
    pub config: PoolConfig,
    pub min_price: u64,
}

pub struct SellNftTradePoolT22InstructionAccounts {
    pub shared: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellNftTradePoolT22 {
    type ArrangedAccounts = SellNftTradePoolT22InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let shared = accounts.get(0)?;
        let nft_escrow = accounts.get(1)?;
        let nft_receipt = accounts.get(2)?;
        let token_program = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let margin_account = accounts.get(5)?;
        let taker_broker = accounts.get(6)?;

        Some(SellNftTradePoolT22InstructionAccounts {
            shared: shared.pubkey,
            nft_escrow: nft_escrow.pubkey,
            nft_receipt: nft_receipt.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            margin_account: margin_account.pubkey,
            taker_broker: taker_broker.pubkey,
        })
    }
}