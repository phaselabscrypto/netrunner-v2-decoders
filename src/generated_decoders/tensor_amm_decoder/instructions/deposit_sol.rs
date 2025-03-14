

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6c514e757d9b38c8")]
pub struct DepositSol{
    pub lamports: u64,
}

pub struct DepositSolInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositSol {
    type ArrangedAccounts = DepositSolInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(DepositSolInstructionAccounts {
            owner: owner.pubkey,
            pool: pool.pubkey,
            system_program: system_program.pubkey,
        })
    }
}