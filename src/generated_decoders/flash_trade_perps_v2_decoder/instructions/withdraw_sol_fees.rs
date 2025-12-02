use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbf35a6617cd4e4db")]
pub struct WithdrawSolFees {
    pub params: WithdrawSolFeesParams,
}

pub struct WithdrawSolFeesInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub receiving_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawSolFees {
    type ArrangedAccounts = WithdrawSolFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, transfer_authority, perpetuals, receiving_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawSolFeesInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            receiving_account: receiving_account.pubkey,
        })
    }
}
