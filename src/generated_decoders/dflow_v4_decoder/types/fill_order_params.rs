use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FillOrderParams {
    pub swap_actions: Vec<Action>,
    pub platform_fee_ubps: u32,
}
