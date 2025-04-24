use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x32ae222403a61dcc")]
pub struct EditPool {
    pub old_config: PoolConfig,
    pub new_config: PoolConfig,
    pub is_cosigned: Option<bool>,
    pub max_taker_sell_count: Option<u32>,
}

pub struct EditPoolInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub old_pool: solana_sdk::pubkey::Pubkey,
    pub new_pool: solana_sdk::pubkey::Pubkey,
    pub old_sol_escrow: solana_sdk::pubkey::Pubkey,
    pub new_sol_escrow: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub nft_authority: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditPool {
    type ArrangedAccounts = EditPoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let old_pool = accounts.get(1)?;
        let new_pool = accounts.get(2)?;
        let old_sol_escrow = accounts.get(3)?;
        let new_sol_escrow = accounts.get(4)?;
        let whitelist = accounts.get(5)?;
        let nft_authority = accounts.get(6)?;
        let owner = accounts.get(7)?;
        let system_program = accounts.get(8)?;

        Some(EditPoolInstructionAccounts {
            tswap: tswap.pubkey,
            old_pool: old_pool.pubkey,
            new_pool: new_pool.pubkey,
            old_sol_escrow: old_sol_escrow.pubkey,
            new_sol_escrow: new_sol_escrow.pubkey,
            whitelist: whitelist.pubkey,
            nft_authority: nft_authority.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
