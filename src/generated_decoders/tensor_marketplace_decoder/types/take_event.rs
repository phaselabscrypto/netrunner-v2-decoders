use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TakeEvent {
    pub taker: solana_sdk::pubkey::Pubkey,
    pub bid_id: Option<solana_sdk::pubkey::Pubkey>,
    pub target: Target,
    pub target_id: solana_sdk::pubkey::Pubkey,
    pub field: Option<Field>,
    pub field_id: Option<solana_sdk::pubkey::Pubkey>,
    pub amount: u64,
    pub quantity: u32,
    pub tcomp_fee: u64,
    pub taker_broker_fee: u64,
    pub maker_broker_fee: u64,
    pub creator_fee: u64,
    pub currency: Option<solana_sdk::pubkey::Pubkey>,
    pub asset_id: Option<solana_sdk::pubkey::Pubkey>,
}
