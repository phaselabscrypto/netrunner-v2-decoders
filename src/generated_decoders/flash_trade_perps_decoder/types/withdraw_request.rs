use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WithdrawRequest {
    pub pending_deactivation: u64,
    pub withdraw_request_timestamp: i64,
}
