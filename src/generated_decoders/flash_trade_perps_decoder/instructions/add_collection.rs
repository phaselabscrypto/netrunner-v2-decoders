use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4face18edbc0ab50")]
pub struct AddCollection {
    pub params: AddCollectionParams,
}

pub struct AddCollectionInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddCollection {
    type ArrangedAccounts = AddCollectionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, perpetuals, collection_mint, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddCollectionInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            perpetuals: perpetuals.pubkey,
            collection_mint: collection_mint.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
