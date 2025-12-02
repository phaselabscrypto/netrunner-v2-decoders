use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dc8973e31e03210de")]
pub struct SwapLogEvent {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub custody_in_uid: u64,
    pub custody_out_uid: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub fee_in_amount: u64,
    pub fee_out_amount: u64,
}
