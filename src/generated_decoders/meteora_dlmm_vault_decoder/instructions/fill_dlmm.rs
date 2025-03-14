

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x016c8d0b047efbde")]
pub struct FillDlmm{
    pub max_amount: u64,
}

pub struct FillDlmmInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_out_vault: solana_sdk::pubkey::Pubkey,
    pub amm_program: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub token_x_mint: solana_sdk::pubkey::Pubkey,
    pub token_y_mint: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub token_x_program: solana_sdk::pubkey::Pubkey,
    pub token_y_program: solana_sdk::pubkey::Pubkey,
    pub dlmm_event_authority: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FillDlmm {
    type ArrangedAccounts = FillDlmmInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let token_vault = accounts.get(1)?;
        let token_out_vault = accounts.get(2)?;
        let amm_program = accounts.get(3)?;
        let pool = accounts.get(4)?;
        let bin_array_bitmap_extension = accounts.get(5)?;
        let reserve_x = accounts.get(6)?;
        let reserve_y = accounts.get(7)?;
        let token_x_mint = accounts.get(8)?;
        let token_y_mint = accounts.get(9)?;
        let oracle = accounts.get(10)?;
        let token_x_program = accounts.get(11)?;
        let token_y_program = accounts.get(12)?;
        let dlmm_event_authority = accounts.get(13)?;
        let event_authority = accounts.get(14)?;
        let program = accounts.get(15)?;

        Some(FillDlmmInstructionAccounts {
            vault: vault.pubkey,
            token_vault: token_vault.pubkey,
            token_out_vault: token_out_vault.pubkey,
            amm_program: amm_program.pubkey,
            pool: pool.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            token_x_mint: token_x_mint.pubkey,
            token_y_mint: token_y_mint.pubkey,
            oracle: oracle.pubkey,
            token_x_program: token_x_program.pubkey,
            token_y_program: token_y_program.pubkey,
            dlmm_event_authority: dlmm_event_authority.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}