
use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1db777a2f452b672e4")]
pub struct ConfigureBondEvent{
    pub bond_authority: Option<PubkeyValueChange>,
    pub cpmpe: Option<U64ValueChange>,
    pub max_stake_wanted: Option<U64ValueChange>,
}
