use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x91851fd8cbc66008")]
pub struct CancelUnstakeTokenRequest {
    pub params: CancelUnstakeTokenRequestParams,
}

pub struct CancelUnstakeTokenRequestInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelUnstakeTokenRequest {
    type ArrangedAccounts = CancelUnstakeTokenRequestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, token_vault, token_stake_account, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CancelUnstakeTokenRequestInstructionAccounts {
            owner: owner.pubkey,
            token_vault: token_vault.pubkey,
            token_stake_account: token_stake_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
