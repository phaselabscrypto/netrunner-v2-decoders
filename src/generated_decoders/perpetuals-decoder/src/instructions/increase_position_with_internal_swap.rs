
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x72376a8cc7dd2070")]
pub struct IncreasePositionWithInternalSwap{
    pub params: IncreasePositionWithInternalSwapParams,
}

pub struct IncreasePositionWithInternalSwapInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position_request: solana_sdk::pubkey::Pubkey,
    pub position_request_ata: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub receiving_custody: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreasePositionWithInternalSwap {
    type ArrangedAccounts = IncreasePositionWithInternalSwapInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let perpetuals = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let position_request = accounts.get(3)?;
        let position_request_ata = accounts.get(4)?;
        let position = accounts.get(5)?;
        let custody = accounts.get(6)?;
        let custody_doves_price_account = accounts.get(7)?;
        let custody_pythnet_price_account = accounts.get(8)?;
        let collateral_custody = accounts.get(9)?;
        let collateral_custody_doves_price_account = accounts.get(10)?;
        let collateral_custody_pythnet_price_account = accounts.get(11)?;
        let collateral_custody_token_account = accounts.get(12)?;
        let receiving_custody = accounts.get(13)?;
        let receiving_custody_doves_price_account = accounts.get(14)?;
        let receiving_custody_pythnet_price_account = accounts.get(15)?;
        let receiving_custody_token_account = accounts.get(16)?;
        let token_program = accounts.get(17)?;
        let event_authority = accounts.get(18)?;
        let program = accounts.get(19)?;

        Some(IncreasePositionWithInternalSwapInstructionAccounts {
            keeper: keeper.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            position: position.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_custody_doves_price_account: collateral_custody_doves_price_account.pubkey,
            collateral_custody_pythnet_price_account: collateral_custody_pythnet_price_account.pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            receiving_custody: receiving_custody.pubkey,
            receiving_custody_doves_price_account: receiving_custody_doves_price_account.pubkey,
            receiving_custody_pythnet_price_account: receiving_custody_pythnet_price_account.pubkey,
            receiving_custody_token_account: receiving_custody_token_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}