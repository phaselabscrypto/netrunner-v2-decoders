

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x21f1603ee4b20178")]
pub struct CancelSettlement{
}

pub struct CancelSettlementInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub settlement_claims: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub bonds_withdrawer_authority: solana_sdk::pubkey::Pubkey,
    pub rent_collector: solana_sdk::pubkey::Pubkey,
    pub split_rent_collector: solana_sdk::pubkey::Pubkey,
    pub split_rent_refund_account: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelSettlement {
    type ArrangedAccounts = CancelSettlementInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let settlement = accounts.get(2)?;
        let settlement_claims = accounts.get(3)?;
        let authority = accounts.get(4)?;
        let bonds_withdrawer_authority = accounts.get(5)?;
        let rent_collector = accounts.get(6)?;
        let split_rent_collector = accounts.get(7)?;
        let split_rent_refund_account = accounts.get(8)?;
        let clock = accounts.get(9)?;
        let stake_program = accounts.get(10)?;
        let stake_history = accounts.get(11)?;
        let event_authority = accounts.get(12)?;
        let program = accounts.get(13)?;

        Some(CancelSettlementInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            settlement: settlement.pubkey,
            settlement_claims: settlement_claims.pubkey,
            authority: authority.pubkey,
            bonds_withdrawer_authority: bonds_withdrawer_authority.pubkey,
            rent_collector: rent_collector.pubkey,
            split_rent_collector: split_rent_collector.pubkey,
            split_rent_refund_account: split_rent_refund_account.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
            stake_history: stake_history.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}