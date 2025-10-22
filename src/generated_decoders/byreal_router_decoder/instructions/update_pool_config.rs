use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x44eccb7ab33eeafc")]
pub struct UpdatePoolConfig {
    pub params: UpdatePoolConfigParams,
}

pub struct UpdatePoolConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub pool_config: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePoolConfig {
    type ArrangedAccounts = UpdatePoolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let pool_config = accounts.get(1)?;

        Some(UpdatePoolConfigInstructionAccounts {
            admin: admin.pubkey,
            pool_config: pool_config.pubkey,
        })
    }
}
