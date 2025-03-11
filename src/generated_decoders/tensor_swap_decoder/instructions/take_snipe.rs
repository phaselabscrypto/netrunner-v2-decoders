
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0a9730e2f818e3e7")]
pub struct TakeSnipe{
    pub config: PoolConfig,
    pub actual_price: u64,
    pub authorization_data: Option<AuthorizationDataLocal>,
}

pub struct TakeSnipeInstructionAccounts {
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeSnipe {
    type ArrangedAccounts = TakeSnipeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let system_program = accounts.get(0)?;

        Some(TakeSnipeInstructionAccounts {
            system_program: system_program.pubkey,
        })
    }
}