use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d207f6fd402130dd4")]
pub struct RemoveCollateralLogV2Event {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub collateral_amount: u64,
    pub collateral_price: u64,
    pub collateral_price_exponent: i32,
}
