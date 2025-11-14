use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BorrowRateState {
    pub current_rate: u64,
    pub cumulative_lock_fee: u128,
    pub last_update: i64,
}
