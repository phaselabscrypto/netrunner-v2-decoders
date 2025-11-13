use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x61d1dc5f72a7e167")]
pub struct SetTokenReward {
    pub params: SetTokenRewardParams,
}

pub struct SetTokenRewardInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenReward {
    type ArrangedAccounts = SetTokenRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, token_vault, token_stake_account, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetTokenRewardInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            token_vault: token_vault.pubkey,
            token_stake_account: token_stake_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
