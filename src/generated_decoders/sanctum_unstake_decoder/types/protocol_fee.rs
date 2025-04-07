
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ProtocolFee { 
        pub destination: solana_sdk::pubkey::Pubkey, 
        pub authority: solana_sdk::pubkey::Pubkey, 
        pub fee_ratio: Rational, 
        pub referrer_fee_ratio: Rational, 
}