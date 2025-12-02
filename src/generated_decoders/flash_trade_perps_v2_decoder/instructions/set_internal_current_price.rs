use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbbf22dcbd607d3d5")]
pub struct SetInternalCurrentPrice {
    pub params: SetInternalCurrentPriceParams,
}

pub struct SetInternalCurrentPriceInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetInternalCurrentPrice {
    type ArrangedAccounts = SetInternalCurrentPriceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetInternalCurrentPriceInstructionAccounts {
            authority: authority.pubkey,
        })
    }
}
