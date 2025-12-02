use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1002ca282e39043f")]
pub struct SwapFeeInternal {
    pub params: SwapFeeInternalParams,
}

pub struct SwapFeeInternalInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub reward_custody: solana_sdk::pubkey::Pubkey,
    pub reward_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub reward_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapFeeInternal {
    type ArrangedAccounts = SwapFeeInternalInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, perpetuals, pool, reward_custody, reward_custody_oracle_account, reward_custody_token_account, event_authority, program, ix_sysvar, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapFeeInternalInstructionAccounts {
            owner: owner.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            reward_custody: reward_custody.pubkey,
            reward_custody_oracle_account: reward_custody_oracle_account.pubkey,
            reward_custody_token_account: reward_custody_token_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
