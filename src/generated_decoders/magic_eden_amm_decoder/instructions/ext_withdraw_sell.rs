
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x337c8008ea3b1fcf")]
pub struct ExtWithdrawSell{
    pub args: WithdrawSellArgs,
}

pub struct ExtWithdrawSellInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub asset_mint: solana_sdk::pubkey::Pubkey,
    pub asset_token_account: solana_sdk::pubkey::Pubkey,
    pub sellside_escrow_token_account: solana_sdk::pubkey::Pubkey,
    pub buyside_sol_escrow_account: solana_sdk::pubkey::Pubkey,
    pub allowlist_aux_account: solana_sdk::pubkey::Pubkey,
    pub sell_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExtWithdrawSell {
    type ArrangedAccounts = ExtWithdrawSellInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let cosigner = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let asset_mint = accounts.get(3)?;
        let asset_token_account = accounts.get(4)?;
        let sellside_escrow_token_account = accounts.get(5)?;
        let buyside_sol_escrow_account = accounts.get(6)?;
        let allowlist_aux_account = accounts.get(7)?;
        let sell_state = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let associated_token_program = accounts.get(11)?;

        Some(ExtWithdrawSellInstructionAccounts {
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            pool: pool.pubkey,
            asset_mint: asset_mint.pubkey,
            asset_token_account: asset_token_account.pubkey,
            sellside_escrow_token_account: sellside_escrow_token_account.pubkey,
            buyside_sol_escrow_account: buyside_sol_escrow_account.pubkey,
            allowlist_aux_account: allowlist_aux_account.pubkey,
            sell_state: sell_state.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}