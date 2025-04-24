use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d79f0267a0066cb7a")]
pub struct ConfigureConfigEvent {
    pub admin_authority: Option<PubkeyValueChange>,
    pub operator_authority: Option<PubkeyValueChange>,
    pub pause_authority: Option<PubkeyValueChange>,
    pub epochs_to_claim_settlement: Option<U64ValueChange>,
    pub minimum_stake_lamports: Option<U64ValueChange>,
    pub withdraw_lockup_epochs: Option<U64ValueChange>,
    pub slots_to_start_settlement_claiming: Option<U64ValueChange>,
    pub min_bond_max_stake_wanted: Option<U64ValueChange>,
}
