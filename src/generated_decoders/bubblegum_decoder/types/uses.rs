use super::*;

use carbon_core::{borsh, CarbonDeserialize};
use mpl_bubblegum::types::Uses as BubblegumUses;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

impl From<Uses> for BubblegumUses {
    fn from(value: Uses) -> Self {
        Self {
            use_method: value.use_method.into(),
            remaining: value.remaining,
            total: value.total,
        }
    }
}
