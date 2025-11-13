use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GetAddCollateralQuoteParams {
    pub amount_in: u64,
    pub is_increase_size: bool,
    pub size_delta: u64,
    pub privilege: Privilege,
    pub discount_index: Option<u8>,
}
