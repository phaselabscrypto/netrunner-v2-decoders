use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RemoveCollateralAndSwapParams {
    pub collateral_delta: u64,
    pub min_swap_amount_out: u64,
}
