use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da725ed07591ee8fc")]
pub struct DepositStakeLogEvent {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub lp_tokens: u64,
}
