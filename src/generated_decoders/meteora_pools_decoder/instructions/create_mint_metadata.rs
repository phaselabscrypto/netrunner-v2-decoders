

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0d46a829fa64945a")]
pub struct CreateMintMetadata{
}

pub struct CreateMintMetadataInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub mint_metadata: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMintMetadata {
    type ArrangedAccounts = CreateMintMetadataInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let lp_mint = accounts.get(1)?;
        let a_vault_lp = accounts.get(2)?;
        let mint_metadata = accounts.get(3)?;
        let metadata_program = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let payer = accounts.get(6)?;

        Some(CreateMintMetadataInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            mint_metadata: mint_metadata.pubkey,
            metadata_program: metadata_program.pubkey,
            system_program: system_program.pubkey,
            payer: payer.pubkey,
        })
    }
}