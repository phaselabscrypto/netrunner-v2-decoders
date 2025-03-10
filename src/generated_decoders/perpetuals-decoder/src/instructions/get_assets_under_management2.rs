
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc1d20df971951d54")]
pub struct GetAssetsUnderManagement2{
    pub params: GetAssetsUnderManagement2Params,
}

pub struct GetAssetsUnderManagement2InstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetAssetsUnderManagement2 {
    type ArrangedAccounts = GetAssetsUnderManagement2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let perpetuals = accounts.get(0)?;
        let pool = accounts.get(1)?;

        Some(GetAssetsUnderManagement2InstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
        })
    }
}