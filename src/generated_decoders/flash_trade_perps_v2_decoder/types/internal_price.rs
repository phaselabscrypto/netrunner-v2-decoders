use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InternalPrice {
    pub price: u64,
    pub conf: u64,
    pub publish_time: i64,
}
