
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CreatePoolArgs {
    pub spot_price: u64,
    pub curve_type: u8,
    pub curve_delta: u64,
    pub reinvest_fulfill_buy: bool,
    pub reinvest_fulfill_sell: bool,
    pub expiry: i64,
    pub lp_fee_bp: u16,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub cosigner_annotation: [u8; 32],
    pub buyside_creator_royalty_bp: u16,
    pub uuid: solana_sdk::pubkey::Pubkey,
    pub payment_mint: solana_sdk::pubkey::Pubkey,
    pub allowlists: [Allowlist; 6],
}
