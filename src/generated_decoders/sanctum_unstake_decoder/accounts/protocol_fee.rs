use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x797f628b486e2c76")]
pub struct ProtocolFee {
    pub destination: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub fee_ratio: Rational,
    pub referrer_fee_ratio: Rational,
}
