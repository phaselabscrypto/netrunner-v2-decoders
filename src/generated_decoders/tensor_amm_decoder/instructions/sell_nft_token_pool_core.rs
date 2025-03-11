

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x89e3c57af5e538cd")]
pub struct SellNftTokenPoolCore{
    pub min_price: u64,
}

pub struct SellNftTokenPoolCoreInstructionAccounts {
    pub trade: solana_sdk::pubkey::Pubkey,
    pub core: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellNftTokenPoolCore {
    type ArrangedAccounts = SellNftTokenPoolCoreInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let trade = accounts.get(0)?;
        let core = accounts.get(1)?;

        Some(SellNftTokenPoolCoreInstructionAccounts {
            trade: trade.pubkey,
            core: core.pubkey,
        })
    }
}