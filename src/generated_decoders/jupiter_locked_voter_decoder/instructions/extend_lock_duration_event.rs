use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d387934b65485c3bf")]
pub struct ExtendLockDurationEvent {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub locker_supply: u64,
    pub duration: i64,
    pub prev_escrow_ends_at: i64,
    pub next_escrow_ends_at: i64,
    pub next_escrow_started_at: i64,
}
