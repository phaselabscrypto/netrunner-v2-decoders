use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OracleParams {
    pub oracle_account: solana_sdk::pubkey::Pubkey,
    pub oracle_type: OracleType,
    pub buffer: u64,
    pub max_price_age_sec: u32,
}
