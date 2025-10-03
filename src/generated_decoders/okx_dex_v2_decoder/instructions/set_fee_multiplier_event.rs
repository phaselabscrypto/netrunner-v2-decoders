use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dc55b5aa5f4c90d9a")]
pub struct SetFeeMultiplierEvent {
    pub fee_multiplier: u8,
}
