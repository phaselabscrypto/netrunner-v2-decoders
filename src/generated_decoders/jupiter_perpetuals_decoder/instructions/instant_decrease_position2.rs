use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa2bfc83e8b3eb011")]
pub struct InstantDecreasePosition2 {
    pub params: InstantDecreasePosition2Params,
}

pub struct InstantDecreasePosition2InstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub api_keeper: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub desired_mint: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub position_request: solana_sdk::pubkey::Pubkey,
    pub position_request_ata: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InstantDecreasePosition2 {
    type ArrangedAccounts = InstantDecreasePosition2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, api_keeper, owner, receiving_account, transfer_authority, perpetuals, pool, position, custody, custody_doves_price_account, collateral_custody, collateral_custody_doves_price_account, collateral_custody_token_account, desired_mint, referral, position_request, position_request_ata, token_program, associated_token_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InstantDecreasePosition2InstructionAccounts {
            keeper: keeper.pubkey,
            api_keeper: api_keeper.pubkey,
            owner: owner.pubkey,
            receiving_account: receiving_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_custody_doves_price_account: collateral_custody_doves_price_account.pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            desired_mint: desired_mint.pubkey,
            referral: referral.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
