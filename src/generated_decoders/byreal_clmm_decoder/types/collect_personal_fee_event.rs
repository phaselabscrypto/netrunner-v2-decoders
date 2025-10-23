use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CollectPersonalFeeEvent {
    pub position_nft_mint: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account_0: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account_1: solana_sdk::pubkey::Pubkey,
    pub amount_0: u64,
    pub amount_1: u64,
}
