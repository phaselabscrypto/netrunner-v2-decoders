
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2869d9bcdc2d6d6e")]
pub struct ClosePositionRequest{
    pub params: ClosePositionRequestParams,
}

pub struct ClosePositionRequestInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub owner_ata: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position_request: solana_sdk::pubkey::Pubkey,
    pub position_request_ata: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePositionRequest {
    type ArrangedAccounts = ClosePositionRequestInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let owner_ata = accounts.get(2)?;
        let pool = accounts.get(3)?;
        let position_request = accounts.get(4)?;
        let position_request_ata = accounts.get(5)?;
        let position = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let event_authority = accounts.get(8)?;
        let program = accounts.get(9)?;

        Some(ClosePositionRequestInstructionAccounts {
            keeper: keeper.pubkey,
            owner: owner.pubkey,
            owner_ata: owner_ata.pubkey,
            pool: pool.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            position: position.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}