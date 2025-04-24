use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbb69d389e03b1de3")]
pub struct AttachPoolToMargin {
    pub config: PoolConfig,
}

pub struct AttachPoolToMarginInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub sol_escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AttachPoolToMargin {
    type ArrangedAccounts = AttachPoolToMarginInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let margin_account = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let whitelist = accounts.get(3)?;
        let sol_escrow = accounts.get(4)?;
        let owner = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(AttachPoolToMarginInstructionAccounts {
            tswap: tswap.pubkey,
            margin_account: margin_account.pubkey,
            pool: pool.pubkey,
            whitelist: whitelist.pubkey,
            sol_escrow: sol_escrow.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
