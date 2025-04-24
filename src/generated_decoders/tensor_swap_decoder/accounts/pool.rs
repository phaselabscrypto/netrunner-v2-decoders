use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xf19a6d0411b16dbc")]
pub struct Pool {
    pub version: u8,
    pub bump: [u8; 1],
    pub sol_escrow_bump: [u8; 1],
    pub created_unix_seconds: i64,
    pub config: PoolConfig,
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub sol_escrow: solana_sdk::pubkey::Pubkey,
    pub taker_sell_count: u32,
    pub taker_buy_count: u32,
    pub nfts_held: u32,
    pub nft_authority: solana_sdk::pubkey::Pubkey,
    pub stats: PoolStats,
    pub margin: Option<solana_sdk::pubkey::Pubkey>,
    pub is_cosigned: bool,
    pub order_type: u8,
    pub frozen: Option<Frozen>,
    pub last_transacted_seconds: i64,
    pub max_taker_sell_count: u32,
}
