use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb2051a55381499a0")]
pub struct MigrateStake {
    pub params: MigrateStakeParams,
}

pub struct MigrateStakeInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub compounding_token_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub flp_stake_account: solana_sdk::pubkey::Pubkey,
    pub reward_custody: solana_sdk::pubkey::Pubkey,
    pub reward_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub pool_staked_lp_vault: solana_sdk::pubkey::Pubkey,
    pub pool_compounding_lp_vault: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub compounding_token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateStake {
    type ArrangedAccounts = MigrateStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, compounding_token_account, transfer_authority, perpetuals, pool, flp_stake_account, reward_custody, reward_custody_oracle_account, pool_staked_lp_vault, pool_compounding_lp_vault, lp_token_mint, compounding_token_mint, token_program, event_authority, program, ix_sysvar, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrateStakeInstructionAccounts {
            owner: owner.pubkey,
            compounding_token_account: compounding_token_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            flp_stake_account: flp_stake_account.pubkey,
            reward_custody: reward_custody.pubkey,
            reward_custody_oracle_account: reward_custody_oracle_account.pubkey,
            pool_staked_lp_vault: pool_staked_lp_vault.pubkey,
            pool_compounding_lp_vault: pool_compounding_lp_vault.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            compounding_token_mint: compounding_token_mint.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
