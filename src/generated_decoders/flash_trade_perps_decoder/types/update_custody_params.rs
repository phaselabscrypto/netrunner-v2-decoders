use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateCustodyParams {
    pub min_reserve_usd: u64,
    pub limit_price_buffer_bps: u64,
}
