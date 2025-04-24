use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum FeeEnum {
    Flat { ratio: Rational },
    LiquidityLinear { params: LiquidityLinearParams },
}
