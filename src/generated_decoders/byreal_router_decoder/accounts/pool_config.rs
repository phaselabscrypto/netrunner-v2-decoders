use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x1a6c0e7b74e6812b")]
pub struct PoolConfig {
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub fee_receiver: solana_sdk::pubkey::Pubkey,
    pub fee_rate: u32,
    pub padding1: [u8; 28],
}
