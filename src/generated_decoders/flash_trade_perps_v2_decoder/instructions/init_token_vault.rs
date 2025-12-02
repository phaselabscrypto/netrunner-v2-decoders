use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcb1ac2a9fce2b3b4")]
pub struct InitTokenVault {
    pub params: InitTokenVaultParams,
}

pub struct InitTokenVaultInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub funding_token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_vault_token_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitTokenVault {
    type ArrangedAccounts = InitTokenVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, perpetuals, transfer_authority, funding_token_account, token_mint, token_vault, token_vault_token_account, system_program, token_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitTokenVaultInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            perpetuals: perpetuals.pubkey,
            transfer_authority: transfer_authority.pubkey,
            funding_token_account: funding_token_account.pubkey,
            token_mint: token_mint.pubkey,
            token_vault: token_vault.pubkey,
            token_vault_token_account: token_vault_token_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
