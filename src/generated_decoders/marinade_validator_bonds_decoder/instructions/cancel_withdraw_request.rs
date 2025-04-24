use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa7646e80719ae04d")]
pub struct CancelWithdrawRequest {}

pub struct CancelWithdrawRequestInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_request: solana_sdk::pubkey::Pubkey,
    pub rent_collector: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelWithdrawRequest {
    type ArrangedAccounts = CancelWithdrawRequestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let vote_account = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let withdraw_request = accounts.get(4)?;
        let rent_collector = accounts.get(5)?;
        let event_authority = accounts.get(6)?;
        let program = accounts.get(7)?;

        Some(CancelWithdrawRequestInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            vote_account: vote_account.pubkey,
            authority: authority.pubkey,
            withdraw_request: withdraw_request.pubkey,
            rent_collector: rent_collector.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
