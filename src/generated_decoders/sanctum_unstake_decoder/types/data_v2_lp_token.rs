use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DataV2LpToken {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
