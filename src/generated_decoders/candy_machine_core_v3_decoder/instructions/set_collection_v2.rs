use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe5233d5b0f0e63a0")]
pub struct SetCollectionV2 {}

pub struct SetCollectionV2InstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub authority_pda: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_delegate_record: solana_sdk::pubkey::Pubkey,
    pub new_collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub new_collection_mint: solana_sdk::pubkey::Pubkey,
    pub new_collection_metadata: solana_sdk::pubkey::Pubkey,
    pub new_collection_master_edition: solana_sdk::pubkey::Pubkey,
    pub new_collection_delegate_record: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCollectionV2 {
    type ArrangedAccounts = SetCollectionV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let authority_pda = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let collection_update_authority = accounts.get(4)?;
        let collection_mint = accounts.get(5)?;
        let collection_metadata = accounts.get(6)?;
        let collection_delegate_record = accounts.get(7)?;
        let new_collection_update_authority = accounts.get(8)?;
        let new_collection_mint = accounts.get(9)?;
        let new_collection_metadata = accounts.get(10)?;
        let new_collection_master_edition = accounts.get(11)?;
        let new_collection_delegate_record = accounts.get(12)?;
        let token_metadata_program = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let sysvar_instructions = accounts.get(15)?;
        let authorization_rules_program = accounts.get(16)?;
        let authorization_rules = accounts.get(17)?;

        Some(SetCollectionV2InstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            authority_pda: authority_pda.pubkey,
            payer: payer.pubkey,
            collection_update_authority: collection_update_authority.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_delegate_record: collection_delegate_record.pubkey,
            new_collection_update_authority: new_collection_update_authority.pubkey,
            new_collection_mint: new_collection_mint.pubkey,
            new_collection_metadata: new_collection_metadata.pubkey,
            new_collection_master_edition: new_collection_master_edition.pubkey,
            new_collection_delegate_record: new_collection_delegate_record.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
        })
    }
}
