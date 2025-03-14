

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xee9965a9f3832401")]
pub struct SetOperator{
}

pub struct SetOperatorInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub operator: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetOperator {
    type ArrangedAccounts = SetOperatorInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let operator = accounts.get(1)?;
        let admin = accounts.get(2)?;

        Some(SetOperatorInstructionAccounts {
            vault: vault.pubkey,
            operator: operator.pubkey,
            admin: admin.pubkey,
        })
    }
}