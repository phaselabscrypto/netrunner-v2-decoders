use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapAmountAndFees {
    pub amount_out: u64,
    pub fee_in: u64,
    pub fee_out: u64,
}
