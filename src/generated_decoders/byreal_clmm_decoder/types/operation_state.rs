use carbon_core::{borsh, CarbonDeserialize};
use serde_big_array::BigArray;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OperationState {
    pub bump: u8,
    pub operation_owners: [solana_sdk::pubkey::Pubkey; 10],
    #[serde(with = "BigArray")]
    pub whitelist_mints: [solana_sdk::pubkey::Pubkey; 100],
}
