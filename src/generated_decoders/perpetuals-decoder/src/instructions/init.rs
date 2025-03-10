
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdc3bcfec6cfa2f64")]
pub struct Init{
    pub params: InitParams,
}

pub struct InitInstructionAccounts {
    pub upgrade_authority: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub perpetuals_program: solana_sdk::pubkey::Pubkey,
    pub perpetuals_program_data: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Init {
    type ArrangedAccounts = InitInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let upgrade_authority = accounts.get(0)?;
        let admin = accounts.get(1)?;
        let transfer_authority = accounts.get(2)?;
        let perpetuals = accounts.get(3)?;
        let perpetuals_program = accounts.get(4)?;
        let perpetuals_program_data = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let token_program = accounts.get(7)?;

        Some(InitInstructionAccounts {
            upgrade_authority: upgrade_authority.pubkey,
            admin: admin.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            perpetuals_program: perpetuals_program.pubkey,
            perpetuals_program_data: perpetuals_program_data.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}