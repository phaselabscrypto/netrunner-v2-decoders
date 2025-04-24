use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xe3ba289807ae83b8")]
pub struct FreezeEscrow {
    pub candy_guard: solana_sdk::pubkey::Pubkey,
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub frozen_count: u64,
    pub first_mint_time: Option<i64>,
    pub freeze_period: i64,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}
