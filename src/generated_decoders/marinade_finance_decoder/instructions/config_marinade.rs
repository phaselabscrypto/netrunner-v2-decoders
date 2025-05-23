use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x43032272beb9113e")]
pub struct ConfigMarinade {
    pub params: ConfigMarinadeParams,
}

pub struct ConfigMarinadeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigMarinade {
    type ArrangedAccounts = ConfigMarinadeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let admin_authority = accounts.get(1)?;

        Some(ConfigMarinadeInstructionAccounts {
            state: state.pubkey,
            admin_authority: admin_authority.pubkey,
        })
    }
}
