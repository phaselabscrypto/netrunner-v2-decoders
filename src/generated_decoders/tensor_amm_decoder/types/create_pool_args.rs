use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreatePoolArgs {
    pub pool_id: [u8; 32],
    pub config: PoolConfig,
    pub currency: Option<solana_sdk::pubkey::Pubkey>,
    pub cosigner: Option<solana_sdk::pubkey::Pubkey>,
    pub maker_broker: Option<solana_sdk::pubkey::Pubkey>,
    pub max_taker_sell_count: Option<u32>,
    pub expire_in_sec: Option<u64>,
}
