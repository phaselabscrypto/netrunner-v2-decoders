use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitTokenVaultParams {
    pub token_permissions: TokenPermissions,
    pub amount: u64,
    pub withdraw_time_limit: i64,
    pub withdraw_instant_fee: u64,
    pub stake_level: [u64; 6],
}
