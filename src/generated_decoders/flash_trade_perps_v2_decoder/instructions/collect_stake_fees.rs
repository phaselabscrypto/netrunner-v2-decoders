use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3daee1a56791fab5")]
pub struct CollectStakeFees {
    pub params: CollectStakeRewardParams,
}

pub struct CollectStakeFeesInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_token_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub fee_custody: solana_sdk::pubkey::Pubkey,
    pub flp_stake_account: solana_sdk::pubkey::Pubkey,
    pub fee_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub receiving_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectStakeFees {
    type ArrangedAccounts = CollectStakeFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, receiving_token_account, transfer_authority, perpetuals, pool, fee_custody, flp_stake_account, fee_custody_token_account, system_program, token_program, event_authority, program, ix_sysvar, receiving_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectStakeFeesInstructionAccounts {
            owner: owner.pubkey,
            receiving_token_account: receiving_token_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            fee_custody: fee_custody.pubkey,
            flp_stake_account: flp_stake_account.pubkey,
            fee_custody_token_account: fee_custody_token_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            receiving_mint: receiving_mint.pubkey,
        })
    }
}
