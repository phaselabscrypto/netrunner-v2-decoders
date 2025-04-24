use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17eb73e8a86001e7")]
pub struct InitConfig {
    pub init_config_args: InitConfigArgs,
}

pub struct InitConfigInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitConfig {
    type ArrangedAccounts = InitConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let rent_payer = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let event_authority = accounts.get(3)?;
        let program = accounts.get(4)?;

        Some(InitConfigInstructionAccounts {
            config: config.pubkey,
            rent_payer: rent_payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
