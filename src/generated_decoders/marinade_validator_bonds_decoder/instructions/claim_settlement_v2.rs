
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xbc358497583234ee")]
pub struct ClaimSettlementV2{
    pub claim_settlement_args: ClaimSettlementV2Args,
}

pub struct ClaimSettlementV2InstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub settlement_claims: solana_sdk::pubkey::Pubkey,
    pub stake_account_from: solana_sdk::pubkey::Pubkey,
    pub stake_account_to: solana_sdk::pubkey::Pubkey,
    pub bonds_withdrawer_authority: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimSettlementV2 {
    type ArrangedAccounts = ClaimSettlementV2InstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let settlement = accounts.get(2)?;
        let settlement_claims = accounts.get(3)?;
        let stake_account_from = accounts.get(4)?;
        let stake_account_to = accounts.get(5)?;
        let bonds_withdrawer_authority = accounts.get(6)?;
        let stake_history = accounts.get(7)?;
        let clock = accounts.get(8)?;
        let stake_program = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;

        Some(ClaimSettlementV2InstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            settlement: settlement.pubkey,
            settlement_claims: settlement_claims.pubkey,
            stake_account_from: stake_account_from.pubkey,
            stake_account_to: stake_account_to.pubkey,
            bonds_withdrawer_authority: bonds_withdrawer_authority.pubkey,
            stake_history: stake_history.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}