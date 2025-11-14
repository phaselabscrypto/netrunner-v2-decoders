use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4ab8418f88a5b206")]
pub struct SetTokenStakeLevel {
    pub params: SetTokenStakeLevelParams,
}

pub struct SetTokenStakeLevelInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenStakeLevel {
    type ArrangedAccounts = SetTokenStakeLevelInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, token_stake_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetTokenStakeLevelInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            token_stake_account: token_stake_account.pubkey,
        })
    }
}
