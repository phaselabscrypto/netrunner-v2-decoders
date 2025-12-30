use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2bef284ba49ce78b")]
pub struct WithdrawUnclaimedTokens {
    pub params: WithdrawUnclaimedTokensParams,
}

pub struct WithdrawUnclaimedTokensInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_vault_token_account: solana_sdk::pubkey::Pubkey,
    pub receiving_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub receiving_token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawUnclaimedTokens {
    type ArrangedAccounts = WithdrawUnclaimedTokensInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, perpetuals, transfer_authority, token_vault, token_vault_token_account, receiving_token_account, token_program, receiving_token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawUnclaimedTokensInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            perpetuals: perpetuals.pubkey,
            transfer_authority: transfer_authority.pubkey,
            token_vault: token_vault.pubkey,
            token_vault_token_account: token_vault_token_account.pubkey,
            receiving_token_account: receiving_token_account.pubkey,
            token_program: token_program.pubkey,
            receiving_token_mint: receiving_token_mint.pubkey,
        })
    }
}
