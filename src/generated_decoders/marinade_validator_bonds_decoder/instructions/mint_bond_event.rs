use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d52bef52123808ec5")]
pub struct MintBondEvent {
    pub bond: solana_sdk::pubkey::Pubkey,
    pub validator_identity: solana_sdk::pubkey::Pubkey,
    pub validator_identity_token_account: solana_sdk::pubkey::Pubkey,
    pub token_metadata: solana_sdk::pubkey::Pubkey,
}
