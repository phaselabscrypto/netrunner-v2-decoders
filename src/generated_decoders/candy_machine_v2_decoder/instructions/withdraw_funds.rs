

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf1241d6fd01f68d9")]
pub struct WithdrawFunds{
}

pub struct WithdrawFundsInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFunds {
    type ArrangedAccounts = WithdrawFundsInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;

        Some(WithdrawFundsInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
        })
    }
}