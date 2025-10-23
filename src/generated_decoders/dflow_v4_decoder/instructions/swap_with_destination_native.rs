use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcd4d7f6cf120c4c3")]
pub struct SwapWithDestinationNative {
    pub params: SwapParams,
}

pub struct SwapWithDestinationNativeInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub user_token_authority: solana_sdk::pubkey::Pubkey,
    pub destination_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapWithDestinationNative {
    type ArrangedAccounts = SwapWithDestinationNativeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let associated_token_program = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let user_token_authority = accounts.get(3)?;
        let destination_account = accounts.get(4)?;
        let event_authority = accounts.get(5)?;
        let program = accounts.get(6)?;

        Some(SwapWithDestinationNativeInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            user_token_authority: user_token_authority.pubkey,
            destination_account: destination_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
