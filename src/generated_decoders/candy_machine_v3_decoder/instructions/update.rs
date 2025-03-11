

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdbc858b09e3ffd7f")]
pub struct Update{
    pub data: Vec<u8>,
}

pub struct UpdateInstructionAccounts {
    pub candy_guard: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Update {
    type ArrangedAccounts = UpdateInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let candy_guard = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(UpdateInstructionAccounts {
            candy_guard: candy_guard.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}