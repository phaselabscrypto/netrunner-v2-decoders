use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d6a22ced43c076a57")]
pub struct SettleRebatesLogEvent {
    pub pool_name: String,
    pub rebate_usd: u64,
    pub rebate_amount: u64,
    pub padding: [u64; 2],
}
