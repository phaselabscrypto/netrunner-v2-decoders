

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x78791792ad6ec7cd")]
pub struct MintV2{
    pub mint_args: Vec<u8>,
    pub label: Option<String>,
}

pub struct MintV2InstructionAccounts {
    pub candy_guard: solana_sdk::pubkey::Pubkey,
    pub candy_machine_program: solana_sdk::pubkey::Pubkey,
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub candy_machine_authority_pda: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub minter: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_mint_authority: solana_sdk::pubkey::Pubkey,
    pub nft_metadata: solana_sdk::pubkey::Pubkey,
    pub nft_master_edition: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub token_record: solana_sdk::pubkey::Pubkey,
    pub collection_delegate_record: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition: solana_sdk::pubkey::Pubkey,
    pub collection_update_authority: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
    pub spl_ata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub recent_slothashes: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintV2 {
    type ArrangedAccounts = MintV2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let candy_guard = accounts.get(0)?;
        let candy_machine_program = accounts.get(1)?;
        let candy_machine = accounts.get(2)?;
        let candy_machine_authority_pda = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let minter = accounts.get(5)?;
        let nft_mint = accounts.get(6)?;
        let nft_mint_authority = accounts.get(7)?;
        let nft_metadata = accounts.get(8)?;
        let nft_master_edition = accounts.get(9)?;
        let token = accounts.get(10)?;
        let token_record = accounts.get(11)?;
        let collection_delegate_record = accounts.get(12)?;
        let collection_mint = accounts.get(13)?;
        let collection_metadata = accounts.get(14)?;
        let collection_master_edition = accounts.get(15)?;
        let collection_update_authority = accounts.get(16)?;
        let token_metadata_program = accounts.get(17)?;
        let spl_token_program = accounts.get(18)?;
        let spl_ata_program = accounts.get(19)?;
        let system_program = accounts.get(20)?;
        let sysvar_instructions = accounts.get(21)?;
        let recent_slothashes = accounts.get(22)?;
        let authorization_rules_program = accounts.get(23)?;
        let authorization_rules = accounts.get(24)?;

        Some(MintV2InstructionAccounts {
            candy_guard: candy_guard.pubkey,
            candy_machine_program: candy_machine_program.pubkey,
            candy_machine: candy_machine.pubkey,
            candy_machine_authority_pda: candy_machine_authority_pda.pubkey,
            payer: payer.pubkey,
            minter: minter.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_mint_authority: nft_mint_authority.pubkey,
            nft_metadata: nft_metadata.pubkey,
            nft_master_edition: nft_master_edition.pubkey,
            token: token.pubkey,
            token_record: token_record.pubkey,
            collection_delegate_record: collection_delegate_record.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_master_edition: collection_master_edition.pubkey,
            collection_update_authority: collection_update_authority.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            spl_token_program: spl_token_program.pubkey,
            spl_ata_program: spl_ata_program.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            recent_slothashes: recent_slothashes.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
        })
    }
}