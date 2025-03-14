
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1fa2e49e99a0c6b6")]
pub struct TammNoop{
    pub event: TAmmEvent,
}

pub struct TammNoopInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TammNoop {
    type ArrangedAccounts = TammNoopInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;

        Some(TammNoopInstructionAccounts {
            pool: pool.pubkey,
        })
    }
}