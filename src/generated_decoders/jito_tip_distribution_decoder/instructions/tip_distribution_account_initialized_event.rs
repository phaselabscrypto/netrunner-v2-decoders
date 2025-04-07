

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d27a5e03d288c8bff")]
pub struct TipDistributionAccountInitializedEvent{
    pub tip_distribution_account: solana_sdk::pubkey::Pubkey,
}
