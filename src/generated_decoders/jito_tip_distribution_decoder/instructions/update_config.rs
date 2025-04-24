use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1d9efcbf0a53db63")]
pub struct UpdateConfig {
    pub new_config: Config,
}

pub struct UpdateConfigInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateConfig {
    type ArrangedAccounts = UpdateConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let authority = accounts.get(1)?;

        Some(UpdateConfigInstructionAccounts {
            config: config.pubkey,
            authority: authority.pubkey,
        })
    }
}
