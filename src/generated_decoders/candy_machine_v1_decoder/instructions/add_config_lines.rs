
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdf32e0e39708736a")]
pub struct AddConfigLines{
    pub index: u32,
    pub config_lines: Vec<ConfigLine>,
}

pub struct AddConfigLinesInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddConfigLines {
    type ArrangedAccounts = AddConfigLinesInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let authority = accounts.get(1)?;

        Some(AddConfigLinesInstructionAccounts {
            config: config.pubkey,
            authority: authority.pubkey,
        })
    }
}