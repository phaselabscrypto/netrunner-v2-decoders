use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf0ab8d697c02e1bc")]
pub struct SetAdminSigners {
    pub params: SetAdminSignersParams,
}

pub struct SetAdminSignersInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetAdminSigners {
    type ArrangedAccounts = SetAdminSignersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetAdminSignersInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
        })
    }
}
