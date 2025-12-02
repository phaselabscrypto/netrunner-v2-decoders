use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ExecuteTriggerOrderParams {
    pub is_stop_loss: bool,
    pub order_id: u8,
    pub privilege: Privilege,
}
