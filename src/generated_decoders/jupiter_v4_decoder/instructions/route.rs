use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe517cb977ae3ad2a")]
pub struct Route {
    pub swap_leg: SwapLeg,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

pub struct RouteInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Route {
    type ArrangedAccounts = RouteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let user_transfer_authority = accounts.get(1)?;
        let destination_token_account = accounts.get(2)?;

        Some(RouteInstructionAccounts {
            token_program: token_program.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            destination_token_account: destination_token_account.pubkey,
        })
    }
}
