use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d01d6d96f0703a5f4")]
pub struct SwapFeeInternalLogV3Event {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_amount: u64,
    pub padding: [u64; 4],
}
