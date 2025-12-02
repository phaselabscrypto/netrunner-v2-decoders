use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x842a358a1cdcaa37")]
pub struct RemovePool {
    pub params: RemovePoolParams,
}

pub struct RemovePoolInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemovePool {
    type ArrangedAccounts = RemovePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, transfer_authority, perpetuals, pool, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RemovePoolInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
