

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct InitBondArgs {
    pub bond_authority: solana_sdk::pubkey::Pubkey,
    pub cpmpe: u64,
    pub max_stake_wanted: u64,
}
