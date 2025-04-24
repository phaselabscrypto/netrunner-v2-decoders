use carbon_core::{borsh, CarbonDeserialize};
use serde_big_array::BigArray;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CustomizableParams {
    pub trade_fee_numerator: u32,
    pub activation_point: Option<u64>,
    pub has_alpha_vault: bool,
    pub activation_type: u8,
    #[serde(with = "BigArray")]
    pub padding: [u8; 90],
}
