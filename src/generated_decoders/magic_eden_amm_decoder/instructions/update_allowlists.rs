
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1c4edeebd144b653")]
pub struct UpdateAllowlists{
    pub args: UpdateAllowlistsArgs,
}

pub struct UpdateAllowlistsInstructionAccounts {
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAllowlists {
    type ArrangedAccounts = UpdateAllowlistsInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let cosigner = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let pool = accounts.get(2)?;

        Some(UpdateAllowlistsInstructionAccounts {
            cosigner: cosigner.pubkey,
            owner: owner.pubkey,
            pool: pool.pubkey,
        })
    }
}