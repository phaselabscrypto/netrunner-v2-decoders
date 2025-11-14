use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5ecf8426e3918b85")]
pub struct GetClosePositionQuote {
    pub params: GetClosePositionQuoteParams,
}

pub struct GetClosePositionQuoteInstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub target_custody: solana_sdk::pubkey::Pubkey,
    pub target_oracle_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_oracle_account: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetClosePositionQuote {
    type ArrangedAccounts = GetClosePositionQuoteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, market, position, target_custody, target_oracle_account, collateral_custody, collateral_oracle_account, ix_sysvar, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GetClosePositionQuoteInstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            market: market.pubkey,
            position: position.pubkey,
            target_custody: target_custody.pubkey,
            target_oracle_account: target_oracle_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_oracle_account: collateral_oracle_account.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
