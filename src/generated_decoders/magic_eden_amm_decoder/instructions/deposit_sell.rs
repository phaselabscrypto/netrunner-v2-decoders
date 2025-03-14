
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x90832c9cc50ac52b")]
pub struct DepositSell{
    pub args: DepositSellArgs,
}

pub struct DepositSellInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub asset_metadata: solana_sdk::pubkey::Pubkey,
    pub asset_master_edition: solana_sdk::pubkey::Pubkey,
    pub asset_mint: solana_sdk::pubkey::Pubkey,
    pub asset_token_account: solana_sdk::pubkey::Pubkey,
    pub sellside_escrow_token_account: solana_sdk::pubkey::Pubkey,
    pub sell_state: solana_sdk::pubkey::Pubkey,
    pub allowlist_aux_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositSell {
    type ArrangedAccounts = DepositSellInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let cosigner = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let asset_metadata = accounts.get(3)?;
        let asset_master_edition = accounts.get(4)?;
        let asset_mint = accounts.get(5)?;
        let asset_token_account = accounts.get(6)?;
        let sellside_escrow_token_account = accounts.get(7)?;
        let sell_state = accounts.get(8)?;
        let allowlist_aux_account = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let associated_token_program = accounts.get(12)?;
        let rent = accounts.get(13)?;

        Some(DepositSellInstructionAccounts {
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            pool: pool.pubkey,
            asset_metadata: asset_metadata.pubkey,
            asset_master_edition: asset_master_edition.pubkey,
            asset_mint: asset_mint.pubkey,
            asset_token_account: asset_token_account.pubkey,
            sellside_escrow_token_account: sellside_escrow_token_account.pubkey,
            sell_state: sell_state.pubkey,
            allowlist_aux_account: allowlist_aux_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}