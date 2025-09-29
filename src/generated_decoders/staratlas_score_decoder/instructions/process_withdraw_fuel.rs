

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2d9393a320dda1b0")]
pub struct ProcessWithdrawFuel{
    pub staking_bump: u8,
    pub scorevars_bump: u8,
    pub scorevars_ship_bump: u8,
    pub escrow_auth_bump: u8,
    pub escrow_bump: u8,
}

pub struct ProcessWithdrawFuelInstructionAccounts {
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub fuel_token_account_escrow: solana_sdk::pubkey::Pubkey,
    pub fuel_token_account_return: solana_sdk::pubkey::Pubkey,
    pub fuel_mint: solana_sdk::pubkey::Pubkey,
    pub escrow_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessWithdrawFuel {
    type ArrangedAccounts = ProcessWithdrawFuelInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            player_account,
            ship_staking_account,
            score_vars_account,
            score_vars_ship_account,
            fuel_token_account_escrow,
            fuel_token_account_return,
            fuel_mint,
            escrow_authority,
            token_program,
            clock,
            ship_mint,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ProcessWithdrawFuelInstructionAccounts {
            player_account: player_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            fuel_token_account_escrow: fuel_token_account_escrow.pubkey,
            fuel_token_account_return: fuel_token_account_return.pubkey,
            fuel_mint: fuel_mint.pubkey,
            escrow_authority: escrow_authority.pubkey,
            token_program: token_program.pubkey,
            clock: clock.pubkey,
            ship_mint: ship_mint.pubkey,
        })
    }
}