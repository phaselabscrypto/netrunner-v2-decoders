use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x62d17a1bde895e86")]
pub struct CreateAndDelegateStakeAccount {
    pub params: CreateAndDelegateStakeAccountParams,
}

pub struct CreateAndDelegateStakeAccountInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_token_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_info: solana_sdk::pubkey::Pubkey,
    pub validator_vote_account: solana_sdk::pubkey::Pubkey,
    pub stake_config: solana_sdk::pubkey::Pubkey,
    pub wsol_mint: solana_sdk::pubkey::Pubkey,
    pub temp_wsol_account: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateAndDelegateStakeAccount {
    type ArrangedAccounts = CreateAndDelegateStakeAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, perpetuals, pool, custody, custody_token_account, transfer_authority, stake_account, stake_info, validator_vote_account, stake_config, wsol_mint, temp_wsol_account, rent, clock, stake_history, stake_program, system_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateAndDelegateStakeAccountInstructionAccounts {
            keeper: keeper.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_token_account: custody_token_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            stake_account: stake_account.pubkey,
            stake_info: stake_info.pubkey,
            validator_vote_account: validator_vote_account.pubkey,
            stake_config: stake_config.pubkey,
            wsol_mint: wsol_mint.pubkey,
            temp_wsol_account: temp_wsol_account.pubkey,
            rent: rent.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            stake_program: stake_program.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
