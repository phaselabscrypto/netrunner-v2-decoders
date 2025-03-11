

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7c91173448715509")]
pub struct SellNftTradePoolT22{
    pub min_price: u64,
}

pub struct SellNftTradePoolT22InstructionAccounts {
    pub trade: solana_sdk::pubkey::Pubkey,
    pub t22: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub taker_ta: solana_sdk::pubkey::Pubkey,
    pub pool_ta: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellNftTradePoolT22 {
    type ArrangedAccounts = SellNftTradePoolT22InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let trade = accounts.get(0)?;
        let t22 = accounts.get(1)?;
        let nft_receipt = accounts.get(2)?;
        let taker_ta = accounts.get(3)?;
        let pool_ta = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let associated_token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;

        Some(SellNftTradePoolT22InstructionAccounts {
            trade: trade.pubkey,
            t22: t22.pubkey,
            nft_receipt: nft_receipt.pubkey,
            taker_ta: taker_ta.pubkey,
            pool_ta: pool_ta.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}