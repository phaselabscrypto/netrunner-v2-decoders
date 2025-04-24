use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EditPoolArgs {
    pub new_config: Option<EditPoolConfig>,
    pub cosigner: Option<solana_sdk::pubkey::Pubkey>,
    pub maker_broker: Option<solana_sdk::pubkey::Pubkey>,
    pub expire_in_sec: Option<u64>,
    pub max_taker_sell_count: Option<u32>,
    pub reset_price_offset: bool,
}
