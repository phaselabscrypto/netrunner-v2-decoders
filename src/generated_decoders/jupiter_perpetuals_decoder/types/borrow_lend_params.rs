use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BorrowLendParams {
    pub borrows_limit_in_bps: u64,
    pub maintainance_margin_bps: u64,
    pub protocol_fee_bps: u64,
    pub liquidation_margin: u64,
    pub liquidation_fee_bps: u64,
}
