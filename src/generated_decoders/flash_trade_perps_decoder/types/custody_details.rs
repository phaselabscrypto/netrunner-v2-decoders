use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CustodyDetails {
    pub trade_spread_min: u64,
    pub trade_spread_max: u64,
    pub delay_seconds: i64,
    pub min_price: OraclePrice,
    pub max_price: OraclePrice,
}
