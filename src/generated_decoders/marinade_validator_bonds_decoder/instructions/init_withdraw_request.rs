use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8e1fded7534f2231")]
pub struct InitWithdrawRequest {
    pub create_withdraw_request_args: InitWithdrawRequestArgs,
}

pub struct InitWithdrawRequestInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_request: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitWithdrawRequest {
    type ArrangedAccounts = InitWithdrawRequestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let vote_account = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let withdraw_request = accounts.get(4)?;
        let rent_payer = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(InitWithdrawRequestInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            vote_account: vote_account.pubkey,
            authority: authority.pubkey,
            withdraw_request: withdraw_request.pubkey,
            rent_payer: rent_payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
