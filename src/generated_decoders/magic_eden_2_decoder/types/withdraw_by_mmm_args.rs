use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WithdrawByMMMArgs {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub mmm_pool_uuid: solana_sdk::pubkey::Pubkey,
}
