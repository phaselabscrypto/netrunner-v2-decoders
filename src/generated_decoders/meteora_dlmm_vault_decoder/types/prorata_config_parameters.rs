use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ProrataConfigParameters {
    pub max_buying_cap: u64,
    pub start_vesting_duration: u64,
    pub end_vesting_duration: u64,
    pub escrow_fee: u64,
    pub activation_type: u8,
    pub index: u64,
}
