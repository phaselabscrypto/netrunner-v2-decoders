use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa8ac184dc59c8765")]
pub struct SwapWithDestination {
    pub params: SwapParams,
}

pub struct SwapWithDestinationInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub user_token_authority: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_authority: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapWithDestination {
    type ArrangedAccounts = SwapWithDestinationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let associated_token_program = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let user_token_authority = accounts.get(3)?;
        let destination_token_account = accounts.get(4)?;
        let destination_token_authority = accounts.get(5)?;
        let destination_mint = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(SwapWithDestinationInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            user_token_authority: user_token_authority.pubkey,
            destination_token_account: destination_token_account.pubkey,
            destination_token_authority: destination_token_authority.pubkey,
            destination_mint: destination_mint.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
