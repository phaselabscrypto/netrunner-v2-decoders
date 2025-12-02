use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d289a04a93dd5a425")]
pub struct VoltagePointsLogEvent {
    pub token_stake: solana_sdk::pubkey::Pubkey,
    pub voltage_points: u64,
    pub rebate_usd: u64,
    pub voltage_points_type: u8,
    pub padding: [u64; 4],
}
