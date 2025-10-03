use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dc3fc85952f7e6be7")]
pub struct InitGlobalConfigEvent {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub trade_fee: u64,
}
