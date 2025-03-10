
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8561828fd7e524b0")]
pub struct SetCustodyConfig{
    pub params: SetCustodyConfigParams,
}

pub struct SetCustodyConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCustodyConfig {
    type ArrangedAccounts = SetCustodyConfigInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let perpetuals = accounts.get(1)?;
        let custody = accounts.get(2)?;

        Some(SetCustodyConfigInstructionAccounts {
            admin: admin.pubkey,
            perpetuals: perpetuals.pubkey,
            custody: custody.pubkey,
        })
    }
}