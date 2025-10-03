use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlatformFeeWrapUnwrapArgsV2 {
    pub amount_in: u64,
    pub commission_info: u32,
    pub platform_fee_rate: u32,
}
