

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ProgramGate {
    pub additional: Vec<solana_sdk::pubkey::Pubkey>,
}
