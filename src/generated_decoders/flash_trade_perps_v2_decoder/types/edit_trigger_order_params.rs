use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EditTriggerOrderParams {
    pub order_id: u8,
    pub trigger_price: OraclePrice,
    pub delta_size_amount: u64,
    pub is_stop_loss: bool,
}
