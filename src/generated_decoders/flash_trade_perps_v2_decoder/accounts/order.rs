use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x86addfb94d561c33")]
pub struct Order {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub limit_orders: [LimitOrder; 5],
    pub take_profit_orders: [TriggerOrder; 5],
    pub stop_loss_orders: [TriggerOrder; 5],
    pub is_initialised: bool,
    pub open_orders: u8,
    pub open_sl: u8,
    pub open_tp: u8,
    pub inactive_sl: u8,
    pub inactive_tp: u8,
    pub active_orders: u8,
    pub bump: u8,
    pub reference_timestamp: i64,
    pub execution_count: u64,
    pub padding: [u64; 6],
}
