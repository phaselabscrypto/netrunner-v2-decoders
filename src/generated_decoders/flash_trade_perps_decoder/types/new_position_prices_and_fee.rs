use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct NewPositionPricesAndFee {
    pub entry_price: OraclePrice,
    pub fee: u64,
}
