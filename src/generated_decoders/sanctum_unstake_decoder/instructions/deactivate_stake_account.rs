

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd9404c10d84d7be2")]
pub struct DeactivateStakeAccount{
}

pub struct DeactivateStakeAccountInstructionAccounts {
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub pool_sol_reserves: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeactivateStakeAccount {
    type ArrangedAccounts = DeactivateStakeAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let stake_account = accounts.get(0)?;
        let pool_account = accounts.get(1)?;
        let pool_sol_reserves = accounts.get(2)?;
        let clock = accounts.get(3)?;
        let stake_program = accounts.get(4)?;

        Some(DeactivateStakeAccountInstructionAccounts {
            stake_account: stake_account.pubkey,
            pool_account: pool_account.pubkey,
            pool_sol_reserves: pool_sol_reserves.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}