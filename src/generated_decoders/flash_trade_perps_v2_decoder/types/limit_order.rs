use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LimitOrder {
    pub limit_price: OraclePrice,
    pub reserve_amount: u64,
    pub reserve_custody_uid: u8,
    pub receive_custody_uid: u8,
    pub size_amount: u64,
    pub stop_loss_price: OraclePrice,
    pub take_profit_price: OraclePrice,
}
