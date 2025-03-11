

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Token2022Payment {
    pub amount: u64,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub destination_ata: solana_sdk::pubkey::Pubkey,
}
