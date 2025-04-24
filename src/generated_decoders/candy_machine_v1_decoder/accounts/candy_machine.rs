use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x33adb17119f16dbd")]
pub struct CandyMachine {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub token_mint: Option<solana_sdk::pubkey::Pubkey>,
    pub config: solana_sdk::pubkey::Pubkey,
    pub data: CandyMachineData,
    pub items_redeemed: u64,
    pub bump: u8,
}
