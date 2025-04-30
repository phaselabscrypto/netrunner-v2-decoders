use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SwapLegDeeper {
    Chain { swap_legs: Vec<SwapLegSwap> },
    Split { split_legs: Vec<SplitLegDeeper> },
    Swap { swap: Swap },
}
