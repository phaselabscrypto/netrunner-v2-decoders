use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe60a208454720eff")]
pub struct UpdateTradingAccount {
    pub params: UpdateTradingAccountParams,
}

pub struct UpdateTradingAccountInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub nft_token_account: solana_sdk::pubkey::Pubkey,
    pub trading_account: solana_sdk::pubkey::Pubkey,
    pub referral_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateTradingAccount {
    type ArrangedAccounts = UpdateTradingAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, fee_payer, nft_token_account, trading_account, referral_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdateTradingAccountInstructionAccounts {
            owner: owner.pubkey,
            fee_payer: fee_payer.pubkey,
            nft_token_account: nft_token_account.pubkey,
            trading_account: trading_account.pubkey,
            referral_account: referral_account.pubkey,
        })
    }
}
