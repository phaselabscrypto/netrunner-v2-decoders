use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6ae44e58708bb977")]
pub struct SetTokenVaultConfig {
    pub params: SetTokenVaultConfigParams,
}

pub struct SetTokenVaultConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenVaultConfig {
    type ArrangedAccounts = SetTokenVaultConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, token_vault, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetTokenVaultConfigInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            token_vault: token_vault.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
