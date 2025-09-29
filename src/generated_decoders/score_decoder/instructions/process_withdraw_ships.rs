

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4bc617d1abe4cf55")]
pub struct ProcessWithdrawShips{
    pub staking_bump: u8,
    pub scorevars_bump: u8,
    pub scorevars_ship_bump: u8,
    pub escrow_auth_bump: u8,
    pub escrow_bump: u8,
    pub treasury_auth_bump: u8,
    pub treasury_bump: u8,
}

pub struct ProcessWithdrawShipsInstructionAccounts {
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub player_atlas_token_account: solana_sdk::pubkey::Pubkey,
    pub ship_token_account_escrow: solana_sdk::pubkey::Pubkey,
    pub ship_token_account_return: solana_sdk::pubkey::Pubkey,
    pub toolkit_token_account_source: solana_sdk::pubkey::Pubkey,
    pub treasury_token_account: solana_sdk::pubkey::Pubkey,
    pub treasury_authority_account: solana_sdk::pubkey::Pubkey,
    pub escrow_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
    pub toolkit_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessWithdrawShips {
    type ArrangedAccounts = ProcessWithdrawShipsInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            player_account,
            ship_staking_account,
            score_vars_account,
            score_vars_ship_account,
            player_atlas_token_account,
            ship_token_account_escrow,
            ship_token_account_return,
            toolkit_token_account_source,
            treasury_token_account,
            treasury_authority_account,
            escrow_authority,
            token_program,
            system_program,
            clock,
            ship_mint,
            toolkit_mint,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ProcessWithdrawShipsInstructionAccounts {
            player_account: player_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            player_atlas_token_account: player_atlas_token_account.pubkey,
            ship_token_account_escrow: ship_token_account_escrow.pubkey,
            ship_token_account_return: ship_token_account_return.pubkey,
            toolkit_token_account_source: toolkit_token_account_source.pubkey,
            treasury_token_account: treasury_token_account.pubkey,
            treasury_authority_account: treasury_authority_account.pubkey,
            escrow_authority: escrow_authority.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            clock: clock.pubkey,
            ship_mint: ship_mint.pubkey,
            toolkit_mint: toolkit_mint.pubkey,
        })
    }
}