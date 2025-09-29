

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x9e9c5d4837219895")]
pub struct ProcessWithdrawFood{
    pub staking_bump: u8,
    pub scorevars_bump: u8,
    pub scorevars_ship_bump: u8,
    pub escrow_auth_bump: u8,
    pub escrow_bump: u8,
}

pub struct ProcessWithdrawFoodInstructionAccounts {
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub food_token_account_escrow: solana_sdk::pubkey::Pubkey,
    pub food_token_account_return: solana_sdk::pubkey::Pubkey,
    pub food_mint: solana_sdk::pubkey::Pubkey,
    pub escrow_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessWithdrawFood {
    type ArrangedAccounts = ProcessWithdrawFoodInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            player_account,
            ship_staking_account,
            score_vars_account,
            score_vars_ship_account,
            food_token_account_escrow,
            food_token_account_return,
            food_mint,
            escrow_authority,
            token_program,
            clock,
            ship_mint,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ProcessWithdrawFoodInstructionAccounts {
            player_account: player_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            food_token_account_escrow: food_token_account_escrow.pubkey,
            food_token_account_return: food_token_account_return.pubkey,
            food_mint: food_mint.pubkey,
            escrow_authority: escrow_authority.pubkey,
            token_program: token_program.pubkey,
            clock: clock.pubkey,
            ship_mint: ship_mint.pubkey,
        })
    }
}