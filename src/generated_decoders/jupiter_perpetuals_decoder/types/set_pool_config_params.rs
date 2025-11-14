use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetPoolConfigParams {
    pub fees: Fees,
    pub limit: Limit,
    pub max_request_execution_sec: i64,
    pub parameter_update_oracle: Secp256k1Pubkey,
    pub max_trigger_price_diff_bps: u64,
}
