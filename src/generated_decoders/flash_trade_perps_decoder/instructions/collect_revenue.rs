use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5760d324f02bf657")]
pub struct CollectRevenue {
    pub params: CollectRevenueParams,
}

pub struct CollectRevenueInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_revenue_account: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub revenue_token_account: solana_sdk::pubkey::Pubkey,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub receiving_token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectRevenue {
    type ArrangedAccounts = CollectRevenueInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, receiving_revenue_account, perpetuals, transfer_authority, token_vault, revenue_token_account, token_stake_account, token_program, event_authority, program, receiving_token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectRevenueInstructionAccounts {
            owner: owner.pubkey,
            receiving_revenue_account: receiving_revenue_account.pubkey,
            perpetuals: perpetuals.pubkey,
            transfer_authority: transfer_authority.pubkey,
            token_vault: token_vault.pubkey,
            revenue_token_account: revenue_token_account.pubkey,
            token_stake_account: token_stake_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            receiving_token_mint: receiving_token_mint.pubkey,
        })
    }
}
