use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x93e09f03a293c7f4")]
pub struct AddLiquidityAndStake {
    pub params: AddLiquidityAndStakeParams,
}

pub struct AddLiquidityAndStakeInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub custody_token_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub flp_stake_account: solana_sdk::pubkey::Pubkey,
    pub pool_staked_lp_vault: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub funding_mint: solana_sdk::pubkey::Pubkey,
    pub funding_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidityAndStake {
    type ArrangedAccounts = AddLiquidityAndStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, fee_payer, funding_account, transfer_authority, perpetuals, pool, custody, custody_oracle_account, custody_token_account, lp_token_mint, flp_stake_account, pool_staked_lp_vault, system_program, token_program, event_authority, program, ix_sysvar, funding_mint, funding_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddLiquidityAndStakeInstructionAccounts {
            owner: owner.pubkey,
            fee_payer: fee_payer.pubkey,
            funding_account: funding_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_oracle_account: custody_oracle_account.pubkey,
            custody_token_account: custody_token_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            flp_stake_account: flp_stake_account.pubkey,
            pool_staked_lp_vault: pool_staked_lp_vault.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            funding_mint: funding_mint.pubkey,
            funding_token_program: funding_token_program.pubkey,
        })
    }
}
