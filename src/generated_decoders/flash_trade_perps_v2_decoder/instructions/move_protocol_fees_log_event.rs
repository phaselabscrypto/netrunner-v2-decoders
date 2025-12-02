use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1daed5db57bbb00563")]
pub struct MoveProtocolFeesLogEvent {
    pub pool_name: String,
    pub revenue_amount: u64,
    pub protocol_fee: u64,
    pub revenue_fee_share: u64,
    pub padding: [u64; 2],
}
