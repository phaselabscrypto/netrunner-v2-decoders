

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Allowlist {
    pub kind: u8,
    pub value: solana_sdk::pubkey::Pubkey,
}
