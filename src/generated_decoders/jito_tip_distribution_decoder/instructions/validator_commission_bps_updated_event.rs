use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d04225c19e45833ce")]
pub struct ValidatorCommissionBpsUpdatedEvent {
    pub tip_distribution_account: solana_sdk::pubkey::Pubkey,
    pub old_commission_bps: u16,
    pub new_commission_bps: u16,
}
