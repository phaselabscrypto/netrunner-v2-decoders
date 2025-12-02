use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8fe58330f8d4a7b9")]
pub struct RemoveCustody {
    pub params: RemoveCustodyParams,
}

pub struct RemoveCustodyInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub receiving_account: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub custody_token_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub receiving_token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveCustody {
    type ArrangedAccounts = RemoveCustodyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, receiving_account, multisig, transfer_authority, perpetuals, pool, custody, custody_oracle_account, custody_token_account, system_program, token_program, ix_sysvar, receiving_token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RemoveCustodyInstructionAccounts {
            admin: admin.pubkey,
            receiving_account: receiving_account.pubkey,
            multisig: multisig.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_oracle_account: custody_oracle_account.pubkey,
            custody_token_account: custody_token_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            receiving_token_mint: receiving_token_mint.pubkey,
        })
    }
}
