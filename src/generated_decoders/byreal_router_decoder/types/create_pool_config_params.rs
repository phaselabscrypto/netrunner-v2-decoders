use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreatePoolConfigParams {
    pub fee_receiver: solana_sdk::pubkey::Pubkey,
    pub fee_rate: u32,
}
