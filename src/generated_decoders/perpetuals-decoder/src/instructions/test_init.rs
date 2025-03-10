
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x30335c7a51137029")]
pub struct TestInit{
    pub params: TestInitParams,
}

pub struct TestInitInstructionAccounts {
    pub upgrade_authority: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TestInit {
    type ArrangedAccounts = TestInitInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let upgrade_authority = accounts.get(0)?;
        let admin = accounts.get(1)?;
        let transfer_authority = accounts.get(2)?;
        let perpetuals = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let token_program = accounts.get(5)?;

        Some(TestInitInstructionAccounts {
            upgrade_authority: upgrade_authority.pubkey,
            admin: admin.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}