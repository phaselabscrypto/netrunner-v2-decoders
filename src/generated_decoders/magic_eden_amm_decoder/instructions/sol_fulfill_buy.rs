use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5c10e24f1ff23576")]
pub struct SolFulfillBuy {
    pub args: SolFulfillBuyArgs,
}

pub struct SolFulfillBuyInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub buyside_sol_escrow_account: solana_sdk::pubkey::Pubkey,
    pub asset_metadata: solana_sdk::pubkey::Pubkey,
    pub asset_master_edition: solana_sdk::pubkey::Pubkey,
    pub asset_mint: solana_sdk::pubkey::Pubkey,
    pub payer_asset_account: solana_sdk::pubkey::Pubkey,
    pub sellside_escrow_token_account: solana_sdk::pubkey::Pubkey,
    pub owner_token_account: solana_sdk::pubkey::Pubkey,
    pub allowlist_aux_account: solana_sdk::pubkey::Pubkey,
    pub sell_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SolFulfillBuy {
    type ArrangedAccounts = SolFulfillBuyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let cosigner = accounts.get(2)?;
        let referral = accounts.get(3)?;
        let pool = accounts.get(4)?;
        let buyside_sol_escrow_account = accounts.get(5)?;
        let asset_metadata = accounts.get(6)?;
        let asset_master_edition = accounts.get(7)?;
        let asset_mint = accounts.get(8)?;
        let payer_asset_account = accounts.get(9)?;
        let sellside_escrow_token_account = accounts.get(10)?;
        let owner_token_account = accounts.get(11)?;
        let allowlist_aux_account = accounts.get(12)?;
        let sell_state = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let associated_token_program = accounts.get(16)?;
        let rent = accounts.get(17)?;

        Some(SolFulfillBuyInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            referral: referral.pubkey,
            pool: pool.pubkey,
            buyside_sol_escrow_account: buyside_sol_escrow_account.pubkey,
            asset_metadata: asset_metadata.pubkey,
            asset_master_edition: asset_master_edition.pubkey,
            asset_mint: asset_mint.pubkey,
            payer_asset_account: payer_asset_account.pubkey,
            sellside_escrow_token_account: sellside_escrow_token_account.pubkey,
            owner_token_account: owner_token_account.pubkey,
            allowlist_aux_account: allowlist_aux_account.pubkey,
            sell_state: sell_state.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
