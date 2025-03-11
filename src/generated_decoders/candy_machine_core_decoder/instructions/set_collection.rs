

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
    pub collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub new_collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub new_collection: solana_sdk::pubkey::Pubkey,
    pub mpl_core_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCollection {
    type ArrangedAccounts = SetCollectionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let authority_pda = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let collection_update_authority = accounts.get(4)?;
        let collection = accounts.get(5)?;
        let new_collection_update_authority = accounts.get(6)?;
        let new_collection = accounts.get(7)?;
        let mpl_core_program = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let sysvar_instructions = accounts.get(10)?;

        Some(SetCollectionInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            authority_pda: authority_pda.pubkey,
            payer: payer.pubkey,
            collection_update_authority: collection_update_authority.pubkey,
            collection: collection.pubkey,
            new_collection_update_authority: new_collection_update_authority.pubkey,
            new_collection: new_collection.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
        })
    }
}