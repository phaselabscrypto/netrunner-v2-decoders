
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb73b486edff3968e")]
pub struct GetRemoveLiquidityAmountAndFee2{
    pub params: GetRemoveLiquidityAmountAndFee2Params,
}

pub struct GetRemoveLiquidityAmountAndFee2InstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetRemoveLiquidityAmountAndFee2 {
    type ArrangedAccounts = GetRemoveLiquidityAmountAndFee2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let perpetuals = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let custody = accounts.get(2)?;
        let custody_doves_price_account = accounts.get(3)?;
        let custody_pythnet_price_account = accounts.get(4)?;
        let lp_token_mint = accounts.get(5)?;

        Some(GetRemoveLiquidityAmountAndFee2InstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
        })
    }
}