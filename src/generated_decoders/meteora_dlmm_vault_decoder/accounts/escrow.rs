use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x1fd57bbbba16da9b")]
pub struct Escrow {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub total_deposit: u64,
    pub claimed_token: u64,
    pub last_claimed_point: u64,
    pub refunded: u8,
    pub padding1: [u8; 7],
    pub max_cap: u64,
    pub padding2: [u8; 8],
    pub padding: [u128; 1],
}
