

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Bumps {
    pub pda: u8,
    pub staker_authority: u8,
    pub settlement_claims: u8,
}
