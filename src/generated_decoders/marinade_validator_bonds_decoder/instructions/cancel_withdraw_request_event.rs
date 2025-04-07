

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1ddd6168231389f8f6")]
pub struct CancelWithdrawRequestEvent{
    pub withdraw_request: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub requested_amount: u64,
    pub withdrawn_amount: u64,
}
