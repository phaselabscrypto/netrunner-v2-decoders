use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x771ba18b154e8242")]
pub struct UnstakeInstant {
    pub params: UnstakeInstantParams,
}

pub struct UnstakeInstantInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub flp_stake_account: solana_sdk::pubkey::Pubkey,
    pub reward_custody: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnstakeInstant {
    type ArrangedAccounts = UnstakeInstantInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, perpetuals, pool, flp_stake_account, reward_custody, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UnstakeInstantInstructionAccounts {
            owner: owner.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            flp_stake_account: flp_stake_account.pubkey,
            reward_custody: reward_custody.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
