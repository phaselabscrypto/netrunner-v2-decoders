use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ConfigLineSettings {
    pub prefix_name: String,
    pub name_length: u32,
    pub prefix_uri: String,
    pub uri_length: u32,
    pub is_sequential: bool,
}
