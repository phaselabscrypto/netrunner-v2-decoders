
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x32ae222403a61dcc")]
pub struct EditPool{
    pub args: EditPoolArgs,
}

pub struct EditPoolInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditPool {
    type ArrangedAccounts = EditPoolInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(EditPoolInstructionAccounts {
            owner: owner.pubkey,
            pool: pool.pubkey,
            system_program: system_program.pubkey,
        })
    }
}