

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x95ea1f671a24a631")]
pub struct SellNftTokenPoolT22{
    pub min_price: u64,
}

pub struct SellNftTokenPoolT22InstructionAccounts {
    pub trade: solana_sdk::pubkey::Pubkey,
    pub t22: solana_sdk::pubkey::Pubkey,
    pub taker_ta: solana_sdk::pubkey::Pubkey,
    pub owner_ta: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellNftTokenPoolT22 {
    type ArrangedAccounts = SellNftTokenPoolT22InstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let trade = accounts.get(0)?;
        let t22 = accounts.get(1)?;
        let taker_ta = accounts.get(2)?;
        let owner_ta = accounts.get(3)?;
        let token_program = accounts.get(4)?;
        let associated_token_program = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(SellNftTokenPoolT22InstructionAccounts {
            trade: trade.pubkey,
            t22: t22.pubkey,
            taker_ta: taker_ta.pubkey,
            owner_ta: owner_ta.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}