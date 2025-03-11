

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct SolMplCoreFulfillSellArgs {
    pub max_payment_amount: u64,
    pub buyside_creator_royalty_bp: u16,
    pub allowlist_aux: Option<String>,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: i16,
    pub compression_proof: Option<Vec<u8>>,
}
