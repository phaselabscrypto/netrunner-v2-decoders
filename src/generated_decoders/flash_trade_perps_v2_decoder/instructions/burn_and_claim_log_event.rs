use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1fcaa3e49c543ec3")]
pub struct BurnAndClaimLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub claim_amount: u64,
    pub current_timestamp: i64,
}
