use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CompoundingTokenData {
    pub lp_price: u64,
    pub compounding_price: u64,
    pub ratios: Vec<u64>,
}
