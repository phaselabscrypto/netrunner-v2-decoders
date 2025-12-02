use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GetRemoveLiquidityAmountAndFeeParams {
    pub lp_amount_in: u64,
}
