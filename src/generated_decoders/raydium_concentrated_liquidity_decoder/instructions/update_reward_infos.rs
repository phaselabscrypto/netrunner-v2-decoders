

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa3ace0340b9a6adf")]
pub struct UpdateRewardInfos{
}

pub struct UpdateRewardInfosInstructionAccounts {
    pub pool_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRewardInfos {
    type ArrangedAccounts = UpdateRewardInfosInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let pool_state = accounts.get(0)?;

        Some(UpdateRewardInfosInstructionAccounts {
            pool_state: pool_state.pubkey,
        })
    }
}