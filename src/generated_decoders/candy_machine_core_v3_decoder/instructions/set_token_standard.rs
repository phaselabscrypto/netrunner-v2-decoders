

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x93d46ac31eaad180")]
pub struct SetTokenStandard{
    pub token_standard: u8,
}

pub struct SetTokenStandardInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub authority_pda: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub rule_set: solana_sdk::pubkey::Pubkey,
    pub collection_delegate_record: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenStandard {
    type ArrangedAccounts = SetTokenStandardInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let authority_pda = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let rule_set = accounts.get(4)?;
        let collection_delegate_record = accounts.get(5)?;
        let collection_mint = accounts.get(6)?;
        let collection_metadata = accounts.get(7)?;
        let collection_authority_record = accounts.get(8)?;
        let collection_update_authority = accounts.get(9)?;
        let token_metadata_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let sysvar_instructions = accounts.get(12)?;
        let authorization_rules_program = accounts.get(13)?;
        let authorization_rules = accounts.get(14)?;

        Some(SetTokenStandardInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            authority_pda: authority_pda.pubkey,
            payer: payer.pubkey,
            rule_set: rule_set.pubkey,
            collection_delegate_record: collection_delegate_record.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
            collection_update_authority: collection_update_authority.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
        })
    }
}