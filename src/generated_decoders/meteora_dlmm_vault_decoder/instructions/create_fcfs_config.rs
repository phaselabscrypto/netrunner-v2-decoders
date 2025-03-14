
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x07fff2f20163b30c")]
pub struct CreateFcfsConfig{
    pub config_parameters: FcfsConfigParameters,
}

pub struct CreateFcfsConfigInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateFcfsConfig {
    type ArrangedAccounts = CreateFcfsConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let admin = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(CreateFcfsConfigInstructionAccounts {
            config: config.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}