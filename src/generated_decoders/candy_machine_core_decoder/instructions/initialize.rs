
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize{
    pub data: CandyMachineData,
}

pub struct InitializeInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority_pda: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub mpl_core_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority_pda = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let collection = accounts.get(4)?;
        let collection_update_authority = accounts.get(5)?;
        let mpl_core_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let sysvar_instructions = accounts.get(8)?;

        Some(InitializeInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority_pda: authority_pda.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            collection: collection.pubkey,
            collection_update_authority: collection_update_authority.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
        })
    }
}