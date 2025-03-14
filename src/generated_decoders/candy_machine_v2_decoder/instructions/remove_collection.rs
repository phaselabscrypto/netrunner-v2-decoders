

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdf346ad93ddc24a0")]
pub struct RemoveCollection{
}

pub struct RemoveCollectionInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub collection_pda: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveCollection {
    type ArrangedAccounts = RemoveCollectionInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let collection_pda = accounts.get(2)?;
        let metadata = accounts.get(3)?;
        let mint = accounts.get(4)?;
        let collection_authority_record = accounts.get(5)?;
        let token_metadata_program = accounts.get(6)?;

        Some(RemoveCollectionInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            collection_pda: collection_pda.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
        })
    }
}