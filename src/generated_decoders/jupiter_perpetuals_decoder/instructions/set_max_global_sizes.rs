use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5902d218a7e30dd6")]
pub struct SetMaxGlobalSizes {
    pub params: SetMaxGlobalSizesParams,
}

pub struct SetMaxGlobalSizesInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetMaxGlobalSizes {
    type ArrangedAccounts = SetMaxGlobalSizesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, custody, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetMaxGlobalSizesInstructionAccounts {
            keeper: keeper.pubkey,
            custody: custody.pubkey,
            pool: pool.pubkey,
        })
    }
}
