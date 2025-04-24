use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f5d5db5dd247e40")]
pub struct InitBond {
    pub init_bond_args: InitBondArgs,
}

pub struct InitBondInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub validator_identity: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitBond {
    type ArrangedAccounts = InitBondInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let vote_account = accounts.get(1)?;
        let validator_identity = accounts.get(2)?;
        let bond = accounts.get(3)?;
        let rent_payer = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let event_authority = accounts.get(6)?;
        let program = accounts.get(7)?;

        Some(InitBondInstructionAccounts {
            config: config.pubkey,
            vote_account: vote_account.pubkey,
            validator_identity: validator_identity.pubkey,
            bond: bond.pubkey,
            rent_payer: rent_payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
