use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00f330b90649be53")]
pub struct EmergencyResume {}

pub struct EmergencyResumeInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub pause_authority: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EmergencyResume {
    type ArrangedAccounts = EmergencyResumeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let pause_authority = accounts.get(1)?;
        let event_authority = accounts.get(2)?;
        let program = accounts.get(3)?;

        Some(EmergencyResumeInstructionAccounts {
            config: config.pubkey,
            pause_authority: pause_authority.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
