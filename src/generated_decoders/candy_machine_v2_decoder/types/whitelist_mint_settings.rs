
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WhitelistMintSettings {
    pub mode: WhitelistMintMode,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub presale: bool,
    pub discount_price: Option<u64>,
}
