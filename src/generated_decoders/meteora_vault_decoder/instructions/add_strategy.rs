

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x407b7fe3c0eac614")]
pub struct AddStrategy{
}

pub struct AddStrategyInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddStrategy {
    type ArrangedAccounts = AddStrategyInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let admin = accounts.get(2)?;

        Some(AddStrategyInstructionAccounts {
            vault: vault.pubkey,
            strategy: strategy.pubkey,
            admin: admin.pubkey,
        })
    }
}