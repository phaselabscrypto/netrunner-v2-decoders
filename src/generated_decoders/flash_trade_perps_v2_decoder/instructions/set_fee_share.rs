use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf4c802fafe7b4e5d")]
pub struct SetFeeShare {
    pub params: SetFeeShareParams,
}

pub struct SetFeeShareInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFeeShare {
    type ArrangedAccounts = SetFeeShareInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetFeeShareInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            pool: pool.pubkey,
        })
    }
}
