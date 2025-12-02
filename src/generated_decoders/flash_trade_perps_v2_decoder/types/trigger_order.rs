use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TriggerOrder {
    pub trigger_price: OraclePrice,
    pub trigger_size: u64,
    pub receive_custody_uid: u8,
}
