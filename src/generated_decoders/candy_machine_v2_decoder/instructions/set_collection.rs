use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc0fece4ca8b63bdf")]
pub struct SetCollection {}

pub struct SetCollectionInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub collection_pda: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCollection {
    type ArrangedAccounts = SetCollectionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let collection_pda = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let rent = accounts.get(5)?;
        let metadata = accounts.get(6)?;
        let mint = accounts.get(7)?;
        let edition = accounts.get(8)?;
        let collection_authority_record = accounts.get(9)?;
        let token_metadata_program = accounts.get(10)?;

        Some(SetCollectionInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            collection_pda: collection_pda.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            edition: edition.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
        })
    }
}
