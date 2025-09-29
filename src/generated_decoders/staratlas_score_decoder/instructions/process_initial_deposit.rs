

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfb162ce7c989daa2")]
pub struct ProcessInitialDeposit{
    pub staking_bump: u8,
    pub scorevars_ship_bump: u8,
    pub escrow_auth_bump: u8,
    pub escrow_bump: u8,
    pub ship_quantity: u64,
}

pub struct ProcessInitialDepositInstructionAccounts {
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub player_faction_account: solana_sdk::pubkey::Pubkey,
    pub escrow_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
    pub ship_token_account_source: solana_sdk::pubkey::Pubkey,
    pub ship_token_account_escrow: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessInitialDeposit {
    type ArrangedAccounts = ProcessInitialDepositInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            player_account,
            ship_staking_account,
            score_vars_ship_account,
            player_faction_account,
            escrow_authority,
            system_program,
            token_program,
            clock,
            rent,
            ship_mint,
            ship_token_account_source,
            ship_token_account_escrow,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ProcessInitialDepositInstructionAccounts {
            player_account: player_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            player_faction_account: player_faction_account.pubkey,
            escrow_authority: escrow_authority.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            ship_mint: ship_mint.pubkey,
            ship_token_account_source: ship_token_account_source.pubkey,
            ship_token_account_escrow: ship_token_account_escrow.pubkey,
        })
    }
}