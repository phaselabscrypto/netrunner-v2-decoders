use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x4ef2598aa1ddb04b")]
pub struct ListState {
    pub version: u8,
    pub bump: [u8; 1],
    pub owner: solana_sdk::pubkey::Pubkey,
    pub asset_id: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub currency: Option<solana_sdk::pubkey::Pubkey>,
    pub expiry: i64,
    pub private_taker: Option<solana_sdk::pubkey::Pubkey>,
    pub maker_broker: Option<solana_sdk::pubkey::Pubkey>,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub reserved: [u8; 32],
    pub reserved1: [u8; 64],
}
