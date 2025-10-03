use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitGlobalConfigEvent {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub trade_fee: u64,
}
