

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum LeafSchema {
    V1
                {
                    id: solana_sdk::pubkey::Pubkey,
                    owner: solana_sdk::pubkey::Pubkey,
                    delegate: solana_sdk::pubkey::Pubkey,
                    nonce: u64,
                    data_hash: [u8; 32],
                    creator_hash: [u8; 32],
                }
    ,
}


