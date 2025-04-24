use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc9a73c9a0718edf7")]
pub struct SetSharedEscrow {
    pub args: SetSharedEscrowArgs,
}

pub struct SetSharedEscrowInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub shared_escrow_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetSharedEscrow {
    type ArrangedAccounts = SetSharedEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let cosigner = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let shared_escrow_account = accounts.get(3)?;

        Some(SetSharedEscrowInstructionAccounts {
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            pool: pool.pubkey,
            shared_escrow_account: shared_escrow_account.pubkey,
        })
    }
}
