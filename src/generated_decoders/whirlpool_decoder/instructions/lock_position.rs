use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe33e02fcf70aabb9")]
pub struct LockPosition {
    pub lock_type: LockType,
}

pub struct LockPositionInstructionAccounts {
    pub funder: solana_sdk::pubkey::Pubkey,
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub lock_config: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token2022_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LockPosition {
    type ArrangedAccounts = LockPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let funder = accounts.get(0)?;
        let position_authority = accounts.get(1)?;
        let position = accounts.get(2)?;
        let position_mint = accounts.get(3)?;
        let position_token_account = accounts.get(4)?;
        let lock_config = accounts.get(5)?;
        let whirlpool = accounts.get(6)?;
        let token2022_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;

        Some(LockPositionInstructionAccounts {
            funder: funder.pubkey,
            position_authority: position_authority.pubkey,
            position: position.pubkey,
            position_mint: position_mint.pubkey,
            position_token_account: position_token_account.pubkey,
            lock_config: lock_config.pubkey,
            whirlpool: whirlpool.pubkey,
            token2022_program: token2022_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
