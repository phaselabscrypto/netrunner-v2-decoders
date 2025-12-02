use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x730984fb034e2828")]
pub struct CollectTokenReward {
    pub params: CollectTokenRewardParams,
}

pub struct CollectTokenRewardInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_token_account: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_vault_token_account: solana_sdk::pubkey::Pubkey,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectTokenReward {
    type ArrangedAccounts = CollectTokenRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, receiving_token_account, perpetuals, transfer_authority, token_vault, token_vault_token_account, token_stake_account, token_program, event_authority, program, token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectTokenRewardInstructionAccounts {
            owner: owner.pubkey,
            receiving_token_account: receiving_token_account.pubkey,
            perpetuals: perpetuals.pubkey,
            transfer_authority: transfer_authority.pubkey,
            token_vault: token_vault.pubkey,
            token_vault_token_account: token_vault_token_account.pubkey,
            token_stake_account: token_stake_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            token_mint: token_mint.pubkey,
        })
    }
}
