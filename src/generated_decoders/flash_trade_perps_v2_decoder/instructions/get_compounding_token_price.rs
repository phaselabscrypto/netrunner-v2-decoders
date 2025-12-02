use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8152b6885fab2c3f")]
pub struct GetCompoundingTokenPrice {
    pub params: GetCompoundingTokenPriceParams,
}

pub struct GetCompoundingTokenPriceInstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetCompoundingTokenPrice {
    type ArrangedAccounts = GetCompoundingTokenPriceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, lp_token_mint, ix_sysvar, _remaining @ ..] = accounts else {
            return None;
        };

        Some(GetCompoundingTokenPriceInstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
