use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x32569c49954ea386")]
pub struct UnstakeRequest {
    pub params: UnstakeRequestParams,
}

pub struct UnstakeRequestInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub flp_stake_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnstakeRequest {
    type ArrangedAccounts = UnstakeRequestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, perpetuals, pool, flp_stake_account, system_program, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UnstakeRequestInstructionAccounts {
            owner: owner.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            flp_stake_account: flp_stake_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
