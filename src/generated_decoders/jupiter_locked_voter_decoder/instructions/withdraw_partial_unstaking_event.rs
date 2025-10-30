use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1328101c6b2d2a53")]
pub struct WithdrawPartialUnstakingEvent {
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
    pub locker: solana_sdk::pubkey::Pubkey,
    pub partial_unstaking: solana_sdk::pubkey::Pubkey,
    pub timestamp: i64,
    pub locker_supply: u64,
    pub released_amount: u64,
}
