use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x069b6711e4ac0ea0")]
pub struct SetProtocolFeeShare {
    pub params: SetProtocolFeeShareParams,
}

pub struct SetProtocolFeeShareInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub protocol_vault: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetProtocolFeeShare {
    type ArrangedAccounts = SetProtocolFeeShareInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, protocol_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetProtocolFeeShareInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            protocol_vault: protocol_vault.pubkey,
        })
    }
}
