use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MarketPermissions {
    pub allow_open_position: bool,
    pub allow_close_position: bool,
    pub allow_collateral_withdrawal: bool,
    pub allow_size_change: bool,
}
