use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6ec9be40a6ba6983")]
pub struct SetPoolFreeze {
    pub config: PoolConfig,
    pub freeze: bool,
}

pub struct SetPoolFreezeInstructionAccounts {
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPoolFreeze {
    type ArrangedAccounts = SetPoolFreezeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let system_program = accounts.get(0)?;

        Some(SetPoolFreezeInstructionAccounts {
            system_program: system_program.pubkey,
        })
    }
}
