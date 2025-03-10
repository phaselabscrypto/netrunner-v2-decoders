
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x90c8f9ff6cd9f974")]
pub struct UpdateDecreasePositionRequest2{
    pub params: UpdateDecreasePositionRequest2Params,
}

pub struct UpdateDecreasePositionRequest2InstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_request: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateDecreasePositionRequest2 {
    type ArrangedAccounts = UpdateDecreasePositionRequest2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let perpetuals = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let position = accounts.get(3)?;
        let position_request = accounts.get(4)?;
        let custody = accounts.get(5)?;
        let custody_doves_price_account = accounts.get(6)?;
        let custody_pythnet_price_account = accounts.get(7)?;

        Some(UpdateDecreasePositionRequest2InstructionAccounts {
            owner: owner.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            position_request: position_request.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
        })
    }
}