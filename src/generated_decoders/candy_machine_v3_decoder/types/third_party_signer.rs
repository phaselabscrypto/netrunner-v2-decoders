

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ThirdPartySigner {
    pub signer_key: solana_sdk::pubkey::Pubkey,
}
