
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc738552692f3259e")]
pub struct Bid{
    pub bid_id: solana_sdk::pubkey::Pubkey,
    pub target: Target,
    pub target_id: solana_sdk::pubkey::Pubkey,
    pub field: Option<Field>,
    pub field_id: Option<solana_sdk::pubkey::Pubkey>,
    pub amount: u64,
    pub quantity: u32,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<solana_sdk::pubkey::Pubkey>,
    pub private_taker: Option<solana_sdk::pubkey::Pubkey>,
    pub maker_broker: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct BidInstructionAccounts {
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub bid_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Bid {
    type ArrangedAccounts = BidInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let system_program = accounts.get(0)?;
        let tcomp_program = accounts.get(1)?;
        let bid_state = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let margin_account = accounts.get(4)?;
        let cosigner = accounts.get(5)?;
        let rent_payer = accounts.get(6)?;

        Some(BidInstructionAccounts {
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            margin_account: margin_account.pubkey,
            cosigner: cosigner.pubkey,
            rent_payer: rent_payer.pubkey,
        })
    }
}