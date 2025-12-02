use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CancelTriggerOrderParams {
    pub order_id: u8,
    pub is_stop_loss: bool,
}
