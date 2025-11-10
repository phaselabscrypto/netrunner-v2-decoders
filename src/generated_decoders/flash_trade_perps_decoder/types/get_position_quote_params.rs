use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GetPositionQuoteParams {
    pub amount_in: u64,
    pub leverage: u64,
    pub privilege: Privilege,
    pub discount_index: Option<u8>,
}
