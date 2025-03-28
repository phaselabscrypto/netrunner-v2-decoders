

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TokenBurn {
    pub amount: u64,
    pub mint: solana_sdk::pubkey::Pubkey,
}
