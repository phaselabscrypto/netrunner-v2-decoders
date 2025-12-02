use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeb7edb8f1d4a95a1")]
pub struct InitRevenueTokenAccount {
    pub params: InitRevenueTokenAccountParams,
}

pub struct InitRevenueTokenAccountInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub revenue_token_account: solana_sdk::pubkey::Pubkey,
    pub protocol_vault: solana_sdk::pubkey::Pubkey,
    pub protocol_token_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitRevenueTokenAccount {
    type ArrangedAccounts = InitRevenueTokenAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, transfer_authority, perpetuals, token_vault, reward_mint, revenue_token_account, protocol_vault, protocol_token_account, system_program, token_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitRevenueTokenAccountInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            token_vault: token_vault.pubkey,
            reward_mint: reward_mint.pubkey,
            revenue_token_account: revenue_token_account.pubkey,
            protocol_vault: protocol_vault.pubkey,
            protocol_token_account: protocol_token_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
