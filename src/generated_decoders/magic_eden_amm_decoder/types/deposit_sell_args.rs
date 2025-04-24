use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DepositSellArgs {
    pub asset_amount: u64,
    pub allowlist_aux: Option<String>,
}
