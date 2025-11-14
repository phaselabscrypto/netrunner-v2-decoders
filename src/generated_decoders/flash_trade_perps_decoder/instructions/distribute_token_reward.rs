use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x96bb35cabceefc20")]
pub struct DistributeTokenReward {
    pub params: DistributeTokenRewardParams,
}

pub struct DistributeTokenRewardInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub funding_token_account: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_vault_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DistributeTokenReward {
    type ArrangedAccounts = DistributeTokenRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, perpetuals, transfer_authority, funding_token_account, token_vault, token_vault_token_account, token_program, event_authority, program, token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DistributeTokenRewardInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            perpetuals: perpetuals.pubkey,
            transfer_authority: transfer_authority.pubkey,
            funding_token_account: funding_token_account.pubkey,
            token_vault: token_vault.pubkey,
            token_vault_token_account: token_vault_token_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            token_mint: token_mint.pubkey,
        })
    }
}
