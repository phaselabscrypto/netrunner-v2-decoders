use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc6d4ab6d90d7ae59")]
pub struct WithdrawFees {
    pub params: WithdrawFeesParams,
}

pub struct WithdrawFeesInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub protocol_vault: solana_sdk::pubkey::Pubkey,
    pub protocol_token_account: solana_sdk::pubkey::Pubkey,
    pub receiving_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub receiving_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFees {
    type ArrangedAccounts = WithdrawFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, transfer_authority, perpetuals, protocol_vault, protocol_token_account, receiving_token_account, token_program, receiving_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawFeesInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            protocol_vault: protocol_vault.pubkey,
            protocol_token_account: protocol_token_account.pubkey,
            receiving_token_account: receiving_token_account.pubkey,
            token_program: token_program.pubkey,
            receiving_mint: receiving_mint.pubkey,
        })
    }
}
