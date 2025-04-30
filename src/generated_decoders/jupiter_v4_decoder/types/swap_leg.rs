use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SwapLeg {
    Chain { swap_legs: Vec<SwapLegDeeper> },
    Split { split_legs: Vec<SplitLeg> },
    Swap { swap: Swap },
}
