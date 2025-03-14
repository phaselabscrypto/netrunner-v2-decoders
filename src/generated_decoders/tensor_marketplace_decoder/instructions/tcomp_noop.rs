
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6aa20ae28444df15")]
pub struct TcompNoop{
    pub event: TcompEvent,
}

pub struct TcompNoopInstructionAccounts {
    pub tcomp_signer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TcompNoop {
    type ArrangedAccounts = TcompNoopInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tcomp_signer = accounts.get(0)?;

        Some(TcompNoopInstructionAccounts {
            tcomp_signer: tcomp_signer.pubkey,
        })
    }
}