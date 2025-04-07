
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum FeeEnum {
    Flat
                {
                    ratio: Rational,
                }
    ,
    LiquidityLinear
                {
                    params: LiquidityLinearParams,
                }
    ,
}


