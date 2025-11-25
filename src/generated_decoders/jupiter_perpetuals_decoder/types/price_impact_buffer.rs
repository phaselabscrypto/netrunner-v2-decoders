use carbon_core::{borsh, CarbonDeserialize};
use serde_big_array::BigArray;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PriceImpactBuffer {
    #[serde(with = "BigArray")]
    pub open_interest: [i64; 60],
    pub last_updated: i64,
    pub fee_factor: u64,
    pub exponent: u64,
    pub delta_imbalance_threshold_decimal: u64,
    pub max_fee_bps: u64,
}
