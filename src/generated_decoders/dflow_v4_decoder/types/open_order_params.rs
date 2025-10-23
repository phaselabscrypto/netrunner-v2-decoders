use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OpenOrderParams {
    pub input_amount: u64,
    pub quoted_out_amount: u64,
    pub fee_budget: u64,
    pub order_account_id: u64,
    pub fillable_for_slots: u32,
    pub slippage_bps: u16,
    pub closer: solana_sdk::pubkey::Pubkey,
    pub flags: u8,
}
