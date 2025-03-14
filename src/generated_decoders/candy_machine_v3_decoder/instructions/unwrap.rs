

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7eafc60ed445322c")]
pub struct Unwrap{
}

pub struct UnwrapInstructionAccounts {
    pub candy_guard: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub candy_machine_authority: solana_sdk::pubkey::Pubkey,
    pub candy_machine_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Unwrap {
    type ArrangedAccounts = UnwrapInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let candy_guard = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let candy_machine = accounts.get(2)?;
        let candy_machine_authority = accounts.get(3)?;
        let candy_machine_program = accounts.get(4)?;

        Some(UnwrapInstructionAccounts {
            candy_guard: candy_guard.pubkey,
            authority: authority.pubkey,
            candy_machine: candy_machine.pubkey,
            candy_machine_authority: candy_machine_authority.pubkey,
            candy_machine_program: candy_machine_program.pubkey,
        })
    }
}