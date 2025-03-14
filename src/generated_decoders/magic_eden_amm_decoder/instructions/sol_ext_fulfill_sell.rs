
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7913c7be30f0b673")]
pub struct SolExtFulfillSell{
    pub args: SolFulfillSellArgs,
}

pub struct SolExtFulfillSellInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub buyside_sol_escrow_account: solana_sdk::pubkey::Pubkey,
    pub asset_mint: solana_sdk::pubkey::Pubkey,
    pub sellside_escrow_token_account: solana_sdk::pubkey::Pubkey,
    pub payer_asset_account: solana_sdk::pubkey::Pubkey,
    pub sell_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SolExtFulfillSell {
    type ArrangedAccounts = SolExtFulfillSellInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let cosigner = accounts.get(2)?;
        let referral = accounts.get(3)?;
        let pool = accounts.get(4)?;
        let buyside_sol_escrow_account = accounts.get(5)?;
        let asset_mint = accounts.get(6)?;
        let sellside_escrow_token_account = accounts.get(7)?;
        let payer_asset_account = accounts.get(8)?;
        let sell_state = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let associated_token_program = accounts.get(12)?;

        Some(SolExtFulfillSellInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            referral: referral.pubkey,
            pool: pool.pubkey,
            buyside_sol_escrow_account: buyside_sol_escrow_account.pubkey,
            asset_mint: asset_mint.pubkey,
            sellside_escrow_token_account: sellside_escrow_token_account.pubkey,
            payer_asset_account: payer_asset_account.pubkey,
            sell_state: sell_state.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}