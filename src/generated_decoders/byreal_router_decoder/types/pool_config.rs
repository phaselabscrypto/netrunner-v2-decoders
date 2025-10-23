use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolConfig {
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub fee_receiver: solana_sdk::pubkey::Pubkey,
    pub fee_rate: u32,
    pub padding1: [u8; 28],
}
