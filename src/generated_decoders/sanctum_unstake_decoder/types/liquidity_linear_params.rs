
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LiquidityLinearParams {
    pub max_liq_remaining: Rational,
    pub zero_liq_remaining: Rational,
}
