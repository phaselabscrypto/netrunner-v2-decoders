use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8cbdd117ef3eef0b")]
pub struct ClosePool {
    pub config: PoolConfig,
}

pub struct ClosePoolInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub sol_escrow: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub nft_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePool {
    type ArrangedAccounts = ClosePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let sol_escrow = accounts.get(2)?;
        let whitelist = accounts.get(3)?;
        let owner = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let nft_authority = accounts.get(6)?;

        Some(ClosePoolInstructionAccounts {
            tswap: tswap.pubkey,
            pool: pool.pubkey,
            sol_escrow: sol_escrow.pubkey,
            whitelist: whitelist.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            nft_authority: nft_authority.pubkey,
        })
    }
}
