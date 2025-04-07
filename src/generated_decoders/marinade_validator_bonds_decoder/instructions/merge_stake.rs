
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0e039217a369f663")]
pub struct MergeStake{
    pub merge_args: MergeStakeArgs,
}

pub struct MergeStakeInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub source_stake: solana_sdk::pubkey::Pubkey,
    pub destination_stake: solana_sdk::pubkey::Pubkey,
    pub staker_authority: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MergeStake {
    type ArrangedAccounts = MergeStakeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let source_stake = accounts.get(1)?;
        let destination_stake = accounts.get(2)?;
        let staker_authority = accounts.get(3)?;
        let stake_history = accounts.get(4)?;
        let clock = accounts.get(5)?;
        let stake_program = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(MergeStakeInstructionAccounts {
            config: config.pubkey,
            source_stake: source_stake.pubkey,
            destination_stake: destination_stake.pubkey,
            staker_authority: staker_authority.pubkey,
            stake_history: stake_history.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}