use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x26cb48e7671dc33d")]
pub struct CreateProrataConfig {
    pub config_parameters: ProrataConfigParameters,
}

pub struct CreateProrataConfigInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateProrataConfig {
    type ArrangedAccounts = CreateProrataConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let admin = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(CreateProrataConfigInstructionAccounts {
            config: config.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
