use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe4ea0ebecef973a7")]
pub struct AddInternalOracle {
    pub params: AddInternalOracleParams,
}

pub struct AddInternalOracleInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub custody_token_mint: solana_sdk::pubkey::Pubkey,
    pub int_oracle_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddInternalOracle {
    type ArrangedAccounts = AddInternalOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, custody_token_mint, int_oracle_account, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddInternalOracleInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            custody_token_mint: custody_token_mint.pubkey,
            int_oracle_account: int_oracle_account.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
