use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RatioFees {
    pub min_fee: u64,
    pub target_fee: u64,
    pub max_fee: u64,
}
