

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct DataV2LpToken {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
