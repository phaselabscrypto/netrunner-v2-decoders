

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7b45a8c3b7d5c7d6")]
pub struct EmergencyUnstake{
    pub stake_index: u32,
    pub validator_index: u32,
}

pub struct EmergencyUnstakeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub validator_manager_authority: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub stake_list: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_deposit_authority: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EmergencyUnstake {
    type ArrangedAccounts = EmergencyUnstakeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let validator_manager_authority = accounts.get(1)?;
        let validator_list = accounts.get(2)?;
        let stake_list = accounts.get(3)?;
        let stake_account = accounts.get(4)?;
        let stake_deposit_authority = accounts.get(5)?;
        let clock = accounts.get(6)?;
        let stake_program = accounts.get(7)?;

        Some(EmergencyUnstakeInstructionAccounts {
            state: state.pubkey,
            validator_manager_authority: validator_manager_authority.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            stake_account: stake_account.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}