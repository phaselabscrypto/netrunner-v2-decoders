

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x728025a747e328b2")]
pub struct ReallocPool{
}

pub struct ReallocPoolInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReallocPool {
    type ArrangedAccounts = ReallocPoolInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let rent = accounts.get(3)?;

        Some(ReallocPoolInstructionAccounts {
            keeper: keeper.pubkey,
            pool: pool.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}