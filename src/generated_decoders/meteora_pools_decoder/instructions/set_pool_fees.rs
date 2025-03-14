
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x662c9e36cd257e4e")]
pub struct SetPoolFees{
    pub fees: PoolFees,
    pub new_partner_fee_numerator: u64,
}

pub struct SetPoolFeesInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub fee_operator: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPoolFees {
    type ArrangedAccounts = SetPoolFeesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let fee_operator = accounts.get(1)?;

        Some(SetPoolFeesInstructionAccounts {
            pool: pool.pubkey,
            fee_operator: fee_operator.pubkey,
        })
    }
}