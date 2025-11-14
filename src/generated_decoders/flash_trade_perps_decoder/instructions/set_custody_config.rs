use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8561828fd7e524b0")]
pub struct SetCustodyConfig {
    pub params: SetCustodyConfigParams,
}

pub struct SetCustodyConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCustodyConfig {
    type ArrangedAccounts = SetCustodyConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, pool, custody, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetCustodyConfigInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
