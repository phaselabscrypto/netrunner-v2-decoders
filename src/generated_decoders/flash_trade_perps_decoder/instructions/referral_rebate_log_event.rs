use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df96ee24e2d786e0f")]
pub struct ReferralRebateLogEvent {
    pub token_stake: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub rebate_usd: u64,
    pub volume_usd: u64,
    pub voltage_points_type: u8,
    pub padding: [u64; 4],
}
