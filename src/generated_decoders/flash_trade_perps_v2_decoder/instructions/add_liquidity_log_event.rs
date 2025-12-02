use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d723b8fadba8b157c")]
pub struct AddLiquidityLogEvent {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub custody_uid: u64,
    pub amount_in: u64,
    pub lp_amount_out: u64,
    pub fee_amount: u64,
}
