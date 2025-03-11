

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct MplCoreDepositSellArgs {
    pub allowlist_aux: Option<String>,
    pub compression_proof: Option<Vec<u8>>,
}
