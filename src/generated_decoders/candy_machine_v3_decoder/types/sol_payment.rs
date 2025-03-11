

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct SolPayment {
    pub lamports: u64,
    pub destination: solana_sdk::pubkey::Pubkey,
}
