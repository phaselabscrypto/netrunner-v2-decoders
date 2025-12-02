use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaea00fa8e89f7acc")]
pub struct SetInternalOraclePrice {
    pub params: SetInternalOraclePriceParams,
}

pub struct SetInternalOraclePriceInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub int_oracle_account: solana_sdk::pubkey::Pubkey,
    pub ext_oracle_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetInternalOraclePrice {
    type ArrangedAccounts = SetInternalOraclePriceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, perpetuals, pool, custody, int_oracle_account, ext_oracle_account, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetInternalOraclePriceInstructionAccounts {
            authority: authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            int_oracle_account: int_oracle_account.pubkey,
            ext_oracle_account: ext_oracle_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
