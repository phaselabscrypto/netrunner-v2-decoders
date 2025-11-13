use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3d11f0f5ac429fe8")]
pub struct CreateReferral {
    pub params: CreateReferralParams,
}

pub struct CreateReferralInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
    pub referral_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateReferral {
    type ArrangedAccounts = CreateReferralInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, fee_payer, token_stake_account, referral_account, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateReferralInstructionAccounts {
            owner: owner.pubkey,
            fee_payer: fee_payer.pubkey,
            token_stake_account: token_stake_account.pubkey,
            referral_account: referral_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
