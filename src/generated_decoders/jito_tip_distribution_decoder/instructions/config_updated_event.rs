

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1df59e81633c64d6dc")]
pub struct ConfigUpdatedEvent{
    pub authority: solana_sdk::pubkey::Pubkey,
}
