

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2f7f5add0aa0b775")]
pub struct ReclaimStakeAccount{
}

pub struct ReclaimStakeAccountInstructionAccounts {
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub pool_sol_reserves: solana_sdk::pubkey::Pubkey,
    pub stake_account_record_account: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReclaimStakeAccount {
    type ArrangedAccounts = ReclaimStakeAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let stake_account = accounts.get(0)?;
        let pool_account = accounts.get(1)?;
        let pool_sol_reserves = accounts.get(2)?;
        let stake_account_record_account = accounts.get(3)?;
        let clock = accounts.get(4)?;
        let stake_history = accounts.get(5)?;
        let stake_program = accounts.get(6)?;

        Some(ReclaimStakeAccountInstructionAccounts {
            stake_account: stake_account.pubkey,
            pool_account: pool_account.pubkey,
            pool_sol_reserves: pool_sol_reserves.pubkey,
            stake_account_record_account: stake_account_record_account.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}