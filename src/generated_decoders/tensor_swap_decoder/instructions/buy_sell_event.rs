use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d62d0783c5d2013b4")]
pub struct BuySellEvent {
    pub current_price: u64,
    pub tswap_fee: u64,
    pub mm_fee: u64,
    pub creators_fee: u64,
}
