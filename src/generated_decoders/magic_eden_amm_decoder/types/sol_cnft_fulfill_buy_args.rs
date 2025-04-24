use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SolCnftFulfillBuyArgs {
    pub asset_id: solana_sdk::pubkey::Pubkey,
    pub root: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub min_payment_amount: u64,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: i16,
    pub metadata_args: MetadataArgs,
}
