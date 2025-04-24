use carbon_core::{borsh, CarbonDeserialize};
use serde_big_array::BigArray;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RebalanceRaw {
    #[serde(with = "BigArray")]
    pub params: [u8; 128],
    #[serde(with = "BigArray")]
    pub state: [u8; 256],
    pub reference_price_type: u8,
}
