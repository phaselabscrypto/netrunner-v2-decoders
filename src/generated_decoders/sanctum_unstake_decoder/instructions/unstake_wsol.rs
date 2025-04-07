

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7d5dbe8759ae8e95")]
pub struct UnstakeWsol{
}

pub struct UnstakeWsolInstructionAccounts {
    pub unstaker: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub pool_sol_reserves: solana_sdk::pubkey::Pubkey,
    pub fee_account: solana_sdk::pubkey::Pubkey,
    pub stake_account_record_account: solana_sdk::pubkey::Pubkey,
    pub protocol_fee_account: solana_sdk::pubkey::Pubkey,
    pub protocol_fee_destination: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnstakeWsol {
    type ArrangedAccounts = UnstakeWsolInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let unstaker = accounts.get(0)?;
        let stake_account = accounts.get(1)?;
        let destination = accounts.get(2)?;
        let pool_account = accounts.get(3)?;
        let pool_sol_reserves = accounts.get(4)?;
        let fee_account = accounts.get(5)?;
        let stake_account_record_account = accounts.get(6)?;
        let protocol_fee_account = accounts.get(7)?;
        let protocol_fee_destination = accounts.get(8)?;
        let clock = accounts.get(9)?;
        let stake_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let token_program = accounts.get(12)?;

        Some(UnstakeWsolInstructionAccounts {
            unstaker: unstaker.pubkey,
            stake_account: stake_account.pubkey,
            destination: destination.pubkey,
            pool_account: pool_account.pubkey,
            pool_sol_reserves: pool_sol_reserves.pubkey,
            fee_account: fee_account.pubkey,
            stake_account_record_account: stake_account_record_account.pubkey,
            protocol_fee_account: protocol_fee_account.pubkey,
            protocol_fee_destination: protocol_fee_destination.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}