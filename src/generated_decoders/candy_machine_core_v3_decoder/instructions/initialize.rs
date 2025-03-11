
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
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition: solana_sdk::pubkey::Pubkey,
    pub collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority_pda = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let collection_metadata = accounts.get(4)?;
        let collection_mint = accounts.get(5)?;
        let collection_master_edition = accounts.get(6)?;
        let collection_update_authority = accounts.get(7)?;
        let collection_authority_record = accounts.get(8)?;
        let token_metadata_program = accounts.get(9)?;
        let system_program = accounts.get(10)?;

        Some(InitializeInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority_pda: authority_pda.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_master_edition: collection_master_edition.pubkey,
            collection_update_authority: collection_update_authority.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}