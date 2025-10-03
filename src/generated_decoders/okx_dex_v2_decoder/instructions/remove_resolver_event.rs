use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d398a7d7a64539c25")]
pub struct RemoveResolverEvent {
    pub resolver: solana_sdk::pubkey::Pubkey,
}
