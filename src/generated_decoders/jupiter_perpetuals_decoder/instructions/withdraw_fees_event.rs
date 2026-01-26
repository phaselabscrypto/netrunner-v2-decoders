use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dec768a5a8badb159")]
pub struct WithdrawFeesEvent {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_mint: solana_sdk::pubkey::Pubkey,
    pub receiving_token_account: solana_sdk::pubkey::Pubkey,
    pub total_trade_swap_fees: u64,
    pub pool_trade_swap_fees: u64,
    pub protocol_trade_swap_fees: u64,
    pub total_borrow_lending_fees: u64,
    pub pool_borrow_lending_fees: u64,
    pub protocol_borrow_lending_fees: u64,
    pub pool_total_fees_usd: u64,
    pub apr_bps_before: u64,
    pub apr_bps_after: u64,
    pub apr_bps_updated_at: i64,
    pub pool_realized_fee_usd_before: u64,
    pub pool_realized_fee_usd_after: u64,
    pub curtime: i64,
}
