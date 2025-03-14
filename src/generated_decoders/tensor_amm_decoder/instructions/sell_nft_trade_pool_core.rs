

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x25cd8d3556f52d4e")]
pub struct SellNftTradePoolCore{
    pub min_price: u64,
}

pub struct SellNftTradePoolCoreInstructionAccounts {
    pub trade: solana_sdk::pubkey::Pubkey,
    pub core: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellNftTradePoolCore {
    type ArrangedAccounts = SellNftTradePoolCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let trade = accounts.get(0)?;
        let core = accounts.get(1)?;
        let nft_receipt = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(SellNftTradePoolCoreInstructionAccounts {
            trade: trade.pubkey,
            core: core.pubkey,
            nft_receipt: nft_receipt.pubkey,
            system_program: system_program.pubkey,
        })
    }
}