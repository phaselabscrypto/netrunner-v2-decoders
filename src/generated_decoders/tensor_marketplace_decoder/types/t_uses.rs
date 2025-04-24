use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TUses {
    pub use_method: TUseMethod,
    pub remaining: u64,
    pub total: u64,
}
