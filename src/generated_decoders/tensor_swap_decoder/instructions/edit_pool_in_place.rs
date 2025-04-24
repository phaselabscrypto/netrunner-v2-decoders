use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7dbf7771060ea417")]
pub struct EditPoolInPlace {
    pub config: PoolConfig,
    pub is_cosigned: Option<bool>,
    pub max_taker_sell_count: Option<u32>,
    pub mm_compound_fees: Option<bool>,
}

pub struct EditPoolInPlaceInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditPoolInPlace {
    type ArrangedAccounts = EditPoolInPlaceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let whitelist = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(EditPoolInPlaceInstructionAccounts {
            tswap: tswap.pubkey,
            pool: pool.pubkey,
            whitelist: whitelist.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
