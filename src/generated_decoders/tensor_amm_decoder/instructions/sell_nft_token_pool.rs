
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x392cc03053086b30")]
pub struct SellNftTokenPool{
    pub min_price: u64,
    pub authorization_data: Option<AuthorizationDataLocal>,
    pub optional_royalty_pct: Option<u16>,
}

pub struct SellNftTokenPoolInstructionAccounts {
    pub trade: solana_sdk::pubkey::Pubkey,
    pub mplx: solana_sdk::pubkey::Pubkey,
    pub taker_ta: solana_sdk::pubkey::Pubkey,
    pub owner_ta: solana_sdk::pubkey::Pubkey,
    pub pool_ta: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellNftTokenPool {
    type ArrangedAccounts = SellNftTokenPoolInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let trade = accounts.get(0)?;
        let mplx = accounts.get(1)?;
        let taker_ta = accounts.get(2)?;
        let owner_ta = accounts.get(3)?;
        let pool_ta = accounts.get(4)?;
        let owner_token_record = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let associated_token_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;

        Some(SellNftTokenPoolInstructionAccounts {
            trade: trade.pubkey,
            mplx: mplx.pubkey,
            taker_ta: taker_ta.pubkey,
            owner_ta: owner_ta.pubkey,
            pool_ta: pool_ta.pubkey,
            owner_token_record: owner_token_record.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}