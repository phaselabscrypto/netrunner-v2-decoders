use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xa9d3ab24dbbd4fbc")]
pub struct TSwap {
    pub version: u8,
    pub bump: [u8; 1],
    pub config: TSwapConfig,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
}
