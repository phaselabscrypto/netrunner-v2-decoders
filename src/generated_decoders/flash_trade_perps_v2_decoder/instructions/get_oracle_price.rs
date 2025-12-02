use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc814006a38d2e68c")]
pub struct GetOraclePrice {
    pub params: GetOraclePriceParams,
}

pub struct GetOraclePriceInstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetOraclePrice {
    type ArrangedAccounts = GetOraclePriceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, custody, custody_oracle_account, ix_sysvar, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GetOraclePriceInstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_oracle_account: custody_oracle_account.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
