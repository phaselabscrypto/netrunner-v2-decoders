use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OracleParams {
    pub int_oracle_account: solana_sdk::pubkey::Pubkey,
    pub ext_oracle_account: solana_sdk::pubkey::Pubkey,
    pub oracle_type: OracleType,
    pub max_divergence_bps: u64,
    pub max_conf_bps: u64,
    pub max_price_age_sec: u32,
    pub max_backup_age_sec: u32,
}
