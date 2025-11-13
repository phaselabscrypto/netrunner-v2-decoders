use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd857417d716eb978")]
pub struct SetPoolConfig {
    pub params: SetPoolConfigParams,
}

pub struct SetPoolConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPoolConfig {
    type ArrangedAccounts = SetPoolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, pool, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetPoolConfigInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            pool: pool.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
