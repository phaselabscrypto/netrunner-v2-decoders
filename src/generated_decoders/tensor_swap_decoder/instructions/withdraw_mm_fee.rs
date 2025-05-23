use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3696817e87cd9578")]
pub struct WithdrawMmFee {
    pub config: PoolConfig,
    pub lamports: u64,
}

pub struct WithdrawMmFeeInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub sol_escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawMmFee {
    type ArrangedAccounts = WithdrawMmFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let whitelist = accounts.get(2)?;
        let sol_escrow = accounts.get(3)?;
        let owner = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(WithdrawMmFeeInstructionAccounts {
            tswap: tswap.pubkey,
            pool: pool.pubkey,
            whitelist: whitelist.pubkey,
            sol_escrow: sol_escrow.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
