use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d64469cf628a9770a")]
pub struct IncreaseLockedAmountEvent {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub locker_supply: u64,
}
