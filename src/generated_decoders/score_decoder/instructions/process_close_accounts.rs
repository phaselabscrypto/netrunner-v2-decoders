

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xac02c5aa71e1be5a")]
pub struct ProcessCloseAccounts{
    pub staking_bump: u8,
    pub scorevars_bump: u8,
    pub ship_bump: u8,
    pub fuel_bump: u8,
    pub food_bump: u8,
    pub arms_bump: u8,
    pub escrow_auth_bump: u8,
}

pub struct ProcessCloseAccountsInstructionAccounts {
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub ship_token_account_escrow: solana_sdk::pubkey::Pubkey,
    pub fuel_token_account_escrow: solana_sdk::pubkey::Pubkey,
    pub food_token_account_escrow: solana_sdk::pubkey::Pubkey,
    pub arms_token_account_escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
    pub fuel_mint: solana_sdk::pubkey::Pubkey,
    pub food_mint: solana_sdk::pubkey::Pubkey,
    pub arms_mint: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessCloseAccounts {
    type ArrangedAccounts = ProcessCloseAccountsInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            player_account,
            ship_staking_account,
            score_vars_account,
            ship_token_account_escrow,
            fuel_token_account_escrow,
            food_token_account_escrow,
            arms_token_account_escrow,
            escrow_authority,
            token_program,
            system_program,
            ship_mint,
            fuel_mint,
            food_mint,
            arms_mint,
            clock,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ProcessCloseAccountsInstructionAccounts {
            player_account: player_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            ship_token_account_escrow: ship_token_account_escrow.pubkey,
            fuel_token_account_escrow: fuel_token_account_escrow.pubkey,
            food_token_account_escrow: food_token_account_escrow.pubkey,
            arms_token_account_escrow: arms_token_account_escrow.pubkey,
            escrow_authority: escrow_authority.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            ship_mint: ship_mint.pubkey,
            fuel_mint: fuel_mint.pubkey,
            food_mint: food_mint.pubkey,
            arms_mint: arms_mint.pubkey,
            clock: clock.pubkey,
        })
    }
}