use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb185205ae5d8832f")]
pub struct NewLocker {
    pub params: LockerParams,
}

pub struct NewLockerInstructionAccounts {
    pub base: solana_sdk::pubkey::Pubkey,
    pub locker: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub governor: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for NewLocker {
    type ArrangedAccounts = NewLockerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [base, locker, token_mint, governor, payer, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(NewLockerInstructionAccounts {
            base: base.pubkey,
            locker: locker.pubkey,
            token_mint: token_mint.pubkey,
            governor: governor.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
