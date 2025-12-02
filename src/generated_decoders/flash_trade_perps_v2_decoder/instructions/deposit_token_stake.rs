use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x694d1d421c23b70a")]
pub struct DepositTokenStake {
    pub params: DepositTokenStakeParams,
}

pub struct DepositTokenStakeInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub funding_token_account: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_vault_token_account: solana_sdk::pubkey::Pubkey,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositTokenStake {
    type ArrangedAccounts = DepositTokenStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, fee_payer, funding_token_account, perpetuals, token_vault, token_vault_token_account, token_stake_account, system_program, token_program, event_authority, program, token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositTokenStakeInstructionAccounts {
            owner: owner.pubkey,
            fee_payer: fee_payer.pubkey,
            funding_token_account: funding_token_account.pubkey,
            perpetuals: perpetuals.pubkey,
            token_vault: token_vault.pubkey,
            token_vault_token_account: token_vault_token_account.pubkey,
            token_stake_account: token_stake_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            token_mint: token_mint.pubkey,
        })
    }
}
