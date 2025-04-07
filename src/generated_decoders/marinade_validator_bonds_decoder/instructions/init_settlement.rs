
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x98b2004134d2f73a")]
pub struct InitSettlement{
    pub init_settlement_args: InitSettlementArgs,
}

pub struct InitSettlementInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub settlement_claims: solana_sdk::pubkey::Pubkey,
    pub operator_authority: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitSettlement {
    type ArrangedAccounts = InitSettlementInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let settlement = accounts.get(2)?;
        let settlement_claims = accounts.get(3)?;
        let operator_authority = accounts.get(4)?;
        let rent_payer = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(InitSettlementInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            settlement: settlement.pubkey,
            settlement_claims: settlement_claims.pubkey,
            operator_authority: operator_authority.pubkey,
            rent_payer: rent_payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}