use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9908168a69b05742")]
pub struct WithdrawStake {
    pub params: WithdrawStakeParams,
}

pub struct WithdrawStakeInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_lp_token_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub flp_stake_account: solana_sdk::pubkey::Pubkey,
    pub pool_staked_lp_vault: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawStake {
    type ArrangedAccounts = WithdrawStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, receiving_lp_token_account, transfer_authority, perpetuals, pool, flp_stake_account, pool_staked_lp_vault, system_program, token_program, event_authority, program, lp_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawStakeInstructionAccounts {
            owner: owner.pubkey,
            receiving_lp_token_account: receiving_lp_token_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            flp_stake_account: flp_stake_account.pubkey,
            pool_staked_lp_vault: pool_staked_lp_vault.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            lp_mint: lp_mint.pubkey,
        })
    }
}
