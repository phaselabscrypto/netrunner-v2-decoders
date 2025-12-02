use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetInternalCurrentPriceParams {
    pub use_current_time: u8,
    pub prices: Vec<InternalPrice>,
}
