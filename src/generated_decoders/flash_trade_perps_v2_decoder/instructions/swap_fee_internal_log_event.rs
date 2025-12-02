use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d90cbfa2f4168091b")]
pub struct SwapFeeInternalLogEvent {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub custody_in_uid: u64,
    pub custody_out_uid: u64,
    pub swap_amount: u64,
    pub reward_custody_amount: u64,
}
