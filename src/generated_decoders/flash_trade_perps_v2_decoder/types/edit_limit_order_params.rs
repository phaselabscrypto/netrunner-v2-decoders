use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EditLimitOrderParams {
    pub order_id: u8,
    pub limit_price: OraclePrice,
    pub size_amount: u64,
    pub stop_loss_price: OraclePrice,
    pub take_profit_price: OraclePrice,
}
