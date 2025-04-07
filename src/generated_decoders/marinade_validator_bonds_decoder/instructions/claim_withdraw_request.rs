

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x30e8173414867a76")]
pub struct ClaimWithdrawRequest{
}

pub struct ClaimWithdrawRequestInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_request: solana_sdk::pubkey::Pubkey,
    pub bonds_withdrawer_authority: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub withdrawer: solana_sdk::pubkey::Pubkey,
    pub split_stake_account: solana_sdk::pubkey::Pubkey,
    pub split_stake_rent_payer: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimWithdrawRequest {
    type ArrangedAccounts = ClaimWithdrawRequestInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let vote_account = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let withdraw_request = accounts.get(4)?;
        let bonds_withdrawer_authority = accounts.get(5)?;
        let stake_account = accounts.get(6)?;
        let withdrawer = accounts.get(7)?;
        let split_stake_account = accounts.get(8)?;
        let split_stake_rent_payer = accounts.get(9)?;
        let stake_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let stake_history = accounts.get(12)?;
        let clock = accounts.get(13)?;
        let event_authority = accounts.get(14)?;
        let program = accounts.get(15)?;

        Some(ClaimWithdrawRequestInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            vote_account: vote_account.pubkey,
            authority: authority.pubkey,
            withdraw_request: withdraw_request.pubkey,
            bonds_withdrawer_authority: bonds_withdrawer_authority.pubkey,
            stake_account: stake_account.pubkey,
            withdrawer: withdrawer.pubkey,
            split_stake_account: split_stake_account.pubkey,
            split_stake_rent_payer: split_stake_rent_payer.pubkey,
            stake_program: stake_program.pubkey,
            system_program: system_program.pubkey,
            stake_history: stake_history.pubkey,
            clock: clock.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}