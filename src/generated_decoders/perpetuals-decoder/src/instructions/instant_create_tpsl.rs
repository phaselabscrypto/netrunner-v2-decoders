
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7562427f1e3249b9")]
pub struct InstantCreateTpsl{
    pub params: InstantCreateTpslParams,
}

pub struct InstantCreateTpslInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub api_keeper: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_account: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_request: solana_sdk::pubkey::Pubkey,
    pub position_request_ata: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub desired_mint: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InstantCreateTpsl {
    type ArrangedAccounts = InstantCreateTpslInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let api_keeper = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let receiving_account = accounts.get(3)?;
        let perpetuals = accounts.get(4)?;
        let pool = accounts.get(5)?;
        let position = accounts.get(6)?;
        let position_request = accounts.get(7)?;
        let position_request_ata = accounts.get(8)?;
        let custody = accounts.get(9)?;
        let custody_doves_price_account = accounts.get(10)?;
        let custody_pythnet_price_account = accounts.get(11)?;
        let collateral_custody = accounts.get(12)?;
        let desired_mint = accounts.get(13)?;
        let referral = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let associated_token_program = accounts.get(16)?;
        let system_program = accounts.get(17)?;
        let event_authority = accounts.get(18)?;
        let program = accounts.get(19)?;

        Some(InstantCreateTpslInstructionAccounts {
            keeper: keeper.pubkey,
            api_keeper: api_keeper.pubkey,
            owner: owner.pubkey,
            receiving_account: receiving_account.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            desired_mint: desired_mint.pubkey,
            referral: referral.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}