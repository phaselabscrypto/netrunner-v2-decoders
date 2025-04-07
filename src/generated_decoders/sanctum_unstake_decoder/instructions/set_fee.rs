
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x129a1812edd61350")]
pub struct SetFee{
    pub fee: Fee,
}

pub struct SetFeeInstructionAccounts {
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub fee_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFee {
    type ArrangedAccounts = SetFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let fee_authority = accounts.get(0)?;
        let pool_account = accounts.get(1)?;
        let fee_account = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let rent = accounts.get(4)?;

        Some(SetFeeInstructionAccounts {
            fee_authority: fee_authority.pubkey,
            pool_account: pool_account.pubkey,
            fee_account: fee_account.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}