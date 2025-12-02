use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1d01903abee47de5")]
pub struct GetAddCompoundingLiquidityAmountAndFee {
    pub params: GetAddCompoundingLiquidityAmountAndFeeParams,
}

pub struct GetAddCompoundingLiquidityAmountAndFeeInstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub in_custody: solana_sdk::pubkey::Pubkey,
    pub in_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub reward_custody: solana_sdk::pubkey::Pubkey,
    pub reward_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub compounding_token_mint: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetAddCompoundingLiquidityAmountAndFee {
    type ArrangedAccounts = GetAddCompoundingLiquidityAmountAndFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, in_custody, in_custody_oracle_account, reward_custody, reward_custody_oracle_account, lp_token_mint, compounding_token_mint, ix_sysvar, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GetAddCompoundingLiquidityAmountAndFeeInstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            in_custody: in_custody.pubkey,
            in_custody_oracle_account: in_custody_oracle_account.pubkey,
            reward_custody: reward_custody.pubkey,
            reward_custody_oracle_account: reward_custody_oracle_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            compounding_token_mint: compounding_token_mint.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
