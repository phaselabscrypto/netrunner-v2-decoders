use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xb7c3c3b48b70ffc1")]
pub struct SellState {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub pool_owner: solana_sdk::pubkey::Pubkey,
    pub asset_mint: solana_sdk::pubkey::Pubkey,
    pub asset_amount: u64,
    pub cosigner_annotation: [u8; 32],
}
