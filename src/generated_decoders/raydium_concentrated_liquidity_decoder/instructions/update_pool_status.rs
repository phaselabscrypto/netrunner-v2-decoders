

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x82576c062ee0757b")]
pub struct UpdatePoolStatus{
    pub status: u8,
}

pub struct UpdatePoolStatusInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePoolStatus {
    type ArrangedAccounts = UpdatePoolStatusInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let pool_state = accounts.get(1)?;

        Some(UpdatePoolStatusInstructionAccounts {
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
        })
    }
}