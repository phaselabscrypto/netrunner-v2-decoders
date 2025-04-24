use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SolMip1FulfillSellArgs {
    pub asset_amount: u64,
    pub max_payment_amount: u64,
    pub allowlist_aux: Option<String>,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: i16,
}
