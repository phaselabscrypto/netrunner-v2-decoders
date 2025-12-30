use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateWhitelistParams {
    pub is_swap_fee_exempt: bool,
    pub is_deposit_fee_exempt: bool,
    pub is_withdrawal_fee_exempt: bool,
    pub pool_address: solana_sdk::pubkey::Pubkey,
}
