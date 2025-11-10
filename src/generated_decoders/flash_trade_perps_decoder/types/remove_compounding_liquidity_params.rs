use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RemoveCompoundingLiquidityParams {
    pub compounding_amount_in: u64,
    pub min_amount_out: u64,
}
