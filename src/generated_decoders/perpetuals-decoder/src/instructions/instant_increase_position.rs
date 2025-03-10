
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa47e44b6dfa640b7")]
pub struct InstantIncreasePosition{
    pub params: InstantIncreasePositionParams,
}

pub struct InstantIncreasePositionInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub api_keeper: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InstantIncreasePosition {
    type ArrangedAccounts = InstantIncreasePositionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let api_keeper = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let funding_account = accounts.get(3)?;
        let perpetuals = accounts.get(4)?;
        let pool = accounts.get(5)?;
        let position = accounts.get(6)?;
        let custody = accounts.get(7)?;
        let custody_doves_price_account = accounts.get(8)?;
        let custody_pythnet_price_account = accounts.get(9)?;
        let collateral_custody = accounts.get(10)?;
        let collateral_custody_doves_price_account = accounts.get(11)?;
        let collateral_custody_pythnet_price_account = accounts.get(12)?;
        let collateral_custody_token_account = accounts.get(13)?;
        let token_ledger = accounts.get(14)?;
        let referral = accounts.get(15)?;
        let token_program = accounts.get(16)?;
        let system_program = accounts.get(17)?;
        let event_authority = accounts.get(18)?;
        let program = accounts.get(19)?;

        Some(InstantIncreasePositionInstructionAccounts {
            keeper: keeper.pubkey,
            api_keeper: api_keeper.pubkey,
            owner: owner.pubkey,
            funding_account: funding_account.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_custody_doves_price_account: collateral_custody_doves_price_account.pubkey,
            collateral_custody_pythnet_price_account: collateral_custody_pythnet_price_account.pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            token_ledger: token_ledger.pubkey,
            referral: referral.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}