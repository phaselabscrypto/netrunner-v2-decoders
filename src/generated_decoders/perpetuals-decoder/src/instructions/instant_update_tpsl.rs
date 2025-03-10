
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x90e47225a5f26f65")]
pub struct InstantUpdateTpsl{
    pub params: InstantUpdateTpslParams,
}

pub struct InstantUpdateTpslInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub api_keeper: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_request: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InstantUpdateTpsl {
    type ArrangedAccounts = InstantUpdateTpslInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let api_keeper = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let perpetuals = accounts.get(3)?;
        let pool = accounts.get(4)?;
        let position = accounts.get(5)?;
        let position_request = accounts.get(6)?;
        let custody = accounts.get(7)?;
        let custody_doves_price_account = accounts.get(8)?;
        let custody_pythnet_price_account = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;

        Some(InstantUpdateTpslInstructionAccounts {
            keeper: keeper.pubkey,
            api_keeper: api_keeper.pubkey,
            owner: owner.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            position_request: position_request.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}