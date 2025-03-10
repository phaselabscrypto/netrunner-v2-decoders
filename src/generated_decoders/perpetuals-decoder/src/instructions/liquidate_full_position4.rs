
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x40b05833a8bc9caf")]
pub struct LiquidateFullPosition4{
    pub params: LiquidateFullPosition4Params,
}

pub struct LiquidateFullPosition4InstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
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
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidateFullPosition4 {
    type ArrangedAccounts = LiquidateFullPosition4InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let signer = accounts.get(0)?;
        let perpetuals = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let position = accounts.get(3)?;
        let custody = accounts.get(4)?;
        let custody_doves_price_account = accounts.get(5)?;
        let custody_pythnet_price_account = accounts.get(6)?;
        let collateral_custody = accounts.get(7)?;
        let collateral_custody_doves_price_account = accounts.get(8)?;
        let collateral_custody_pythnet_price_account = accounts.get(9)?;
        let collateral_custody_token_account = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;

        Some(LiquidateFullPosition4InstructionAccounts {
            signer: signer.pubkey,
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
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}