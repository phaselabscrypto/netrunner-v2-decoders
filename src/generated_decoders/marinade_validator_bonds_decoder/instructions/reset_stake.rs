

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb725459fa38bd4eb")]
pub struct ResetStake{
}

pub struct ResetStakeInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub bonds_withdrawer_authority: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub stake_config: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ResetStake {
    type ArrangedAccounts = ResetStakeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let settlement = accounts.get(2)?;
        let stake_account = accounts.get(3)?;
        let bonds_withdrawer_authority = accounts.get(4)?;
        let vote_account = accounts.get(5)?;
        let stake_history = accounts.get(6)?;
        let stake_config = accounts.get(7)?;
        let clock = accounts.get(8)?;
        let stake_program = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;

        Some(ResetStakeInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            settlement: settlement.pubkey,
            stake_account: stake_account.pubkey,
            bonds_withdrawer_authority: bonds_withdrawer_authority.pubkey,
            vote_account: vote_account.pubkey,
            stake_history: stake_history.pubkey,
            stake_config: stake_config.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}