use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TradeEvent {
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub total_creator_trading_fees: u64,
    pub total_fee_paid: u64,
    pub price_sol_usd: u64,
    pub base_in: u64,
    pub base_out: u64,
    pub quote_in: u64,
    pub quote_out: u64,
}
