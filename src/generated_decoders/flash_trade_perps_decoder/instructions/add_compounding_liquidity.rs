use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf4e72ac0be860334")]
pub struct AddCompoundingLiquidity {
    pub params: AddCompoundingLiquidityParams,
}

pub struct AddCompoundingLiquidityInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub compounding_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_compounding_lp_vault: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub in_custody: solana_sdk::pubkey::Pubkey,
    pub in_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub in_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub reward_custody: solana_sdk::pubkey::Pubkey,
    pub reward_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub compounding_token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub funding_mint: solana_sdk::pubkey::Pubkey,
    pub funding_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddCompoundingLiquidity {
    type ArrangedAccounts = AddCompoundingLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, funding_account, compounding_token_account, pool_compounding_lp_vault, transfer_authority, perpetuals, pool, in_custody, in_custody_oracle_account, in_custody_token_account, reward_custody, reward_custody_oracle_account, lp_token_mint, compounding_token_mint, token_program, event_authority, program, ix_sysvar, funding_mint, funding_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddCompoundingLiquidityInstructionAccounts {
            owner: owner.pubkey,
            funding_account: funding_account.pubkey,
            compounding_token_account: compounding_token_account.pubkey,
            pool_compounding_lp_vault: pool_compounding_lp_vault.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            in_custody: in_custody.pubkey,
            in_custody_oracle_account: in_custody_oracle_account.pubkey,
            in_custody_token_account: in_custody_token_account.pubkey,
            reward_custody: reward_custody.pubkey,
            reward_custody_oracle_account: reward_custody_oracle_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            compounding_token_mint: compounding_token_mint.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            funding_mint: funding_mint.pubkey,
            funding_token_program: funding_token_program.pubkey,
        })
    }
}
