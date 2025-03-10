
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1a88e1d916155314")]
pub struct IncreasePositionPreSwap{
    pub params: IncreasePositionPreSwapParams,
}

pub struct IncreasePositionPreSwapInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub keeper_ata: solana_sdk::pubkey::Pubkey,
    pub position_request: solana_sdk::pubkey::Pubkey,
    pub position_request_ata: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub instruction: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreasePositionPreSwap {
    type ArrangedAccounts = IncreasePositionPreSwapInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let keeper_ata = accounts.get(1)?;
        let position_request = accounts.get(2)?;
        let position_request_ata = accounts.get(3)?;
        let position = accounts.get(4)?;
        let collateral_custody = accounts.get(5)?;
        let collateral_custody_token_account = accounts.get(6)?;
        let instruction = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let event_authority = accounts.get(9)?;
        let program = accounts.get(10)?;

        Some(IncreasePositionPreSwapInstructionAccounts {
            keeper: keeper.pubkey,
            keeper_ata: keeper_ata.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            position: position.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            instruction: instruction.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}