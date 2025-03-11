

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d74d449de21f48694")]
pub struct SwapFillEvent{
    pub vault: solana_sdk::pubkey::Pubkey,
    pub pair: solana_sdk::pubkey::Pubkey,
    pub fill_amount: u64,
    pub purchased_amount: u64,
    pub unfilled_amount: u64,
}
