use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SolMplCoreFulfillBuyArgs {
    pub min_payment_amount: u64,
    pub allowlist_aux: Option<String>,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: i16,
    pub compression_proof: Option<Vec<u8>>,
}
