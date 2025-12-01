use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x455acc6f9c8c8ab8")]
pub struct InitCompounding {
    pub params: InitCompoundingParams,
}

pub struct InitCompoundingInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub compounding_vault: solana_sdk::pubkey::Pubkey,
    pub compounding_token_mint: solana_sdk::pubkey::Pubkey,
    pub metadata_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitCompounding {
    type ArrangedAccounts = InitCompoundingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, transfer_authority, perpetuals, pool, lp_token_mint, compounding_vault, compounding_token_mint, metadata_account, system_program, token_program, metadata_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitCompoundingInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            compounding_vault: compounding_vault.pubkey,
            compounding_token_mint: compounding_token_mint.pubkey,
            metadata_account: metadata_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            metadata_program: metadata_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
