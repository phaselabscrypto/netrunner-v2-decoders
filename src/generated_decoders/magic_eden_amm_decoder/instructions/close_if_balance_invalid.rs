

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x452a6c605d6170c8")]
pub struct CloseIfBalanceInvalid{
}

pub struct CloseIfBalanceInvalidInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub buyside_sol_escrow_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseIfBalanceInvalid {
    type ArrangedAccounts = CloseIfBalanceInvalidInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let buyside_sol_escrow_account = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(CloseIfBalanceInvalidInstructionAccounts {
            authority: authority.pubkey,
            owner: owner.pubkey,
            pool: pool.pubkey,
            buyside_sol_escrow_account: buyside_sol_escrow_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}