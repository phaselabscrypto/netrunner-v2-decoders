

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x158f1b8ec8b5d2ff")]
pub struct EmergencyPause{
}

pub struct EmergencyPauseInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub pause_authority: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EmergencyPause {
    type ArrangedAccounts = EmergencyPauseInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let pause_authority = accounts.get(1)?;
        let event_authority = accounts.get(2)?;
        let program = accounts.get(3)?;

        Some(EmergencyPauseInstructionAccounts {
            config: config.pubkey,
            pause_authority: pause_authority.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}