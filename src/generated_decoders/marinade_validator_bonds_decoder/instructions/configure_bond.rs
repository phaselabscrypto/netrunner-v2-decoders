
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe46c4ff252366941")]
pub struct ConfigureBond{
    pub configure_bond_args: ConfigureBondArgs,
}

pub struct ConfigureBondInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigureBond {
    type ArrangedAccounts = ConfigureBondInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let vote_account = accounts.get(3)?;
        let event_authority = accounts.get(4)?;
        let program = accounts.get(5)?;

        Some(ConfigureBondInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            authority: authority.pubkey,
            vote_account: vote_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}