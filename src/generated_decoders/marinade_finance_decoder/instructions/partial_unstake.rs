

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x37f1cddd2d72cda3")]
pub struct PartialUnstake{
    pub stake_index: u32,
    pub validator_index: u32,
    pub desired_unstake_amount: u64,
}

pub struct PartialUnstakeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub validator_manager_authority: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub stake_list: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_deposit_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_pda: solana_sdk::pubkey::Pubkey,
    pub split_stake_account: solana_sdk::pubkey::Pubkey,
    pub split_stake_rent_payer: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PartialUnstake {
    type ArrangedAccounts = PartialUnstakeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let validator_manager_authority = accounts.get(1)?;
        let validator_list = accounts.get(2)?;
        let stake_list = accounts.get(3)?;
        let stake_account = accounts.get(4)?;
        let stake_deposit_authority = accounts.get(5)?;
        let reserve_pda = accounts.get(6)?;
        let split_stake_account = accounts.get(7)?;
        let split_stake_rent_payer = accounts.get(8)?;
        let clock = accounts.get(9)?;
        let rent = accounts.get(10)?;
        let stake_history = accounts.get(11)?;
        let system_program = accounts.get(12)?;
        let stake_program = accounts.get(13)?;

        Some(PartialUnstakeInstructionAccounts {
            state: state.pubkey,
            validator_manager_authority: validator_manager_authority.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            stake_account: stake_account.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            reserve_pda: reserve_pda.pubkey,
            split_stake_account: split_stake_account.pubkey,
            split_stake_rent_payer: split_stake_rent_payer.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            stake_history: stake_history.pubkey,
            system_program: system_program.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}