use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd6a569b6d5a2d422")]
pub struct SetPermissions {
    pub params: SetPermissionsParams,
}

pub struct SetPermissionsInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPermissions {
    type ArrangedAccounts = SetPermissionsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, perpetuals, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetPermissionsInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            perpetuals: perpetuals.pubkey,
        })
    }
}
