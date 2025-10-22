use carbon_core::{borsh, CarbonDeserialize};
use serde_big_array::BigArray;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RecordIdOptions {
    #[serde(with = "BigArray")]
    pub id: [u8; 76],
}
