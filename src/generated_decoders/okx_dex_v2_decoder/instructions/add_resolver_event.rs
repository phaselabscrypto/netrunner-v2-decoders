use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dad891dfbc33a7347")]
pub struct AddResolverEvent {
    pub resolver: solana_sdk::pubkey::Pubkey,
}
