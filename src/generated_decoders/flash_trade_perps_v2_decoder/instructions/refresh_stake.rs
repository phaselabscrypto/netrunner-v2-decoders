use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc27b28f725ed7777")]
pub struct RefreshStake {
    pub params: RefreshStakeParams,
}

pub struct RefreshStakeInstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub reward_custody: solana_sdk::pubkey::Pubkey,
    pub fee_distribution_token_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RefreshStake {
    type ArrangedAccounts = RefreshStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, reward_custody, fee_distribution_token_account, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RefreshStakeInstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            reward_custody: reward_custody.pubkey,
            fee_distribution_token_account: fee_distribution_token_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
