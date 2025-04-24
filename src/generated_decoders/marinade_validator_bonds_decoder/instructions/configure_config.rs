use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc662a1a589c8e6cb")]
pub struct ConfigureConfig {
    pub configure_config_args: ConfigureConfigArgs,
}

pub struct ConfigureConfigInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigureConfig {
    type ArrangedAccounts = ConfigureConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let admin_authority = accounts.get(1)?;
        let event_authority = accounts.get(2)?;
        let program = accounts.get(3)?;

        Some(ConfigureConfigInstructionAccounts {
            config: config.pubkey,
            admin_authority: admin_authority.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
