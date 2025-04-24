use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x20823eafe736aa72")]
pub struct SettlementClaims {
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub version: u8,
    pub max_records: u64,
}
