use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x49401186db476753")]
pub struct SetWhitelistConfig {
    pub params: SetWhitelistConfigParams,
}

pub struct SetWhitelistConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetWhitelistConfig {
    type ArrangedAccounts = SetWhitelistConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, owner, whitelist, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetWhitelistConfigInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            owner: owner.pubkey,
            whitelist: whitelist.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
