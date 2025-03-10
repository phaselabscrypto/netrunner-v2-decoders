
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x504815bf1d792d6f")]
pub struct SetPerpetualsConfig{
    pub params: SetPerpetualsConfigParams,
}

pub struct SetPerpetualsConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPerpetualsConfig {
    type ArrangedAccounts = SetPerpetualsConfigInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let perpetuals = accounts.get(1)?;

        Some(SetPerpetualsConfigInstructionAccounts {
            admin: admin.pubkey,
            perpetuals: perpetuals.pubkey,
        })
    }
}