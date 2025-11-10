use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf779286323526420")]
pub struct GetSwapAmountAndFees {
    pub params: GetSwapAmountAndFeesParams,
}

pub struct GetSwapAmountAndFeesInstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub receiving_custody: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub dispensing_custody: solana_sdk::pubkey::Pubkey,
    pub dispensing_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetSwapAmountAndFees {
    type ArrangedAccounts = GetSwapAmountAndFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, receiving_custody, receiving_custody_oracle_account, dispensing_custody, dispensing_custody_oracle_account, ix_sysvar, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GetSwapAmountAndFeesInstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            receiving_custody: receiving_custody.pubkey,
            receiving_custody_oracle_account: receiving_custody_oracle_account.pubkey,
            dispensing_custody: dispensing_custody.pubkey,
            dispensing_custody_oracle_account: dispensing_custody_oracle_account.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
