

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3339e12fb69289a6")]
pub struct Mint{
    pub mint_args: Vec<u8>,
    pub label: Option<String>,
}

pub struct MintInstructionAccounts {
    pub candy_guard: solana_sdk::pubkey::Pubkey,
    pub candy_machine_program: solana_sdk::pubkey::Pubkey,
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub candy_machine_authority_pda: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub nft_metadata: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_mint_authority: solana_sdk::pubkey::Pubkey,
    pub nft_master_edition: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition: solana_sdk::pubkey::Pubkey,
    pub collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub recent_slothashes: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Mint {
    type ArrangedAccounts = MintInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let candy_guard = accounts.get(0)?;
        let candy_machine_program = accounts.get(1)?;
        let candy_machine = accounts.get(2)?;
        let candy_machine_authority_pda = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let nft_metadata = accounts.get(5)?;
        let nft_mint = accounts.get(6)?;
        let nft_mint_authority = accounts.get(7)?;
        let nft_master_edition = accounts.get(8)?;
        let collection_authority_record = accounts.get(9)?;
        let collection_mint = accounts.get(10)?;
        let collection_metadata = accounts.get(11)?;
        let collection_master_edition = accounts.get(12)?;
        let collection_update_authority = accounts.get(13)?;
        let token_metadata_program = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let system_program = accounts.get(16)?;
        let recent_slothashes = accounts.get(17)?;
        let instruction_sysvar_account = accounts.get(18)?;

        Some(MintInstructionAccounts {
            candy_guard: candy_guard.pubkey,
            candy_machine_program: candy_machine_program.pubkey,
            candy_machine: candy_machine.pubkey,
            candy_machine_authority_pda: candy_machine_authority_pda.pubkey,
            payer: payer.pubkey,
            nft_metadata: nft_metadata.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_mint_authority: nft_mint_authority.pubkey,
            nft_master_edition: nft_master_edition.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_master_edition: collection_master_edition.pubkey,
            collection_update_authority: collection_update_authority.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            recent_slothashes: recent_slothashes.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}