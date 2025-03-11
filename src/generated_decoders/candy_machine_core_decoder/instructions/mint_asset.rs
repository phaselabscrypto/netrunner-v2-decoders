
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, PartialEq, Eq, Clone)]
#[carbon(discriminator = "0x54afd39c38fa6876")]
pub struct MintAsset{
    pub args: MintAssetArgs,
}

pub struct MintAssetInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority_pda: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub asset_owner: solana_sdk::pubkey::Pubkey,
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub mpl_core_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub recent_slothashes: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintAsset {
    type ArrangedAccounts = MintAssetInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority_pda = accounts.get(1)?;
        let mint_authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let asset_owner = accounts.get(4)?;
        let asset = accounts.get(5)?;
        let collection = accounts.get(6)?;
        let mpl_core_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let sysvar_instructions = accounts.get(9)?;
        let recent_slothashes = accounts.get(10)?;

        Some(MintAssetInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority_pda: authority_pda.pubkey,
            mint_authority: mint_authority.pubkey,
            payer: payer.pubkey,
            asset_owner: asset_owner.pubkey,
            asset: asset.pubkey,
            collection: collection.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            recent_slothashes: recent_slothashes.pubkey,
        })
    }
}