use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d6052b5cc54b1488d")]
pub struct NewEscrowEvent {
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
    pub locker: solana_sdk::pubkey::Pubkey,
    pub timestamp: i64,
}
