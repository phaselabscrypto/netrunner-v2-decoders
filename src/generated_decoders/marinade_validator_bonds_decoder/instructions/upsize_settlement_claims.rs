

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xcf2e22588d243f84")]
pub struct UpsizeSettlementClaims{
}

pub struct UpsizeSettlementClaimsInstructionAccounts {
    pub settlement_claims: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpsizeSettlementClaims {
    type ArrangedAccounts = UpsizeSettlementClaimsInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let settlement_claims = accounts.get(0)?;
        let rent_payer = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(UpsizeSettlementClaimsInstructionAccounts {
            settlement_claims: settlement_claims.pubkey,
            rent_payer: rent_payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}