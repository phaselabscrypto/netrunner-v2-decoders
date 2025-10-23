use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdatePoolConfigParams {
    pub fee_receiver: Option<solana_sdk::pubkey::Pubkey>,
    pub fee_rate: Option<u32>,
}
