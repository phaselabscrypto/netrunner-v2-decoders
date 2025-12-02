use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x80e7aac5b1f686ee")]
pub struct UnstakeTokenRequest {
    pub params: UnstakeTokenRequestParams,
}

pub struct UnstakeTokenRequestInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnstakeTokenRequest {
    type ArrangedAccounts = UnstakeTokenRequestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, token_vault, token_stake_account, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UnstakeTokenRequestInstructionAccounts {
            owner: owner.pubkey,
            token_vault: token_vault.pubkey,
            token_stake_account: token_stake_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
