

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x36fdbc2264913b7f")]
pub struct WithdrawRemainingQuote{
}

pub struct WithdrawRemainingQuoteInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub destination_token: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawRemainingQuote {
    type ArrangedAccounts = WithdrawRemainingQuoteInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let escrow = accounts.get(2)?;
        let token_vault = accounts.get(3)?;
        let destination_token = accounts.get(4)?;
        let token_mint = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let owner = accounts.get(7)?;
        let event_authority = accounts.get(8)?;
        let program = accounts.get(9)?;

        Some(WithdrawRemainingQuoteInstructionAccounts {
            vault: vault.pubkey,
            pool: pool.pubkey,
            escrow: escrow.pubkey,
            token_vault: token_vault.pubkey,
            destination_token: destination_token.pubkey,
            token_mint: token_mint.pubkey,
            token_program: token_program.pubkey,
            owner: owner.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}