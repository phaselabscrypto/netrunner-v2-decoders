use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum RebalanceAction {
    NewPriceRange(DexSpecificPrice, DexSpecificPrice),
    NewTickRange(i32, i32),
    WithdrawAndFreeze,
}
