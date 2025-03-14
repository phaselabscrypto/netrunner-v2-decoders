

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa3663a6bb804a979")]
pub struct BuyNftCore{
    pub max_amount: u64,
}

pub struct BuyNftCoreInstructionAccounts {
    pub trade: solana_sdk::pubkey::Pubkey,
    pub core: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuyNftCore {
    type ArrangedAccounts = BuyNftCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let trade = accounts.get(0)?;
        let core = accounts.get(1)?;
        let nft_receipt = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(BuyNftCoreInstructionAccounts {
            trade: trade.pubkey,
            core: core.pubkey,
            nft_receipt: nft_receipt.pubkey,
            system_program: system_program.pubkey,
        })
    }
}