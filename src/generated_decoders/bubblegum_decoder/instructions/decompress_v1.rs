
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x36554c46e4faa451")]
pub struct DecompressV1{
    pub metadata: MetadataArgs,
}

pub struct DecompressV1InstructionAccounts {
    pub voucher: solana_sdk::pubkey::Pubkey,
    pub leaf_owner: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_rent: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecompressV1 {
    type ArrangedAccounts = DecompressV1InstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let voucher = accounts.get(0)?;
        let leaf_owner = accounts.get(1)?;
        let token_account = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let mint_authority = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let master_edition = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let sysvar_rent = accounts.get(8)?;
        let token_metadata_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let associated_token_program = accounts.get(11)?;
        let log_wrapper = accounts.get(12)?;

        Some(DecompressV1InstructionAccounts {
            voucher: voucher.pubkey,
            leaf_owner: leaf_owner.pubkey,
            token_account: token_account.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
            metadata: metadata.pubkey,
            master_edition: master_edition.pubkey,
            system_program: system_program.pubkey,
            sysvar_rent: sysvar_rent.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}