use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xebc91185ea4854d2")]
pub struct LiquidateBorrowPosition {
    pub params: LiquidateBorrowPositionParams,
}

pub struct LiquidateBorrowPositionInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub borrow_position: solana_sdk::pubkey::Pubkey,
    pub collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidateBorrowPosition {
    type ArrangedAccounts = LiquidateBorrowPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, perpetuals, pool, custody, transfer_authority, borrow_position, collateral_token_account, lp_token_mint, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LiquidateBorrowPositionInstructionAccounts {
            signer: signer.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            transfer_authority: transfer_authority.pubkey,
            borrow_position: borrow_position.pubkey,
            collateral_token_account: collateral_token_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
