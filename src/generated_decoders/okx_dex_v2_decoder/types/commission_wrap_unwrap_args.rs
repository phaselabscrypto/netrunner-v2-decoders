use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CommissionWrapUnwrapArgs {
    pub amount_in: u64,
    pub wrap_direction: bool,
    pub commission_rate: u16,
    pub commission_direction: bool,
}
