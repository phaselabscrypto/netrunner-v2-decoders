
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x37f3fdf04ebae8a6")]
pub struct CreateMerkleRootConfig{
    pub params: CreateMerkleRootConfigParams,
}

pub struct CreateMerkleRootConfigInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub merkle_root_config: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMerkleRootConfig {
    type ArrangedAccounts = CreateMerkleRootConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let merkle_root_config = accounts.get(1)?;
        let admin = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let event_authority = accounts.get(4)?;
        let program = accounts.get(5)?;

        Some(CreateMerkleRootConfigInstructionAccounts {
            vault: vault.pubkey,
            merkle_root_config: merkle_root_config.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}