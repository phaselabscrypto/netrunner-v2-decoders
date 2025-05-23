use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xbfcc95ead5a50d41")]
pub struct Voucher {
    pub leaf_schema: LeafSchema,
    pub index: u32,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
}
