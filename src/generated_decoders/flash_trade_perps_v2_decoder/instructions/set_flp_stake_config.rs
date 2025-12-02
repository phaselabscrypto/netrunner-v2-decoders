use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6613df77631509a7")]
pub struct SetFlpStakeConfig {
    pub params: SetFlpStakeConfigParams,
}

pub struct SetFlpStakeConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub flp_stake_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFlpStakeConfig {
    type ArrangedAccounts = SetFlpStakeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, flp_stake_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetFlpStakeConfigInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            flp_stake_account: flp_stake_account.pubkey,
        })
    }
}
