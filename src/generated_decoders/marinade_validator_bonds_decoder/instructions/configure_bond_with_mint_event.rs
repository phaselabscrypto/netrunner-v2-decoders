
use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1dd1a7c8c663470460")]
pub struct ConfigureBondWithMintEvent{
    pub validator_identity: solana_sdk::pubkey::Pubkey,
    pub bond_authority: Option<PubkeyValueChange>,
    pub cpmpe: Option<U64ValueChange>,
    pub max_stake_wanted: Option<U64ValueChange>,
}
