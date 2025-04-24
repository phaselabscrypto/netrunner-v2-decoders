use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6711c819765f7d3d")]
pub struct SetCollectionDuringMint {}

pub struct SetCollectionDuringMintInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub collection_pda: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub instructions: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCollectionDuringMint {
    type ArrangedAccounts = SetCollectionDuringMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let metadata = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let collection_pda = accounts.get(3)?;
        let token_metadata_program = accounts.get(4)?;
        let instructions = accounts.get(5)?;
        let collection_mint = accounts.get(6)?;
        let collection_metadata = accounts.get(7)?;
        let collection_master_edition = accounts.get(8)?;
        let authority = accounts.get(9)?;
        let collection_authority_record = accounts.get(10)?;

        Some(SetCollectionDuringMintInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            metadata: metadata.pubkey,
            payer: payer.pubkey,
            collection_pda: collection_pda.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            instructions: instructions.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_master_edition: collection_master_edition.pubkey,
            authority: authority.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
        })
    }
}
