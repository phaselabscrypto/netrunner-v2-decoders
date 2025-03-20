

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x9b227aa5f589936b")]
pub struct UpdateSharesMetadata{
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub struct UpdateSharesMetadataInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub shares_mint: solana_sdk::pubkey::Pubkey,
    pub shares_metadata: solana_sdk::pubkey::Pubkey,
    pub shares_mint_authority: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSharesMetadata {
    type ArrangedAccounts = UpdateSharesMetadataInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let global_config = accounts.get(2)?;
        let shares_mint = accounts.get(3)?;
        let shares_metadata = accounts.get(4)?;
        let shares_mint_authority = accounts.get(5)?;
        let metadata_program = accounts.get(6)?;

        Some(UpdateSharesMetadataInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            shares_mint: shares_mint.pubkey,
            shares_metadata: shares_metadata.pubkey,
            shares_mint_authority: shares_mint_authority.pubkey,
            metadata_program: metadata_program.pubkey,
        })
    }
}