

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6cd4e93584533fdb")]
pub struct CloseExpiredPool{
}

pub struct CloseExpiredPoolInstructionAccounts {
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseExpiredPool {
    type ArrangedAccounts = CloseExpiredPoolInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let rent_payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(CloseExpiredPoolInstructionAccounts {
            rent_payer: rent_payer.pubkey,
            owner: owner.pubkey,
            pool: pool.pubkey,
            system_program: system_program.pubkey,
        })
    }
}