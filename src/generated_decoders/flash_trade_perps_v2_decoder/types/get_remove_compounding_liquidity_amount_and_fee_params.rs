use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GetRemoveCompoundingLiquidityAmountAndFeeParams {
    pub compounding_amount_in: u64,
}
