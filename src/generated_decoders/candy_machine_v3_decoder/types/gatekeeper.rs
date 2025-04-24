use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Gatekeeper {
    pub gatekeeper_network: solana_sdk::pubkey::Pubkey,
    pub expire_on_use: bool,
}
