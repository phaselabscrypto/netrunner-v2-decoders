use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2e1e3907e1c65ca4")]
pub struct SetInternalEmaPrice {
    pub params: SetInternalEmaPriceParams,
}

pub struct SetInternalEmaPriceInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetInternalEmaPrice {
    type ArrangedAccounts = SetInternalEmaPriceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetInternalEmaPriceInstructionAccounts {
            authority: authority.pubkey,
        })
    }
}
