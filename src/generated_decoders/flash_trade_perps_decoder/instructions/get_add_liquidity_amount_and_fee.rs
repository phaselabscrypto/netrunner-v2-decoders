use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xac96f9b5e9f14e8b")]
pub struct GetAddLiquidityAmountAndFee {
    pub params: GetAddLiquidityAmountAndFeeParams,
}

pub struct GetAddLiquidityAmountAndFeeInstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetAddLiquidityAmountAndFee {
    type ArrangedAccounts = GetAddLiquidityAmountAndFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, custody, custody_oracle_account, lp_token_mint, ix_sysvar, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GetAddLiquidityAmountAndFeeInstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_oracle_account: custody_oracle_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
