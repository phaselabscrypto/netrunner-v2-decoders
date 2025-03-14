

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc0fece4ca8b63bdf")]
pub struct SetCollection{
}

pub struct SetCollectionInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub authority_pda: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub new_collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub new_collection_metadata: solana_sdk::pubkey::Pubkey,
    pub new_collection_mint: solana_sdk::pubkey::Pubkey,
    pub new_collection_master_edition: solana_sdk::pubkey::Pubkey,
    pub new_collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCollection {
    type ArrangedAccounts = SetCollectionInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let authority_pda = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let collection_mint = accounts.get(4)?;
        let collection_metadata = accounts.get(5)?;
        let collection_authority_record = accounts.get(6)?;
        let new_collection_update_authority = accounts.get(7)?;
        let new_collection_metadata = accounts.get(8)?;
        let new_collection_mint = accounts.get(9)?;
        let new_collection_master_edition = accounts.get(10)?;
        let new_collection_authority_record = accounts.get(11)?;
        let token_metadata_program = accounts.get(12)?;
        let system_program = accounts.get(13)?;

        Some(SetCollectionInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            authority_pda: authority_pda.pubkey,
            payer: payer.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
            new_collection_update_authority: new_collection_update_authority.pubkey,
            new_collection_metadata: new_collection_metadata.pubkey,
            new_collection_mint: new_collection_mint.pubkey,
            new_collection_master_edition: new_collection_master_edition.pubkey,
            new_collection_authority_record: new_collection_authority_record.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}