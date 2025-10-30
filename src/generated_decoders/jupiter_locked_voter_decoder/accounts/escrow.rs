use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x1fd57bbbba16da9b")]
pub struct Escrow {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub bump: u8,
    pub tokens: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub escrow_started_at: i64,
    pub escrow_ends_at: i64,
    pub vote_delegate: solana_sdk::pubkey::Pubkey,
    pub is_max_lock: bool,
    pub partial_unstaking_amount: u64,
    pub padding: u64,
    pub buffers: [u128; 9],
}
