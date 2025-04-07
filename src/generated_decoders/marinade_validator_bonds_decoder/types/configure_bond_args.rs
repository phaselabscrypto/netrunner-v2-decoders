

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ConfigureBondArgs {
    pub bond_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub cpmpe: Option<u64>,
    pub max_stake_wanted: Option<u64>,
}
