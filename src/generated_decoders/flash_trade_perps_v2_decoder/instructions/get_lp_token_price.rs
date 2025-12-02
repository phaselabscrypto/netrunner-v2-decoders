use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x47ac1519b0a83c0a")]
pub struct GetLpTokenPrice {
    pub params: GetLpTokenPriceParams,
}

pub struct GetLpTokenPriceInstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetLpTokenPrice {
    type ArrangedAccounts = GetLpTokenPriceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, lp_token_mint, ix_sysvar, _remaining @ ..] = accounts else {
            return None;
        };

        Some(GetLpTokenPriceInstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
