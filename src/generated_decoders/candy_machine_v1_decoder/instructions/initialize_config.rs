
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd07f1501c2bec446")]
pub struct InitializeConfig{
    pub data: ConfigData,
}

pub struct InitializeConfigInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeConfig {
    type ArrangedAccounts = InitializeConfigInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let rent = accounts.get(3)?;

        Some(InitializeConfigInstructionAccounts {
            config: config.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
        })
    }
}